// Ownership é um conceito central em Rust que garante a segurança de memória e evita problemas como vazamentos de memória e condições de corrida. Em Rust, cada valor tem um único dono, e quando o dono sai do escopo, o valor é automaticamente desalocado. Isso significa que você não precisa se preocupar com gerenciamento manual de memória, como em outras linguagens.

// Implementamos três funções para demonstrar diferentes aspectos do ownership em Rust: `ownership`, `ownership_mut` e `ownership_error`. Cada função ilustra como a propriedade de uma string é transferida ou emprestada entre funções, mostrando como o Rust lida com a segurança de memória e o gerenciamento de recursos.

// A função `ownership` demonstra como a propriedade de uma string é transferida para outra função. A string `uma_string_imut` é criada e passada como referência para a função `roubar_ownership`, que imprime seu valor. Após a chamada da função, a string ainda está acessível no escopo original, mostrando que a propriedade não foi transferida, mas sim emprestada.
pub fn ownership() {
    let uma_string_imut = String::from("Olá, mundo!");
    roubar_ownership(&uma_string_imut);
    println!("{}", uma_string_imut);
}

fn roubar_ownership(s_imut: &String) {
    println!("A string roubada é: {}", s_imut);
}


// A função `ownership_mut` demonstra como a propriedade de uma string pode ser mutável. A string `uma_string_mut` é criada como mutável e passada como referência mutável para a função `roubar_ownership_mut`, que modifica seu valor. Após a chamada da função, a string ainda está acessível no escopo original, mostrando que a propriedade foi emprestada de forma mutável.
pub fn ownership_mut() {
    let mut uma_string_mut = String::from("Olá, mundo!");
    roubar_ownership_mut(&mut uma_string_mut);
    println!("{}", uma_string_mut);
}

fn roubar_ownership_mut(s_mut: &mut String) {
    s_mut.push_str(" Adicionando mais texto.");
    println!("A string roubada é: {}", s_mut);
}


// A função `ownership_error` demonstra um erro comum em Rust relacionado à propriedade. A string `uma_string_imut` é criada e passada para a função `roubar_ownership_error`, que tenta imprimir seu valor. No entanto, após a chamada da função, a string não está mais acessível no escopo original, resultando em um erro de compilação. Isso ocorre porque a propriedade da string foi transferida para a função, e o dono original não pode mais acessá-la.
pub fn ownership_error() {
    let uma_string_imut = String::from("Olá, mundo!");
    roubar_ownership_error(uma_string_imut);
    println!("{}", uma_string_imut);
}

fn roubar_ownership_error(s_imut: String) {
    println!("A string roubada é: {}", s_imut);
}