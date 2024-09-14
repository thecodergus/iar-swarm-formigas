use super::outros::Ponto;

pub struct Grao {
    pub posicao: Ponto,
}

impl Grao {
    pub fn new(posicao: Ponto) -> Grao {
        Grao { posicao }
    }
}
