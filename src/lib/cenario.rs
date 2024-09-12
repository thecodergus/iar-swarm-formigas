use super::mapa::Mapa;
use super::formiga::Formiga;
use gtk4::prelude::*;
use gtk4::{DrawingArea, Window};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use rand::Rng;


pub struct Cenario{
    mapa: Mapa,
    formigas: Vec<Formiga>,
    drawing_area: DrawingArea,
    window: Window,
    width: i32,
    height: i32,
}

