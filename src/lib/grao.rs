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
    pub grupo: i32,
}

impl Grao {
    pub fn new(posicao: Ponto, dados: Vec<f64>, grupo: i32) -> Grao {
        Grao {
            id: Uuid::new_v4(),
            posicao,
            dados,
            grupo,
        }
    }
}

pub fn gerar_graos(
    numero: i32,
    tamanho_mapa: (f64, f64),
    dados: Vec<f64>,
    grupo: i32,
) -> Vec<Grao> {
    let mut graos: Vec<Grao> = vec![];

    let mut rng = rand::thread_rng();

    for _ in 0..numero {
        let x: i32 = rng.gen_range(0..=(tamanho_mapa.0 as i32));
        let y: i32 = rng.gen_range(0..=tamanho_mapa.1 as i32);
        graos.push(Grao::new(Ponto { x: x, y: y }, dados.clone(), grupo));
    }

    graos
}

pub fn ler_graos_de_arquivo_normalizado(
    caminho: &str,
    tamanho_mapa: (f64, f64),
) -> Result<Vec<Grao>, Box<dyn Error>> {
    // Abre o arquivo
    let arquivo = File::open(caminho)?;

    // Usa um buffer para ler o arquivo linha por linha
    let leitor = io::BufReader::new(arquivo);

    // Vetor para armazenar todas as linhas de dados
    let mut todas_linhas: Vec<Vec<f64>> = Vec::new();

    // Itera sobre as linhas do arquivo
    for linha in leitor.lines() {
        let linha = linha?;
        let valores: Vec<&str> = linha.split_whitespace().collect();

        // Transforma cada valor da linha em f64
        let mut dados: Vec<f64> = valores
            .iter()
            .map(|&valor| valor.replace(",", ".").parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()?;

        // Realiza qualquer troca necessária, se aplicável (no seu caso, troca o índice 0 com o 2)
        dados.swap(0, 2);

        // Adiciona a linha processada ao vetor
        todas_linhas.push(dados);
    }

    // Vetores para armazenar os valores mínimos e máximos de cada coluna
    let num_colunas = todas_linhas[0].len();
    let mut min_vals = vec![f64::MAX; num_colunas];
    let mut max_vals = vec![f64::MIN; num_colunas];

    // Calcula os valores mínimos e máximos de cada coluna
    for linha in &todas_linhas {
        for i in 0..num_colunas {
            if linha[i] < min_vals[i] {
                min_vals[i] = linha[i];
            }
            if linha[i] > max_vals[i] {
                max_vals[i] = linha[i];
            }
        }
    }

    // Vetor para armazenar os grãos
    let mut graos: Vec<Grao> = Vec::new();
    let mut rng = rand::thread_rng();

    // Normaliza os dados e cria os grãos com posições aleatórias
    for mut dados in todas_linhas {
        let grupo: i32 = dados[0] as i32;
        dados.remove(0);

        let dados_normalizados: Vec<f64> = dados
            .iter()
            .enumerate()
            .map(|(i, &valor)| {
                if max_vals[i] - min_vals[i] == 0.0 {
                    // Evita divisão por zero caso todos os valores da coluna sejam iguais
                    0.0
                } else {
                    (valor - min_vals[i]) / (max_vals[i] - min_vals[i])
                }
            })
            .collect();

        // Gera posições aleatórias dentro do tamanho do mapa
        let x = rng.gen_range(0.0..tamanho_mapa.0) as i32;
        let y = rng.gen_range(0.0..tamanho_mapa.1) as i32;

        // Cria o grão com a posição aleatória e os dados normalizados
        let grao = Grao::new(Ponto { x, y }, dados_normalizados, grupo);

        // Adiciona o grão ao vetor
        graos.push(grao);
    }

    // Retorna o vetor de grãos
    Ok(graos)
}

pub fn ler_graos_de_arquivo(
    caminho: &str,
    tamanho_mapa: (f64, f64),
) -> Result<Vec<Grao>, Box<dyn Error>> {
    // Abre o arquivo
    let arquivo = File::open(caminho)?;

    // Usa um buffer para ler o arquivo linha por linha
    let leitor = io::BufReader::new(arquivo);

    // Vetor para armazenar todas as linhas de dados
    let mut todas_linhas: Vec<Vec<f64>> = Vec::new();

    // Itera sobre as linhas do arquivo
    for linha in leitor.lines() {
        let linha = linha?;
        let valores: Vec<&str> = linha.split_whitespace().collect();

        // Transforma cada valor da linha em f64
        let mut dados: Vec<f64> = valores
            .iter()
            .map(|&valor| valor.replace(",", ".").parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()?;

        // Realiza qualquer troca necessária, se aplicável (no seu caso, troca o índice 0 com o 2)
        dados.swap(0, 2);

        // Adiciona a linha processada ao vetor
        todas_linhas.push(dados);
    }

    // Vetor para armazenar os grãos
    let mut graos: Vec<Grao> = Vec::new();
    let mut rng = rand::thread_rng();

    // Cria os grãos com posições aleatórias (sem normalizar os dados)
    for mut dados in todas_linhas {
        let grupo: i32 = dados[0] as i32;
        dados.remove(0);

        // Gera posições aleatórias dentro do tamanho do mapa
        let x = rng.gen_range(0.0..tamanho_mapa.0) as i32;
        let y = rng.gen_range(0.0..tamanho_mapa.1) as i32;

        // Cria o grão com a posição aleatória e os dados sem normalização
        let grao = Grao::new(Ponto { x, y }, dados, grupo);

        // Adiciona o grão ao vetor
        graos.push(grao);
    }

    // Retorna o vetor de grãos
    Ok(graos)
}
