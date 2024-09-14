use super::formiga::Formiga;
use super::grao::Grao;
use super::outros::Ponto;
use std::sync::{Arc, Mutex};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct Cenario {
    dimensoes: (f64, f64),
    formigas: Vec<Formiga>,
    graos: Vec<Arc<Mutex<Grao>>>,
    gl: Option<GlGraphics>,
    window: Option<Window>,
}

impl Cenario {
    pub fn new(tamanho: (f64, f64)) -> Cenario {
        Cenario {
            dimensoes: tamanho,
            formigas: vec![],
            graos: vec![],
            gl: None,
            window: None,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        // Cores
        const PRETO: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const VERDE_LIMAO: [f32; 4] = [191.0, 255.0, 0.0, 1.0];
        const VERMELHO: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Iniciando desenho
        self.gl
            .as_mut()
            .expect("Sema acesso ao opengl")
            .draw(args.viewport(), |c, gl| {
                clear(PRETO, gl);
            })
    }

    fn update(&self, args: &UpdateArgs) {}

    // Adicionar formiga representado por um ponto da cor vermelha
    fn adicionar_formiga(&mut self, posicao: Ponto) {
        self.formigas.push(Formiga::new(posicao));
    }

    // Adicionar grão representado por um ponto da cor verde
    fn adicionar_grao(&mut self, posicao: Ponto) {
        self.graos.push(Arc::new(Mutex::new(Grao::new(posicao))));
    }

    pub fn start(&mut self) {
        // Versão do OpenGL
        let opengl = OpenGL::V3_2;

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
