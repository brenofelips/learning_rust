pub fn condicionais() {
    let numero = 6;

    if numero > 0 {
        println!("O número é positivo.");
    } else if numero < 0 {
        println!("O número é negativo.");
    } else {
        println!("O número é zero.");
    }

    let numero_condicional = if numero % 2 == 0 { "par" } else { "ímpar" };
    println!("O número {} é {}.", numero, numero_condicional);
}
