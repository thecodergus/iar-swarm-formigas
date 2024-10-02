mod lib;
use lib::cenario::Cenario;
use lib::formiga::{gerar_formigas, Formiga};
use lib::grao::{gerar_graos, ler_graos_de_arquivo};

fn main() {
    let tamanho: (f64, f64) = (64.0, 64.0);
    let tamanho_mapa: (f64, f64) = tamanho.clone();

    // Criar grãos aleatórios igual a 0,01% do tamanho da matriz
    let quantidade_formigas = 15;

    let mut cenario: Cenario = Cenario::new(
        tamanho,
        gerar_formigas(quantidade_formigas, tamanho_mapa),
        ler_graos_de_arquivo("Square1-DataSet-400itens.txt", tamanho_mapa).unwrap(),
    );
    cenario.start(50_000_000)
}
