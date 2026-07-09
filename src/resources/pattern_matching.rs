// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#match-guards

// explicando o que está acontecendo aqui. O `match` é uma expressão que compara um valor com uma série de padrões e executa o código associado ao primeiro padrão que corresponder. No caso do nosso código, estamos usando o `match` para comparar o valor de `x` com diferentes padrões e retornar uma string correspondente.

// O `match` é uma forma poderosa de controle de fluxo em Rust, permitindo que você lide com diferentes casos de maneira clara e concisa. No nosso exemplo, estamos usando o `match` para categorizar os números de 1 a 20 em diferentes faixas, retornando uma string que descreve a quantidade correspondente a cada número.
pub fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{} é {}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Um pouco mais",
                4..=10 => "Um bucado",
                _ if x % 2 == 0 => "Uma boa quantidade",
                _ => "Muito",
            }
        );
    }
}

// escreva mais 3 exemplos onde eu possa usar o pattern matching, e explique o que está acontecendo em cada um deles. Use exemplos simples, como comparar números ou strings, e explique como o pattern matching ajuda a tornar o código mais legível e fácil de entender.
fn exemplo1() {
    let numero = 5;
    match numero {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        4 => println!("Quatro"),
        5 => println!("Cinco"),
        _ => println!("Outro número"),
    }
}

fn exemplo2() {
    let fruta = "maçã";
    match fruta {
        "banana" => println!("É uma banana"),
        "maçã" => println!("É uma maçã"),
        "laranja" => println!("É uma laranja"),
        _ => println!("Outra fruta"),
    }
}

fn exemplo3() {
    let cor = "vermelho";
    match cor {
        "vermelho" => println!("É vermelho"),
        "verde" => println!("É verde"),
        "azul" => println!("É azul"),
        _ => println!("Outra cor"),
    }
}

// Agora adicione exemplos de pattern matching com tuplas e enums, e explique o que está acontecendo em cada um deles. Use exemplos simples, como comparar coordenadas ou tipos de mensagens, e explique como o pattern matching ajuda a tornar o código mais legível e fácil de entender.
fn exemplo4() {
    let coordenadas = (3, 5);
    match coordenadas {
        (0, 0) => println!("Origem"),
        (x, 0) => println!("Eixo X: {}", x),
        (0, y) => println!("Eixo Y: {}", y),
        (x, y) => println!("Coordenadas: ({}, {})", x, y),
    }
}

// Agora vamos adicionar exemplos de pattern matching com enums. Enums são tipos que podem ter diferentes variantes, e o pattern matching nos permite lidar com cada variante de forma clara e concisa.
enum Mensagem {
    Texto(String),
    Imagem(String),
    Video(String),
}

fn exemplo5() {
    let mensagem = Mensagem::Texto(String::from("Olá, mundo!"));
    match mensagem {
        Mensagem::Texto(texto) => println!("Mensagem de texto: {}", texto),
        Mensagem::Imagem(imagem) => println!("Mensagem de imagem: {}", imagem),
        Mensagem::Video(video) => println!("Mensagem de vídeo: {}", video),
    }
}

// Em resumo, o pattern matching é uma ferramenta poderosa em Rust que nos permite lidar com diferentes casos de forma clara e concisa. Ele nos ajuda a tornar o código mais legível e fácil de entender, permitindo que lidemos com diferentes tipos de dados e estruturas de forma eficiente.

// Adicione mais exemplos de pattern matching com Option e Result, e explique o que está acontecendo em cada um deles. Use exemplos simples, como lidar com valores opcionais ou resultados de operações, e explique como o pattern matching ajuda a tornar o código mais legível e fácil de entender.
fn exemplo6() {
    let valor_opcional: Option<i32> = Some(10);
    match valor_opcional {
        Some(valor) => println!("O valor é: {}", valor),
        None => println!("Não há valor"),
    }
}

// Adicione exemplos mais complexos de pattern matching que envolvam combinações de diferentes tipos de dados, como tuplas, enums e Option, e explique o que está acontecendo em cada um deles. Use exemplos simples, como lidar com coordenadas e mensagens opcionais, e explique como o pattern matching ajuda a tornar o código mais legível e fácil de entender.
fn exemplo7() {
    let coordenadas_opcionais: Option<(i32, i32)> = Some((3, 5));
    match coordenadas_opcionais {
        Some((x, y)) => println!("Coordenadas: ({}, {})", x, y),
        None => println!("Não há coordenadas"),
    }
}

// Adicione exemplos de pattern matching com guardas, e explique o que está acontecendo em cada um deles. Use exemplos simples, como comparar números ou strings com condições adicionais, e explique como o pattern matching ajuda a tornar o código mais legível e fácil de entender.
fn exemplo8() {
    let numero = 10;
    match numero {
        n if n < 0 => println!("Número negativo"),
        n if n == 0 => println!("Número zero"),
        n if n > 0 => println!("Número positivo"),
        _ => println!("Outro número"),
    }
}
