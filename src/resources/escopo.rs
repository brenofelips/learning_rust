const PI: f64 = 3.14159; // Constante para o valor de pi
static VAR_GLOBAL: i32 = 100; // Variável global estática

pub fn escopo() {
    // ----------------------- Variáveis -----------------------
    // Variáveis são usadas para armazenar dados que podem mudar durante a execução do programa.
    // Variáveis são declaradas usando a palavra-chave let e podem ser mutáveis ou imutáveis.
    // Variáveis mutáveis são declaradas usando a palavra-chave mut e podem ter seu valor alterado após a declaração.
    // Variáveis imutáveis não podem ter seu valor alterado após a declaração, o que ajuda a evitar erros e tornar o código mais seguro.
    let variavel: u16 = 300;
    println!(
        "O valor da variável é: {} e seu tamanho em bytes é: {:?}",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    let decimal = 3.14;
    println!(
        "O valor do decimal é: {} e seu tamanho em bytes é: {:?}",
        decimal,
        std::mem::size_of_val(&decimal)
    );

    let mut booleano = true; // Variável mutável para armazenar um valor booleano
    println!(
        "O valor do booleano é: {} e seu tamanho em bytes é: {:?}",
        booleano,
        std::mem::size_of_val(&booleano)
    );

    let caractere = 'A';
    println!(
        "O valor do caractere é: {} e seu tamanho em bytes é: {:?}",
        caractere,
        std::mem::size_of_val(&caractere)
    );

    booleano = false; // Modificando o valor do booleano para false
    println!(
        "O valor do booleano após modificação é: {} e seu tamanho em bytes é: {:?}",
        booleano,
        std::mem::size_of_val(&booleano)
    );

    // ----------------------- Contsantes -----------------------
    // Constantes são valores imutáveis que são definidos em tempo de compilação. Elas não podem ser alteradas durante a execução do programa.
    // Constantes são declaradas usando a palavra-chave const e devem ter um tipo explícito.
    // Constantes são úteis para valores que não mudam, como o valor de pi ou a velocidade da luz.
    // Constantes são armazenadas na memória de forma eficiente, pois o compilador pode otimizar seu uso.
    // fazer uma constante global é uma boa prática para valores que são usados em várias partes do programa, evitando a repetição de código e facilitando a manutenção.
    println!(
        "O valor da constante PI é: {} e seu tamanho em bytes é: {:?}",
        PI,
        std::mem::size_of_val(&PI)
    );
    println!("O valor da variável global VAR_GLOBAL é: {}", VAR_GLOBAL);
}
