pub fn repeticoes() {
    let multiplicador: u8 = 5;
    let mut contador: u8 = 0;
    // tabuada usando while
    println!("Tabuada do {} usando while:", multiplicador);
    while contador < 10 {
        contador += 1;
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );
    }

    // tabuada usando loop
    println!("Tabuada do {} usando loop:", multiplicador);
    contador = 0; // Resetando o contador para a próxima repetição
    loop {
        contador += 1;
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );
        if contador >= 10 {
            break; // Saindo do loop quando o contador atingir 10
        }
    }

    // tabuada usando for
    println!("Tabuada do {} usando for:", multiplicador);
    for i in 1..=10 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }

    let linguagem = "Rust";
    let proposito = match linguagem {
        "Rust" => "sistemas e segurança",
        "Python" => "desenvolvimento web e ciência de dados",
        "JavaScript" => "desenvolvimento web",
        _ => "Propósito desconhecido",
    };
    println!("O propósito da linguagem {} é: {}", linguagem, proposito);
}
