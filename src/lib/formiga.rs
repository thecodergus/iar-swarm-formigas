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
const ALPHA: f64 = 12.0;
const K1: f64 = 0.5;
const K2: f64 = 0.5;

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
    let posicao_formiga_guard = posicao_formiga
        .lock()
        .expect("Não foi possivel dar lock em formiga");
    let objeto_guard = objeto.lock().expect("Não foi possivel dar lock em objeto");
    let graos_guard = graos.lock().expect("Não foi possivel dar lock em graos");

    if let Some(mao) = &*objeto_guard {
    } else if let Some(grao) = procurar_grao_local(&posicao_formiga_guard, &graos_guard) {
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

fn procurar_graos_redor(local: &Ponto, graos: &Vec<Grao>, mao: &Option<Grao>) -> Vec<Grao> {
    let mut resultado: Vec<Grao> = vec![];

    if let Some(g) = mao {
        resultado.push(g.clone());
    }

    for g in graos {
        let dist_x: i32 = (local.x - g.posicao.x).abs();
        let dist_y: i32 = (local.y - g.posicao.y).abs();

        if dist_x <= TAMANHO_VIZINHANCA && dist_y <= TAMANHO_VIZINHANCA {
            resultado.push(g.clone());
        }
    }

    return resultado;
}
