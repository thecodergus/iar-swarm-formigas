mod lib;
use lib::cenario::Cenario;
use lib::formiga::{gerar_formigas, Formiga};
use lib::grao::gerar_graos;

fn main() {
    let tamanho: (f64, f64) = (800.0, 600.0);
    let tamanho_mapa: (f64, f64) = tamanho.clone();
    
    // Criar grãos aleatórios igual a 0,01% do tamanho da matriz
    let quantidade_graos = (tamanho.0 * tamanho.1 * 0.001) as i32;

    let mut cenario: Cenario = Cenario::new(tamanho, gerar_formigas(20, tamanho_mapa), gerar_graos(quantidade_graos, tamanho_mapa));
    cenario.start(500_000_000)
}
