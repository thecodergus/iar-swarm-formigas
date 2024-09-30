use std::vec;

use super::outros::Ponto;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Grao {
    pub id: Uuid,
    pub posicao: Ponto,
    pub dados: Vec<f64>,
}

impl Grao {
    pub fn new(posicao: Ponto, dados: Vec<f64>) -> Grao {
        Grao {
            id: Uuid::new_v4(),
            posicao,
            dados,
        }
    }
}

pub fn gerar_graos(numero: i32, tamanho_mapa: (f64, f64), dados: Vec<f64>) -> Vec<Grao> {
    let mut graos: Vec<Grao> = vec![];

    let mut rng = rand::thread_rng();

    for _ in 0..numero {
        let x: i32 = rng.gen_range(0..=(tamanho_mapa.0 as i32));
        let y: i32 = rng.gen_range(0..=tamanho_mapa.1 as i32);
        graos.push(Grao::new(Ponto { x: x, y: y }, dados.clone()));
    }

    graos
}

/// Função que lê o arquivo e cria grãos com posições aleatórias e valores de dados do arquivo
pub fn ler_graos_de_arquivo(
    caminho: &str,
    tamanho_mapa: (f64, f64),
) -> Result<Vec<Grao>, Box<dyn Error>> {
    // Abre o arquivo
    let arquivo = File::open(caminho)?;

    // Usa um buffer para ler o arquivo linha por linha
    let leitor = io::BufReader::new(arquivo);

    // Vetor para armazenar os grãos
    let mut graos: Vec<Grao> = Vec::new();

    // Gera um RNG para as posições aleatórias
    let mut rng = rand::thread_rng();

    // Itera sobre as linhas do arquivo
    for linha in leitor.lines() {
        let linha = linha?;
        let valores: Vec<&str> = linha.split_whitespace().collect();

        // Transforma cada valor da linha em f64
        let mut dados: Vec<f64> = valores
            .iter()
            .map(|&valor| valor.replace(",", ".").parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()?;

        dados.swap(0, 2);

        // Gera posições aleatórias dentro do tamanho do mapa
        let x = rng.gen_range(0.0..tamanho_mapa.0) as i32;
        let y = rng.gen_range(0.0..tamanho_mapa.1) as i32;

        // Cria o grão com a posição aleatória e os dados da linha
        let grao = Grao::new(Ponto { x, y }, dados);

        // Adiciona o grão ao vetor
        graos.push(grao);
    }

    // Retorna o vetor de grãos
    Ok(graos)
}
