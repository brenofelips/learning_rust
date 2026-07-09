// Cada função agora vive em seu próprio arquivo dentro da pasta `resources`.
// O atributo `#[path = "..."]` indica ao compilador onde encontrar o módulo.
#[path = "resources/condicionais.rs"]
mod condicionais;
#[path = "resources/escopo.rs"]
mod escopo;
#[path = "resources/repeticoes.rs"]
mod repeticoes;
#[path = "resources/soma.rs"]
mod soma;
#[path = "resources/sombra.rs"]
mod sombra;
// #[path = "resources/ownership.rs"]
// mod ownership;
#[path = "resources/pattern_matching.rs"]
mod pattern_matching;

fn main() {
    escopo::escopo();
    sombra::sombra();
    soma::soma(10, 20);
    // ownership::ownership();
    repeticoes::repeticoes();
    condicionais::condicionais();
    pattern_matching::pattern_matching();
}
