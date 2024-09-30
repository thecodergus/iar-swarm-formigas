use super::formiga::{self, Formiga};
use super::grao::{self, Grao};
use super::outros::Ponto;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_filled_circle_mut;
use imageproc::rect::Rect;
use rand::Rng; // Necessário para desenhar retângulos

#[derive(Clone, Debug)]
pub struct Cenario {
    dimensoes: (f64, f64),
    formigas: Vec<Formiga>,
    graos: Arc<Mutex<Vec<Grao>>>,
    cores_por_grupo: HashMap<String, Rgb<u8>>,
}

impl Cenario {
    /// Inicializa um novo cenário e cria o mapa de cores para os grupos de grãos
    pub fn new(dimensoes: (f64, f64), formigas: Vec<Formiga>, graos: Vec<Grao>) -> Self {
        let mut cores_por_grupo = HashMap::new();

        // Inicializa o mapa de cores para os grupos de grãos
        for grao in &graos {
            cores_por_grupo
                .entry(format!("{:?}", grao.dados))
                .or_insert_with(|| gerar_cor_aleatoria());
        }

        Cenario {
            dimensoes,
            formigas,
            graos: Arc::new(Mutex::new(graos)),
            cores_por_grupo, // Armazena o mapa de cores
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
        // Variáveis para rastrear se as imagens já foram geradas
        let mut gerou_75_porcento = false;
        let mut gerou_50_porcento = false;
        let mut gerou_25_porcento = false;
        let mut gerou_0_porcento = false;

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
                    let percentual_restante = (*contador_guard as f64) / (numero_interacoes as f64);

                    if !gerou_75_porcento && percentual_restante <= 0.75 {
                        println!("Loop {} - 25% concluído", contador_guard);
                        match self
                            .gerar_imagem(&format!("Cenario-{}.png", contador_img), (800, 640))
                        {
                            Ok(_) => println!("Imagem gerada com sucesso!"),
                            Err(e) => eprintln!("Erro ao gerar a imagem: {}", e),
                        }
                        contador_img += 1;
                        gerou_75_porcento = true;
                    } else if !gerou_50_porcento && percentual_restante <= 0.50 {
                        println!("Loop {} - 50% concluído", contador_guard);
                        match self
                            .gerar_imagem(&format!("Cenario-{}.png", contador_img), (800, 640))
                        {
                            Ok(_) => println!("Imagem gerada com sucesso!"),
                            Err(e) => eprintln!("Erro ao gerar a imagem: {}", e),
                        }
                        contador_img += 1;
                        gerou_50_porcento = true;
                    } else if !gerou_25_porcento && percentual_restante <= 0.25 {
                        println!("Loop {} - 75% concluído", contador_guard);
                        match self
                            .gerar_imagem(&format!("Cenario-{}.png", contador_img), (800, 640))
                        {
                            Ok(_) => println!("Imagem gerada com sucesso!"),
                            Err(e) => eprintln!("Erro ao gerar a imagem: {}", e),
                        }
                        contador_img += 1;
                        gerou_25_porcento = true;
                    } else if !gerou_0_porcento && percentual_restante <= 1.0 {
                        println!("Loop {} - 0% concluído", contador_guard);
                        match self
                            .gerar_imagem(&format!("Cenario-{}.png", contador_img), (800, 640))
                        {
                            Ok(_) => println!("Imagem gerada com sucesso!"),
                            Err(e) => eprintln!("Erro ao gerar a imagem: {}", e),
                        }
                        contador_img += 1;
                        gerou_0_porcento = true;
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

        // Definir a dimensão lógica (a "base" de 100x100)
        let base_dimensions = (100.0, 100.0);

        // Cria uma imagem com fundo preto
        let mut img = ImageBuffer::from_pixel(img_width, img_height, Rgb([0u8, 0u8, 0u8]));

        // Escalar o tamanho da imagem real em relação à base de 100x100
        let scale_x = img_width as f64 / base_dimensions.0;
        let scale_y = img_height as f64 / base_dimensions.1;

        // Tamanho do lado dos quadrados (grãos e formigas) em pixels
        let tamanho_grao = (1.0 * scale_x / 1.0).round() as i32;
        let tamanho_formiga = (1.0 * scale_x / 1.0).round() as i32;

        // Desenha os grãos
        if let Ok(graos) = self.graos.lock() {
            println!("Numero de grãos: {}", graos.len());

            for grao in graos.iter() {
                // Verifica se já existe uma cor associada ao grupo de dados desse grão
                let cor = self
                    .cores_por_grupo
                    .get(&format!("{:?}", grao.dados))
                    .expect("Cor não encontrada para o grupo de grãos");

                // Ajustar as coordenadas dos grãos para o tamanho da imagem
                let x_px =
                    ((grao.posicao.x as f64 / self.dimensoes.0) * base_dimensions.0 * scale_x)
                        .round() as i32;
                let y_px =
                    ((grao.posicao.y as f64 / self.dimensoes.1) * base_dimensions.1 * scale_y)
                        .round() as i32;

                // Desenha o quadrado (retângulo) representando o grão, com a cor do grupo
                let rect = Rect::at(x_px, y_px).of_size(tamanho_grao as u32, tamanho_grao as u32);
                imageproc::drawing::draw_filled_rect_mut(&mut img, rect, *cor);
            }
        } else {
            return Err("Falha ao adquirir o lock dos grãos.".into());
        }

        // Desenha as formigas
        const VERMELHO: Rgb<u8> = Rgb([255u8, 0u8, 0u8]);
        const AMARELO: Rgb<u8> = Rgb([255u8, 255u8, 0u8]);

        for formiga in self.formigas.iter() {
            if let Ok(pos) = formiga.posicao.lock() {
                // Ajustar as coordenadas das formigas para o tamanho da imagem
                let x_px = ((pos.x as f64 / self.dimensoes.0) * base_dimensions.0 * scale_x).round()
                    as i32;
                let y_px = ((pos.y as f64 / self.dimensoes.1) * base_dimensions.1 * scale_y).round()
                    as i32;

                if let Ok(mao) = formiga.segurando_objeto.lock() {
                    let cor_formiga = if mao.is_some() { AMARELO } else { VERMELHO };

                    // Desenha o quadrado (retângulo) representando a formiga, com tamanho reduzido
                    let rect = Rect::at(x_px, y_px)
                        .of_size(tamanho_formiga as u32, tamanho_formiga as u32);
                    imageproc::drawing::draw_filled_rect_mut(&mut img, rect, cor_formiga);
                }
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

/// Gera uma cor aleatória que não seja similar a vermelho ou amarelo
fn gerar_cor_aleatoria() -> Rgb<u8> {
    let mut rng = rand::thread_rng();

    loop {
        // Gera uma cor aleatória
        let cor = Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);

        // Verifica se a cor gerada é similar a vermelho ou amarelo
        // Critérios simples para evitar vermelho/amarelo:
        // - Evitar cores onde o valor de R (vermelho) é dominante
        // - Evitar valores altos simultâneos de R e G (que formariam amarelo)
        if !cor_semelhante_vermelho_ou_amarelo(&cor) {
            return cor;
        }
    }
}

/// Verifica se uma cor se assemelha a vermelho ou amarelo
fn cor_semelhante_vermelho_ou_amarelo(cor: &Rgb<u8>) -> bool {
    let Rgb([r, g, b]) = cor;

    // Verifica se o vermelho (R) é dominante (evita vermelho)
    if *r > 200 && *g < 100 && *b < 100 {
        return true;
    }

    // Verifica se o vermelho (R) e o verde (G) são altos ao mesmo tempo (evita amarelo)
    if *r > 200 && *g > 200 && *b < 100 {
        return true;
    }

    false
}
