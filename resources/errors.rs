// Errors: em PT-BR, erros são situações inesperadas que podem ocorrer durante a execução de um programa. Em Rust, existem dois tipos principais de erros: erros recuperáveis e erros irrecuperáveis.

// Erros recuperáveis são aqueles que podem ser tratados e recuperados, permitindo que o programa continue sua execução. Um exemplo de erro recuperável é quando tentamos abrir um arquivo que não existe. Nesse caso, podemos tratar o erro e fornecer uma mensagem adequada ao usuário, ou tentar abrir outro arquivo. Em Rust, erros recuperáveis são representados pelo tipo Result<T, E>, onde T é o tipo de valor esperado em caso de sucesso, e E é o tipo de erro que pode ocorrer. Podemos usar o pattern matching para lidar com os diferentes casos de sucesso e erro, tornando o código mais legível e fácil de entender.

// Erros irrecuperáveis, por outro lado, são aqueles que não podem ser tratados e que geralmente resultam na interrupção do programa. Um exemplo de erro irrecuperável é quando tentamos acessar um índice fora dos limites de um vetor. Nesse caso, o programa entra em pânico e é encerrado. Em Rust, erros irrecuperáveis são representados pelo tipo panic!, que indica que o programa encontrou uma situação inesperada e não pode continuar sua execução. É importante tratar erros irrecuperáveis com cuidado, garantindo que o programa seja robusto e confiável.


pub fn errors() {
  let v = vec![1, 2, 3];
  // em pt-BR: Isso vai causar um pânico porque o índice 99 está fora dos limites
  // Esse erro é inrecuperável, então o programa vai parar de rodar.
  v[99]; // This will cause a panic because index 99 is out of bounds
}

pub fn errors_recoverable() {
  match result_recoverable() {
    Ok(message) => println!("Sucesso: {}", message),
    Err(code) => println!("Erro recuperável com código: {}", code),
  }
}

fn result_recoverable() -> Result<String, u8> {
    Ok(String::from("Operação bem-sucedida"))
    Err(42) // Código do erro
}


// Adicione mais exemplos de erros recuperáveis e irrecuperáveis, e explique o que está acontecendo em cada um deles. Use exemplos simples, como tentar abrir um arquivo que não existe ou acessar um índice fora dos limites de um vetor, e explique como o tratamento de erros ajuda a tornar o código mais robusto e confiável.
fn exemplo_erro_recoveravel() {
    let resultado = std::fs::read_to_string("arquivo_inexistente.txt");
    match resultado {
        Ok(conteudo) => println!("Conteúdo do arquivo: {}", conteudo),
        Err(erro) => println!("Erro ao abrir o arquivo: {}", erro),
    }
}