use super::grao::{self, Grao};
use super::outros::Ponto;
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

    pub fn start(&mut self, tamanho_mapa: (f64, f64), graos: Arc<Mutex<Vec<Grao>>>, contador: Arc<Mutex<i64>>) {
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
                    }else{
                        if let Ok(mut contador_guard) = contador.lock(){
                            if *contador_guard <= 0 {
                                return;
                            }else{
                                *contador_guard -= 1;
                            }
                        }
                    }
                } else {
                    eprintln!("Erro ao bloquear mutex: matar_thread");
                    std::process::exit(1);
                }

                // Movendo a formiga
                if let Ok(mut posicao_guard) = posicao.lock(){
                    let nova_posicao = nova_posicao(Arc::clone(&posicao), tamanho_mapa);
                    *posicao_guard = nova_posicao;
                }

                // Verificando se há grãos por perto
                let graos_por_perto = procurar_graos_por_perto(Arc::clone(&posicao), Arc::clone(&graos));

                let num_celulas_ao_redor = 8;
                let num_itens_ao_redor = graos_por_perto.len();
                let valor_aletorio: f64 = rng.gen_range(0.0..=1.0);

                // Manipulação de segurando_objeto
                if let Ok(mut objeto_guard) = segurando_objeto.lock() {
                    if objeto_guard.is_some() {
                        // Largar o objeto
                        let pode_largar = num_itens_ao_redor as f64 / num_celulas_ao_redor as f64;
                        if valor_aletorio <= pode_largar {
                            *objeto_guard = None; // Retira o objeto da mão
                            
                            // Adiciona o grão ao vetor
                            if let Ok(mut graos_guard) = graos.lock() {
                                graos_guard.push(Grao::new(*posicao.lock().unwrap()));
                            }

                        }
                    } else {
                        // Pegar um objeto
                        let pode_pegar = 1.0 - (num_itens_ao_redor as f64 / num_celulas_ao_redor as f64);
                        if valor_aletorio <= pode_pegar {
                            if let Some(item_selecionado) = graos_por_perto.first().cloned() {
                                *objeto_guard = Some(item_selecionado.clone()); // Pega o item

                                // Remove o item da lista de grãos
                                if let Ok(mut graos_guard) = graos.lock() {
                                    graos_guard.retain(|g| g.id != item_selecionado.id);
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("Erro ao bloquear mutex: segurando_objeto");
                    std::process::exit(1);
                }
            }
        });
    }


    pub fn stop(mut self) {
        *self.matar_thread.lock().unwrap() = true;
    }
}


fn nova_posicao(posicao: Arc<Mutex<Ponto>>, tamanho_mapa: (f64, f64)) -> Ponto{
    let mut rng = rand::thread_rng();
    let num_aleatorio: i32 = rng.gen_range(1..=4);

    if let Ok(posicao_guard) = posicao.lock(){
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
    }else{
        eprintln!("Erro ao bloquear mutex: segurando_objeto");
        std::process::exit(1);
    }
}

pub fn gerar_formigas(numero: i32, tamanho_mapa: (f64, f64)) -> Vec<Formiga>{
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

fn procurar_graos_por_perto(
    posicao_formiga: Arc<Mutex<Ponto>>,
    graos: Arc<Mutex<Vec<Grao>>>
) -> Vec<Grao> {
    let mut resultado: Vec<Grao> = vec![];

    // Tenta adquirir o lock no Mutex da posição da formiga
    if let Ok(posicao_formiga_guard) = posicao_formiga.lock() {
        // Tenta adquirir o lock no Mutex da lista de grãos
        if let Ok(graos_guard) = graos.lock() {
            // Itera sobre os grãos e realiza a comparação
            for g in graos_guard.iter() {
                if (g.posicao.x - posicao_formiga_guard.x).abs() <= 1.0
                    && (g.posicao.y - posicao_formiga_guard.y).abs() <= 1.0
                {
                    resultado.push(g.clone()); // Clone para adicionar o grão à lista de resultado
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
