use std::vec;

use super::outros::Ponto;
use rand::Rng;

pub struct Grao {
    pub posicao: Ponto,
}

impl Grao {
    pub fn new(posicao: Ponto) -> Grao {
        Grao { posicao }
    }
}

pub fn gerar_graos(numero: i32, tamanho_mapa: (i32, i32)) -> Vec<Grao>{
    let mut graos: Vec<Grao> = vec![];

    let mut rng = rand::thread_rng();
        
    for _ in 0..numero {
        let x = rng.gen_range(0..=tamanho_mapa.0);
        let y = rng.gen_range(0..=tamanho_mapa.1);
        graos.push(Grao::new(Ponto { x, y }));
    }

    graos
}