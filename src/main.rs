mod lib;
use lib::cenario::Cenario;
use lib::formiga::{gerar_formigas, Formiga};
use lib::grao::gerar_graos;

fn main() {
    let tamanho: (f64, f64) = (64.0, 64.0);
    let tamanho_mapa: (f64, f64) = tamanho.clone();

    // Criar grãos aleatórios igual a 0,01% do tamanho da matriz
    let quantidade_graos = 400;
    let quantidade_formigas = 100;

    let mut cenario: Cenario = Cenario::new(
        tamanho,
        gerar_formigas(quantidade_formigas, tamanho_mapa),
        gerar_graos(quantidade_graos / 2, tamanho_mapa, vec![2.0, 3.0])
            .into_iter()
            .chain(gerar_graos(quantidade_graos / 2, tamanho_mapa, vec![5.0, 7.0]).into_iter())
            .collect::<Vec<_>>(),
    );
    cenario.start(12_000_000)
}
