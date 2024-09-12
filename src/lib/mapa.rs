use super::grao::Grao;
use std::sync::{Arc, Mutex};

pub struct Mapa{
    pub tamanho: (f64, f64),
    pub graos: Arc<Mutex<Vec<Grao>>>
}

impl Mapa{
    pub fn new(tamanho: (f64, f64)) -> Mapa{
        Mapa { tamanho, graos: Arc::new(Mutex::new(vec![])) }
    }
}