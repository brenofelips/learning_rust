pub fn sombra() {
    let a = 123;

    {
        let b = 456;
        let a = 789;
        println!("a no escopo: {}, b no escopo: {}", a, b);
    }

    println!("a fora do escopo: {}", a);
}
