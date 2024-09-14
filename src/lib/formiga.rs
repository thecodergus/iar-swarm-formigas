use super::grao::Grao;
use super::outros::Ponto;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Formiga {
    pub posicao: Arc<Mutex<Ponto>>,
    pub segurando_objeto: Arc<Mutex<Option<Grao>>>,
    matar_thread: Arc<Mutex<bool>>,
}

impl Formiga {
    pub fn new(ponto_surgimento: Ponto) -> Formiga {
        Formiga {
            posicao: Arc::new(Mutex::new(ponto_surgimento)),
            segurando_objeto: Arc::new(Mutex::new(None)),
            matar_thread: Arc::new(Mutex::new(false)),
        }
    }

    pub fn novo_movimento(&mut self, tamanho_mapa: (f64, f64)) {
        let mut rng = rand::thread_rng();
        let numero_aleatorio = rng.gen_range(1..=4);
        let posicao = Arc::clone(&self.posicao);

        // 1 - Cima
        // 2 - Direita
        // 3 - Baixo
        // 4 - Esquerda
        match numero_aleatorio {
            1 => {
                if posicao.lock().unwrap().y + 1.0 < tamanho_mapa.1 {
                    posicao.lock().unwrap().y += 1.0;
                }
            }
            2 => {
                if posicao.lock().unwrap().x + 1.0 < tamanho_mapa.0 {
                    posicao.lock().unwrap().x += 1.0;
                }
            }
            3 => {
                if posicao.lock().unwrap().y - 1.0 > 0.0 {
                    posicao.lock().unwrap().y -= 1.0;
                }
            }
            4 => {
                if posicao.lock().unwrap().x - 1.0 > 0.0 {
                    posicao.lock().unwrap().x -= 1.0;
                }
            }
            _ => (),
        }
    }

    pub fn start(mut self, tamanho_mapa: (f64, f64)) {
        thread::spawn(move || loop {
            let mut rng = rand::thread_rng();
            let sleep_duration = Duration::from_millis(rng.gen_range(500..=1500));
            thread::sleep(sleep_duration);

            self.novo_movimento(tamanho_mapa);

            if *self.matar_thread.lock().unwrap() {
                return;
            }
        });
    }

    pub fn stop(mut self) {
        *self.matar_thread.lock().unwrap() = true;
    }
}
