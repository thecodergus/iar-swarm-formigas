use std::vec;

use super::outros::Ponto;
use rand::Rng;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Grao {
    pub id: Uuid,
    pub posicao: Ponto,
    pub dados: Vec<f64>,
}

impl Grao {
    pub fn new(posicao: Ponto, dados: Vec<f64>) -> Grao {
        Grao {
            id: Uuid::new_v4(),
            posicao,
            dados,
        }
    }
}

pub fn gerar_graos(numero: i32, tamanho_mapa: (f64, f64), dados: Vec<f64>) -> Vec<Grao> {
    let mut graos: Vec<Grao> = vec![];

    let mut rng = rand::thread_rng();

    for _ in 0..numero {
        let x: i32 = rng.gen_range(0..=(tamanho_mapa.0 as i32));
        let y: i32 = rng.gen_range(0..=tamanho_mapa.1 as i32);
        graos.push(Grao::new(Ponto { x: x, y: y }, dados.clone()));
    }

    graos
}
