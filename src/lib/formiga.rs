use super::grao::Grao;
use super::outros::Ponto;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, vec};
use std::time::Duration;

pub struct Formiga {
    pub posicao: Arc<Mutex<Ponto>>,
    pub segurando_objeto: Arc<Mutex<Option<Grao>>>,
    matar_thread: Arc<Mutex<bool>>,
}

const VELOCIDADE: i32 = 1;

impl Formiga {
    pub fn new(ponto_surgimento: Ponto) -> Formiga {
        Formiga {
            posicao: Arc::new(Mutex::new(ponto_surgimento)),
            segurando_objeto: Arc::new(Mutex::new(None)),
            matar_thread: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&mut self, tamanho_mapa: (f64, f64)) {
        let posicao = Arc::clone(&self.posicao);
        let segurando_objeto = Arc::clone(&self.segurando_objeto);
        let matar_thread = Arc::clone(&self.matar_thread);

        thread::spawn(move || loop {
            let mut rng = rand::thread_rng();
            let sleep_duration = Duration::from_millis(1);
            thread::sleep(sleep_duration);

            let mut posicao = posicao.lock().unwrap_or_else(|e| {
                eprintln!("Erro ao bloquear mutex: {}", e);
                std::process::exit(1);
            });

            novo_movimento(&mut posicao, tamanho_mapa, &mut rng);

            let matar_thread = matar_thread.lock().unwrap_or_else(|e| {
                eprintln!("Erro ao bloquear mutex: {}", e);
                std::process::exit(1);
            });

            if *matar_thread {
                return;
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