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
                .entry(format!("{:?}", grao.grupo))
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
        // Inicializa o contador compartilhado
        let contador = Arc::new(Mutex::new(numero_interacoes));

        // Inicia as threads das formigas
        for formiga in &mut self.formigas {
            formiga.start(
                self.dimensoes.clone(),
                Arc::clone(&self.graos),
                Arc::clone(&contador),
            );
        }

        // Controla as porcentagens de progresso para gerar imagens
        let porcentagens = [0.75, 0.50, 0.25, 0.0];
        let mut gerou_percentuais = [false; 4];

        let mut contador_img: i64 = 0;

        // Loop principal
        loop {
            if let Ok(mut contador_guard) = contador.lock() {
                // Quando o contador atinge zero, encerra o loop
                if *contador_guard <= 0 {
                    println!("Fim do programa");
                    if let Err(e) =
                        self.gerar_imagem_com_log("Cenario-final.png", &mut contador_img)
                    {
                        eprintln!("Erro ao gerar a imagem final: {}", e);
                    }
                    break;
                }

                // Calcula o percentual restante
                let percentual_restante = *contador_guard as f64 / numero_interacoes as f64;

                // Gera imagens em diferentes estágios de progresso
                for (i, &percentual) in porcentagens.iter().enumerate() {
                    if !gerou_percentuais[i] && percentual_restante <= percentual {
                        if let Err(e) = self.gerar_imagem_com_log(
                            &format!("Cenario-{}.png", contador_img),
                            &mut contador_img,
                        ) {
                            eprintln!("Erro ao gerar a imagem: {}", e);
                        }
                        gerou_percentuais[i] = true;
                    }
                }
            }
        }
    }

    /// Função auxiliar para gerar a imagem e fazer o log
    fn gerar_imagem_com_log(
        &self,
        path: &str,
        contador_img: &mut i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Gerando imagem: {}", path);
        self.gerar_imagem(path, (800, 640))?;
        *contador_img += 1;
        println!("Imagem {} gerada com sucesso!", path);
        Ok(())
    }

    pub fn gerar_imagem(
        &self,
        path: &str,
        image_dimensions: (u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (img_width, img_height) = image_dimensions;
        let base_dimensions: (f64, f64) = self.dimensoes;

        // Cria uma imagem com fundo preto
        let mut img = ImageBuffer::from_pixel(img_width, img_height, Rgb([0u8, 0u8, 0u8]));

        // Escalar o tamanho da imagem real em relação à base_dimensions fornecida
        let scale_x = img_width as f64 / base_dimensions.0;
        let scale_y = img_height as f64 / base_dimensions.1;

        // Reduz o tamanho dos grãos e das formigas pela metade
        let tamanho_grao = ((1.0 * scale_x / 1.0).round() as i32) / 2; // Reduzido pela metade
        let raio_formiga = (((1.0 * scale_x / 1.0).round() as i32) / 2) / 2; // Raio também reduzido pela metade

        // Desenha os grãos
        if let Ok(graos) = self.graos.lock() {
            println!("Numero de grãos: {}", graos.len());

            for grao in graos.iter() {
                // Verifica se já existe uma cor associada ao grupo de dados desse grão
                let cor = self
                    .cores_por_grupo
                    .get(&format!("{:?}", grao.grupo))
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

                    // Desenha um círculo representando a formiga, com a cor apropriada
                    draw_filled_circle_mut(&mut img, (x_px, y_px), raio_formiga, cor_formiga);
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
