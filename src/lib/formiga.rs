use super::grao::{self, Grao};
use super::outros::{distancia_euclidiana, Ponto};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, vec};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Formiga {
    pub id: Uuid,
    pub posicao: Arc<Mutex<Ponto>>,
    pub segurando_objeto: Arc<Mutex<Option<Grao>>>,
    matar_thread: Arc<Mutex<bool>>,
}

const VELOCIDADE: f64 = 1.0;

impl Formiga {
    pub fn new(ponto_surgimento: Ponto) -> Formiga {
        Formiga {
            id: Uuid::new_v4(),
            posicao: Arc::new(Mutex::new(ponto_surgimento)),
            segurando_objeto: Arc::new(Mutex::new(None)),
            matar_thread: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(
        &mut self,
        tamanho_mapa: (f64, f64),
        graos: Arc<Mutex<Vec<Grao>>>,
        contador: Arc<Mutex<i64>>,
    ) {
        let posicao: Arc<Mutex<Ponto>> = Arc::clone(&self.posicao);
        let segurando_objeto: Arc<Mutex<Option<Grao>>> = Arc::clone(&self.segurando_objeto);
        let matar_thread: Arc<Mutex<bool>> = Arc::clone(&self.matar_thread);

        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

            loop {
                // Verificação de matar_thread antes de continuar o loop
                if let Ok(matar) = matar_thread.lock() {
                    if *matar {
                        return; // Encerra o loop e a thread
                    } else {
                        if let Ok(mut contador_guard) = contador.lock() {
                            if *contador_guard <= 0 {
                                return;
                            } else {
                                *contador_guard -= 1;
                            }
                        }
                    }
                } else {
                    eprintln!("Erro ao bloquear mutex: matar_thread");
                    std::process::exit(1);
                }

                // Movendo a formiga
                let nova_posicao = nova_posicao(Arc::clone(&posicao), tamanho_mapa);
                if let Ok(mut posicao_guard) = posicao.lock() {
                    *posicao_guard = nova_posicao;
                }

                // Verificando se há grãos por perto
                let graos_por_perto =
                    encontrar_graos_vizinhanca(Arc::clone(&posicao), Arc::clone(&graos));

                segurar_objeto(
                    Arc::clone(&posicao),
                    Arc::clone(&segurando_objeto),
                    graos_por_perto,
                    Arc::clone(&graos),
                );
            }
        });
    }

    pub fn stop(&mut self) {
        if let Ok(mut matar_guard) = self.matar_thread.lock() {
            *matar_guard = true;
        } else {
            eprintln!("Erro ao tentar bloquear o mutex: matar_thread");
        }
    }
}

fn nova_posicao(posicao: Arc<Mutex<Ponto>>, tamanho_mapa: (f64, f64)) -> Ponto {
    let mut rng = rand::thread_rng();
    let num_aleatorio: i32 = rng.gen_range(1..=4);

    if let Ok(posicao_guard) = posicao.lock() {
        let mut nova_posicao = match num_aleatorio {
            1 => Ponto {
                x: posicao_guard.x,
                y: posicao_guard.y + (1.0 * VELOCIDADE),
            },
            2 => Ponto {
                x: posicao_guard.x + (1.0 * VELOCIDADE),
                y: posicao_guard.y,
            },
            3 => Ponto {
                x: posicao_guard.x,
                y: posicao_guard.y - (1.0 * VELOCIDADE),
            },
            4 => Ponto {
                x: posicao_guard.x - (1.0 * VELOCIDADE),
                y: posicao_guard.y,
            },
            _ => Ponto {
                x: posicao_guard.x,
                y: posicao_guard.y, // Mantém a posição atual se o número aleatório não for esperado
            },
        };

        // Verificação dos limites do mapa
        if nova_posicao.x < 0.0 {
            nova_posicao.x = 0.0;
        } else if nova_posicao.x > tamanho_mapa.0 {
            nova_posicao.x = tamanho_mapa.0;
        }

        if nova_posicao.y < 0.0 {
            nova_posicao.y = 0.0;
        } else if nova_posicao.y > tamanho_mapa.1 {
            nova_posicao.y = tamanho_mapa.1;
        }

        nova_posicao
    } else {
        eprintln!("Erro ao bloquear mutex: segurando_objeto");
        std::process::exit(1);
    }
}

