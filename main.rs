// Cada função agora vive em seu próprio arquivo dentro da pasta `resources`.
// O atributo `#[path = "..."]` indica ao compilador onde encontrar o módulo.
#[path = "resources/repeticoes.rs"]
mod repeticoes;
#[path = "resources/escopo.rs"]
mod escopo;
#[path = "resources/sombra.rs"]
mod sombra;
#[path = "resources/soma.rs"]
mod soma;
#[path = "resources/condicionais.rs"]
mod condicionais;

fn main() {
    repeticoes::repeticoes();
    escopo::escopo();
    sombra::sombra();
    soma::soma(10, 20);
    condicionais::condicionais();
}
