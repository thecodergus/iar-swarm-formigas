use std::vec;

use super::outros::Ponto;
use rand::Rng;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct Grao {
    pub id: Uuid,
    pub posicao: Ponto,
}

impl Grao {
    pub fn new(posicao: Ponto) -> Grao {
        Grao { 
            id: Uuid::new_v4(), 
            posicao 
        }
    }
}

pub fn gerar_graos(numero: i32, tamanho_mapa: (f64, f64)) -> Vec<Grao>{
    let mut graos: Vec<Grao> = vec![];

    let mut rng = rand::thread_rng();
        
    for _ in 0..numero {
        let x = rng.gen_range(0.0..=tamanho_mapa.0);
        let y = rng.gen_range(0.0..=tamanho_mapa.1);
        graos.push(Grao::new(Ponto { x, y }));
    }

    graos
}