pub fn gerar_formigas(numero: i32, tamanho_mapa: (f64, f64)) -> Vec<Formiga> {
    // Criar 10 formigas aleatórias
    let mut rng = rand::thread_rng();
    let mut formigas: Vec<Formiga> = vec![];

    for _ in 0..numero {
        let x = rng.gen_range(0.0..=tamanho_mapa.0);
        let y = rng.gen_range(0.0..=tamanho_mapa.1);
        formigas.push(Formiga::new(Ponto { x, y }));
    }

    formigas
}

fn encontrar_graos_vizinhanca(
    posicao: Arc<Mutex<Ponto>>,
    graos: Arc<Mutex<Vec<Grao>>>,
) -> Vec<Grao> {
    let mut resultado: Vec<Grao> = vec![];

    // Definindo o tamanho da vizinhança (a distância máxima em cada direção)
    let tamanho_vizinhanca = 1.0;

    // Trava o mutex para acessar a posição da formiga
    if let Ok(posicao_guard) = posicao.lock() {
        // Trava o mutex para acessar a lista de grãos
        if let Ok(graos_guard) = graos.lock() {
            for grao in graos_guard.iter() {
                let distancia_x = (grao.posicao.x - posicao_guard.x).abs();
                let distancia_y = (grao.posicao.y - posicao_guard.y).abs();

                // Se o grão está dentro da vizinhança 3x3 (distância <= 1.0 em x e y)
                if distancia_x <= tamanho_vizinhanca && distancia_y <= tamanho_vizinhanca {
                    resultado.push(grao.clone()); // Adiciona o grão à lista de resultado
                }
            }
        } else {
            eprintln!("Erro ao tentar adquirir o lock do Mutex de graos");
        }
    } else {
        eprintln!("Erro ao tentar adquirir o lock do Mutex de posicao_formiga");
    }

    resultado
}

fn segurar_objeto(
    posicao_formiga: Arc<Mutex<Ponto>>,
    segurando_objeto: Arc<Mutex<Option<Grao>>>,
    graos_perto: Vec<Grao>,
    graos: Arc<Mutex<Vec<Grao>>>,
) {
    let num_celulas_ao_redor = 8;
    let num_itens_ao_redor = graos_perto.len();
    let mut rng = rand::thread_rng();
    let valor_aletorio: f64 = rng.gen_range(0.0..=1.0);

    // Manipulação de segurando_objeto
    if let Ok(mut objeto_guard) = segurando_objeto.lock() {
        if let Ok(p) = posicao_formiga.lock() {
            if objeto_guard.is_some() {
                // Largar item
                if valor_aletorio <= pode_largar(*p, graos_perto.clone()) {
                    if let Ok(mut graos) = graos.lock() {
                        graos.push(Grao::new(*p));
                    }
                }
            } else {
                // Pegar item
                if valor_aletorio <= pode_pegar(*p, graos_perto.clone()) {
                    if let Ok(mut segurando_objeto_guard) = segurando_objeto.lock() {
                        match graos_perto.first() {
                            Some(o) => {
                                *segurando_objeto_guard = Some(*o);
                                if let Ok(mut graos) = graos.lock() {
                                    graos.retain(|g| g.id != o.id);
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Erro ao bloquear mutex: segurando_objeto");
        std::process::exit(1);
    }
}

fn similaridade_entre_dado_vizinhanca(p: Ponto, itens: Vec<Grao>) -> f64 {
    // Hiperparametro
    const ALPHA: f64 = 11.8;

    if itens.len() != 0 {
        (1.0 / (itens.len() as f64).powi(2))
            * itens
                .into_iter()
                .map(|g| (1.0 - distancia_euclidiana(&p, &g.posicao)) / ALPHA)
                .sum::<f64>()
    } else {
        0.0
    }
}

fn pode_pegar(p: Ponto, itens_proximos: Vec<Grao>) -> f64 {
    const K1: f64 = 0.3;
    let fi: f64 = similaridade_entre_dado_vizinhanca(p, itens_proximos);
    (K1 / (K1 + fi)).powi(2)
}

fn pode_largar(p: Ponto, itens_proximos: Vec<Grao>) -> f64 {
    const K2: f64 = 0.6;
    let fi: f64 = similaridade_entre_dado_vizinhanca(p, itens_proximos);
    (fi / (K2 + fi)).powi(2)
}
