use super::grao::Grao;
use std::sync::{Arc, Mutex};

pub struct Mapa{
    tamanho: (i32, i32),
    graos: Arc<Mutex<Vec<Grao>>>
}