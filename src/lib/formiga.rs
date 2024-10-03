use super::grao::{self, Grao};
use super::outros::Ponto;
use rand::{thread_rng, Rng};
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

// Parametros
const TAMANHO_VIZINHANCA: i32 = 1; // Definindo o tamanho da vizinhança (a distância máxima em cada direção)
const ALPHA: f64 = 0.5;
const K1: f64 = 0.05;
const K2: f64 = 0.0125;

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

        let _: thread::JoinHandle<()> = thread::spawn(move || {
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

                // Ações relacionadas a ter ou não itens na mão
                acao_segurar_objeto(
                    Arc::clone(&posicao),
                    Arc::clone(&segurando_objeto),
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
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    // Gera um número aleatório de 1 a 8 para escolher a direção
    let num_aleatorio: i32 = rng.gen_range(1..=8);

    if let Ok(posicao_guard) = posicao.lock() {
        let mut nova_posicao = match num_aleatorio {
            1 => Ponto {
                // Cima
                x: posicao_guard.x,
                y: posicao_guard.y + 1 as i32,
            },
            2 => Ponto {
                // Direita
                x: posicao_guard.x + 1 as i32,
                y: posicao_guard.y,
            },
            3 => Ponto {
                // Baixo
                x: posicao_guard.x,
                y: posicao_guard.y - 1 as i32,
            },
            4 => Ponto {
                // Esquerda
                x: posicao_guard.x - 1 as i32,
                y: posicao_guard.y,
            },
            5 => Ponto {
                // Diagonal superior direita
                x: posicao_guard.x + 1 as i32,
                y: posicao_guard.y + 1 as i32,
            },
            6 => Ponto {
                // Diagonal superior esquerda
                x: posicao_guard.x - 1 as i32,
                y: posicao_guard.y + 1 as i32,
            },
            7 => Ponto {
                // Diagonal inferior direita
                x: posicao_guard.x + 1 as i32,
                y: posicao_guard.y - 1 as i32,
            },
            8 => Ponto {
                // Diagonal inferior esquerda
                x: posicao_guard.x - 1 as i32,
                y: posicao_guard.y - 1 as i32,
            },
            _ => Ponto {
                // Mantém a posição atual, caso um número inesperado seja gerado
                x: posicao_guard.x,
                y: posicao_guard.y,
            },
        };

        // Verificação dos limites do mapa com teletransporte
        // Se ultrapassar o limite à esquerda, vai para o limite direito
        if nova_posicao.x < 0 {
            nova_posicao.x = tamanho_mapa.0 as i32;
        // Se ultrapassar o limite direito, vai para o limite esquerdo
        } else if nova_posicao.x > tamanho_mapa.0 as i32 {
            nova_posicao.x = 0;
        }

        // Se ultrapassar o limite superior, vai para o limite inferior
        if nova_posicao.y < 0 {
            nova_posicao.y = tamanho_mapa.1 as i32;
        // Se ultrapassar o limite inferior, vai para o limite superior
        } else if nova_posicao.y > tamanho_mapa.1 as i32 {
            nova_posicao.y = 0;
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
        let x: i32 = rng.gen_range(0..=tamanho_mapa.0 as i32);
        let y: i32 = rng.gen_range(0..=tamanho_mapa.1 as i32);
        formigas.push(Formiga::new(Ponto { x: x, y: y }));
    }

    formigas
}

fn acao_segurar_objeto(
    posicao_formiga: Arc<Mutex<Ponto>>,
    objeto: Arc<Mutex<Option<Grao>>>,
    graos: Arc<Mutex<Vec<Grao>>>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let numero_aleatorio: f64 = rng.gen_range(0.0..=1.0);

    // Locks de Mutex
    let posicao_formiga_guard = posicao_formiga
        .lock()
        .expect("Não foi possivel dar lock em formiga");
    let mut objeto_guard = objeto.lock().expect("Não foi possivel dar lock em objeto");
    let mut graos_guard = graos.lock().expect("Não foi possivel dar lock em graos");

    // Procurar pelos grãos ao redor
    let graos_perto = procurar_graos_redor(&*posicao_formiga_guard, &*graos_guard);

    // Operações
    if let Some(ref mut mao) = &mut *objeto_guard {
        // Tem algo na mão que pode largar
        if procurar_grao_local(&posicao_formiga_guard, &graos_guard).is_none() {
            if numero_aleatorio <= pd(&mao, &graos_perto) {
                mao.posicao = *posicao_formiga_guard;
                graos_guard.push(mao.clone());
                *objeto_guard = None;
            }
        }
    } else if let Some(grao) = procurar_grao_local(&posicao_formiga_guard, &graos_guard) {
        // Não tem nada na mão mas tem algo na localização que pode pegar

        if numero_aleatorio <= pp(&grao, &graos_perto) {
            objeto_guard.replace(grao.clone());

            // Removendo a lista de grãos
            graos_guard.retain(|g| g.id != grao.id);
        }
    }
}

fn procurar_grao_local(local: &Ponto, graos: &Vec<Grao>) -> Option<Grao> {
    for g in graos.iter() {
        if g.posicao == *local {
            return Some(g.clone());
        }
    }

    return None;
}

fn procurar_graos_redor(local: &Ponto, graos: &Vec<Grao>) -> Vec<Grao> {
    let mut resultado: Vec<Grao> = vec![];

    for g in graos {
        let dist_x: i32 = (local.x - g.posicao.x).abs();
        let dist_y: i32 = (local.y - g.posicao.y).abs();

        if dist_x <= TAMANHO_VIZINHANCA && dist_y <= TAMANHO_VIZINHANCA {
            resultado.push(g.clone());
        }
    }

    return resultado;
}

fn distancia_entre_par_de_dados(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter()
        .zip(b)
        .map(|(i, j)| (*i - *j).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn similaridade(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    let quantidade_dados: usize = graos_perto.len();

    if quantidade_dados > 0 {
        return graos_perto
            .iter()
            .map(|g| (1.0 - distancia_entre_par_de_dados(&grao.dados, &g.dados)) / ALPHA)
            .sum::<f64>()
            / 2.0
            * TAMANHO_VIZINHANCA as f64
            + 1.0;
    }

    return 0.0;
}

fn pp(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    (K1 / (K1 + similaridade(grao, graos_perto))).powi(2)
}

fn pd(grao: &Grao, graos_perto: &Vec<Grao>) -> f64 {
    let resultado: f64 = similaridade(grao, graos_perto);

    if resultado < K2 {
        return 2.0 * resultado;
    } else if resultado >= K2 {
        return 1.0;
    }

    return 0.0;
}
