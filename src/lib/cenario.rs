use super::formiga::{self, Formiga};
use super::grao::{self, Grao};
use super::outros::Ponto;
use std::sync::{Arc, Mutex};
use std::thread;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Cenario {
    dimensoes: (f64, f64),
    formigas: Vec<Formiga>,
    graos: Arc<Mutex<Vec<Grao>>>,
}

impl Cenario {
    pub fn new(dimensoes: (f64, f64), formigas: Vec<Formiga>, graos: Vec<Grao>) -> Self {
        Cenario {
            dimensoes: dimensoes,
            formigas: formigas,
            graos: Arc::new(Mutex::new(graos)),
        }
    }

    pub fn start(&mut self, numero_interacoes: i64) {
        // Iniciar Variaveis
        let contador: Arc<Mutex<i64>> = Arc::new(Mutex::new(numero_interacoes));

        // Iniciar Spawn de formigas
        for formiga in &mut self.formigas {
            formiga.start(
                self.dimensoes.clone(),
                Arc::clone(&self.graos),
                Arc::clone(&contador),
            );
        }

        loop {
            if let Ok(contador_guard) = contador.lock() {
                if *contador_guard <= 0 {
                    // Gerar uma imagem final
                    println!("Fim do programa");
                    break;
                } else {
                    println!("Loop {}", contador_guard);
                }
            }
        }
    }
}
