#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ponto{
    pub x: f64,
    pub y: f64
}

pub fn distancia_euclidiana(p1: &Ponto, p2: &Ponto) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}
