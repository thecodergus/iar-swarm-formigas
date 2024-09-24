use super::formiga::{self, Formiga};
use super::grao::{self, Grao};
use super::outros::Ponto;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_filled_circle_mut;
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

        let mut contador_img: i64 = 0;

        loop {
            if let Ok(contador_guard) = contador.lock() {
                if *contador_guard <= 0 {
                    // Gerar uma imagem final
                    println!("Fim do programa");
                    match self.gerar_imagem(
                        "/home/gus/Documentos/iar-swarm-formigas/Cenario-final.png",
                        (800, 640),
                    ) {
                        Ok(_) => println!("Imagem gerada com sucesso!"),
                        Err(e) => eprintln!("Erro ao gerar a imagem: {}", e),
                    }
                    break;
                } else {
                    if (1 / 4) * numero_interacoes <= *contador_guard
                        || (2 / 4) * numero_interacoes <= *contador_guard
                        || (3 / 4) * numero_interacoes <= *contador_guard
                    {
                        println!("Loop {}", contador_guard);
                        match self
                            .gerar_imagem(&format!("Cenario-{}.png", contador_img), (800, 640))
                        {
                            Ok(_) => println!("Imagem gerada com sucesso!"),
                            Err(e) => eprintln!("Erro ao gerar a imagem: {}", e),
                        }
                        contador_img += 1;
                    }
                }
            }
        }
    }

    pub fn gerar_imagem(
        &self,
        path: &str,
        image_dimensions: (u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (img_width, img_height) = image_dimensions;

        // Cria uma imagem com fundo preto
        let mut img = ImageBuffer::from_pixel(img_width, img_height, Rgb([0u8, 0u8, 0u8]));

        // Desenha os grãos
        if let Ok(graos) = self.graos.lock() {
            for grao in graos.iter() {
                let x_px = (grao.posicao.x / self.dimensoes.0) * img_width as f64;
                let y_px = (grao.posicao.y / self.dimensoes.1) * img_height as f64;
                draw_filled_circle_mut(
                    &mut img,
                    (x_px.round() as i32, y_px.round() as i32),
                    3,
                    Rgb([255u8, 255u8, 0u8]), // Amarelo para grãos
                );
            }
        } else {
            return Err("Falha ao adquirir o lock dos grãos.".into());
        }

        // Desenha as formigas
        for formiga in self.formigas.iter() {
            if let Ok(pos) = formiga.posicao.lock() {
                let x_px = (pos.x / self.dimensoes.0) * img_width as f64;
                let y_px = (pos.y / self.dimensoes.1) * img_height as f64;
                draw_filled_circle_mut(
                    &mut img,
                    (x_px.round() as i32, y_px.round() as i32),
                    5,
                    Rgb([255u8, 0u8, 0u8]), // Vermelho para formigas
                );
            } else {
                return Err(format!("Falha ao adquirir o lock da formiga {}", formiga.id).into());
            }
        }

        // Salva a imagem em um arquivo
        match img.save(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Erro ao salvar a imagem: {}", e).into()),
        }
    }
}
