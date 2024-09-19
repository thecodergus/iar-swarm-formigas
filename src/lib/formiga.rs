use super::grao::{self, Grao};
use super::outros::Ponto;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, vec};
use std::time::Duration;
use uuid::Uuid;

pub struct Formiga {
    pub id: Uuid,
    pub posicao: Arc<Mutex<Ponto>>,
    pub segurando_objeto: Arc<Mutex<Option<Grao>>>,
    matar_thread: Arc<Mutex<bool>>,
}

const VELOCIDADE: i32 = 1;

impl Formiga {
    pub fn new(ponto_surgimento: Ponto) -> Formiga {
        Formiga {
            id: Uuid::new_v4(),
            posicao: Arc::new(Mutex::new(ponto_surgimento)),
            segurando_objeto: Arc::new(Mutex::new(None)),
            matar_thread: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self, tamanho_mapa: (f64, f64), graos: Arc<Mutex<Vec<Grao>>>) {
        let posicao = Arc::clone(&self.posicao);
        let segurando_objeto = Arc::clone(&self.segurando_objeto);
        let matar_thread = Arc::clone(&self.matar_thread);

        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                // Verificação de matar_thread antes de continuar o loop
                if let Ok(matar) = matar_thread.lock() {
                    if *matar {
                        return; // Encerra o loop e a thread
                    }
                } else {
                    eprintln!("Erro ao bloquear mutex: matar_thread");
                    std::process::exit(1);
                }

                // Simulando o movimento
                // thread::sleep(Duration::from_millis(1));

                // Movendo a formiga
                if let Ok(mut posicao) = posicao.lock() {
                    novo_movimento(&mut posicao, tamanho_mapa, &mut rng);
                } else {
                    eprintln!("Erro ao bloquear mutex: posicao");
                    std::process::exit(1);
                }

                // Verificando se há grãos por perto
                let graos_por_perto;
                if let Ok(graos_guard) = graos.lock() {
                    graos_por_perto = procurar_graos_por_perto(&posicao.lock().unwrap(), &graos_guard);
                } else {
                    eprintln!("Erro ao bloquear mutex: graos");
                    std::process::exit(1);
                }

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

fn novo_movimento(posicao: &mut Ponto, tamanho_mapa: (f64, f64), rng: &mut rand::prelude::ThreadRng) {
    let numero_aleatorio = rng.gen_range(1..=4);

    match numero_aleatorio {
        1 => {
            if posicao.y + (1 * VELOCIDADE) < tamanho_mapa.1 as i32 {
                posicao.y += (1 * VELOCIDADE);
            }
        }
        2 => {
            if posicao.x + (1 * VELOCIDADE) < tamanho_mapa.0 as i32 {
                posicao.x += (1 * VELOCIDADE);
            }
        }
        3 => {
            if posicao.y - (1 * VELOCIDADE) > 0 {
                posicao.y -= (1 * VELOCIDADE);
            }
        }
        4 => {
            if posicao.x - (1 * VELOCIDADE) > 0 {
                posicao.x -= (1 * VELOCIDADE);
            }
        }
        _ => (),
    }
}

pub fn gerar_formigas(numero: i32, tamanho_mapa: (i32, i32)) -> Vec<Formiga>{
    // Criar 10 formigas aleatórias
    let mut rng = rand::thread_rng();
    let mut formigas: Vec<Formiga> = vec![];

    for _ in 0..numero {
        let x = rng.gen_range(0..=tamanho_mapa.0);
        let y = rng.gen_range(0..=tamanho_mapa.1);
        formigas.push(Formiga::new(Ponto { x, y }));
    }

    formigas
}

fn procurar_graos_por_perto(posicao_formiga: &Ponto, graos: &Vec<Grao>) -> Vec<Grao>{
    let mut resultado: Vec<Grao> = vec![];

    for g in graos{
        if (g.posicao.x - posicao_formiga.x).abs() <= 1 && (g.posicao.y - posicao_formiga.y).abs() <= 1{
            resultado.push(*g);
        }
    }

    return resultado;
}