use super::formiga::{self, Formiga};
use super::grao::Grao;
use super::outros::Ponto;
use std::sync::{Arc, Mutex};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

pub struct Cenario {
    dimensoes: (f64, f64),
    formigas: Arc<Mutex<Vec<Formiga>>>,
    graos: Arc<Mutex<Vec<Grao>>>,
    gl: Option<GlGraphics>,
    window: Option<Window>,
}

impl Cenario {
    pub fn new(tamanho: (f64, f64), formigas: Vec<Formiga>, graos: Vec<Grao>) -> Cenario {
        Cenario {
            dimensoes: tamanho,
            formigas: Arc::new(Mutex::new(formigas)),
            graos: Arc::new(Mutex::new(graos)),
            gl: None,
            window: None,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        // Cores
        const PRETO: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const VERDE_LIMAO: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const VERMELHO: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Iniciando desenho
        self.gl
            .as_mut()
            .expect("Sema acesso ao opengl")
            .draw(args.viewport(), |c, gl| {
                clear(PRETO, gl);

                // Desenhando Grãos
                for grao in self.graos.lock().unwrap().iter() {
                    let posicao = &grao.posicao;
                    ellipse(
                        VERDE_LIMAO,
                        [posicao.x as f64, posicao.y as f64, 5.0, 5.0],
                        c.transform,
                        gl,
                    );
                }

                // Desenhando Formigas
                for formiga in self.formigas.lock().unwrap().iter() {
                    let posicao = &formiga.posicao.lock().unwrap();
                    ellipse(
                        VERMELHO,
                        [posicao.x as f64, posicao.y as f64, 5.0, 5.0],
                        c.transform,
                        gl,
                    );
                }
            })
    }

    fn update(&self, args: &UpdateArgs) {}

    // Adicionar formiga representado por um ponto da cor vermelha
    fn adicionar_formiga(&mut self, posicao: Ponto) {
        if let Ok(mut formigas) = self.formigas.lock() {
            formigas.push(Formiga::new(posicao));
        } else {
            eprintln!("Erro ao bloquear mutex para adicionar formiga");
        }
    }

    // Adicionar grão representado por um ponto da cor verde
    fn adicionar_grao(&mut self, posicao: Ponto) {
        if let Ok(mut graos) = self.graos.lock() {
            graos.push(Grao::new(posicao));
        } else {
            eprintln!("Erro ao bloquear mutex para adicionar grão");
        }
    }


    pub fn start(&mut self) {
        // Versão do OpenGL
        let opengl = OpenGL::V3_2;

        // Iniciar movimento das formigas
        for formiga in self.formigas.lock().unwrap().iter_mut() {
            formiga.start(self.dimensoes);
        }

        self.window = Some(
            WindowSettings::new("Formigueiro", [self.dimensoes.0, self.dimensoes.1])
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
        );

        self.gl = Some(GlGraphics::new(opengl));

        let mut eventos = Events::new(EventSettings::new());
        while let Some(e) = eventos.next(self.window.as_mut().expect("Sem acesso a janela")) {
            if let Some(args) = e.render_args() {
                self.render(&args)
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }
}
