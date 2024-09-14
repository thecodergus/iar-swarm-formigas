mod lib;
use lib::cenario::Cenario;
use lib::formiga::Formiga;

fn main() {
    let tamanho: (f64, f64) = (800.0, 600.0);
    let mut cenario: Cenario = Cenario::new(tamanho);
    cenario.start();
}
