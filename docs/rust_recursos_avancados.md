# Rust - Recursos Avançados

Este arquivo reúne anotações de recursos mais avançados do Rust. A ideia é servir como referência rápida para estudo e revisão.

## 1. Generics

Generics permitem escrever código reutilizável para diferentes tipos.

```rust
fn maior<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### Ideia principal

- reduzem duplicação;
- deixam funções, structs e enums mais flexíveis;
- normalmente exigem *traits bounds*.

## 2. Traits

Traits definem comportamento compartilhado.

```rust
trait Descrevivel {
    fn descricao(&self) -> String;
}
```

Implementação:

```rust
struct Usuario {
    nome: String,
}

impl Descrevivel for Usuario {
    fn descricao(&self) -> String {
        format!("Usuário: {}", self.nome)
    }
}
```

### Trait bounds

Você pode restringir tipos genéricos com traits.

```rust
fn exibe<T: std::fmt::Display>(valor: T) {
    println!("{}", valor);
}
```

## 3. Lifetimes

Lifetimes ajudam o compilador a entender por quanto tempo as referências são válidas.

### Objetivo

- evitar referências penduradas;
- garantir que dados referenciados continuem válidos enquanto forem usados.

Exemplo simples:

```rust
fn maior<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

### Quando aparecem

- em funções que retornam referências;
- em structs com referências;
- em APIs mais complexas.

## 4. Closures

Closures são funções anônimas que podem capturar variáveis do ambiente.

```rust
let soma = |a, b| a + b;
println!("{}", soma(2, 3));
```

### Características

- podem capturar valores por referência, mut referência ou por movimento;
- são úteis em iteradores, callbacks e APIs funcionais.

## 5. Iterators

Iterators permitem processar coleções de forma expressiva.

```rust
let numeros = vec![1, 2, 3, 4];
let dobrados: Vec<i32> = numeros.iter().map(|n| n * 2).collect();
```

### Métodos comuns

- `iter()`
- `iter_mut()`
- `into_iter()`
- `map()`
- `filter()`
- `fold()`
- `collect()`

### Vantagens

- código mais conciso;
- menos mutação manual;
- boa composição de operações.

## 6. Pattern matching

O `match` é um dos recursos mais fortes do Rust.

```rust
match numero {
    0 => println!("zero"),
    1..=9 => println!("pequeno"),
    _ => println!("outro"),
}
```

### Usos frequentes

- destravar `enum`;
- tratar `Option` e `Result`;
- validar faixas e padrões.

## 7. `if let` e `while let`

Essas construções simplificam casos em que você quer tratar apenas um padrão.

```rust
if let Some(valor) = opcao {
    println!("{}", valor);
}
```

```rust
while let Some(item) = pilha.pop() {
    println!("{}", item);
}
```

## 8. Error handling

Rust trata erros de forma explícita, geralmente com `Result`.

```rust
fn ler_arquivo() -> Result<String, std::io::Error> {
    std::fs::read_to_string("dados.txt")
}
```

### Boas práticas

- use `Result` para falhas recuperáveis;
- use `panic!` só para erros irreversíveis ou violação de invariantes;
- propague erro com `?` quando fizer sentido.

```rust
fn carregar() -> Result<String, std::io::Error> {
    let conteudo = std::fs::read_to_string("dados.txt")?;
    Ok(conteudo)
}
```

## 9. Smart pointers

Smart pointers são tipos com comportamento extra em relação a referências simples.

### Principais

- `Box<T>`: coloca dados na heap;
- `Rc<T>`: contagem de referências em contexto de thread única;
- `Arc<T>`: contagem de referências segura para múltiplas threads;
- `RefCell<T>`: empréstimo verificado em tempo de execução;
- `Mutex<T>`: exclusão mútua para acesso concorrente.

### Quando usar

- para ownership compartilhado;
- para estruturas recursivas;
- para interior mutability;
- para concorrência.

## 10. Modules e crates

Rust organiza código com módulos e crates.

### Módulos

```rust
mod utils {
    pub fn soma(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

### Itens públicos

Use `pub` para expor funções, structs, enums e campos.

### Crates

- `crate` é a unidade de compilação;
- pode ser binária ou biblioteca;
- o arquivo `Cargo.toml` define dependências e metadados.

## 11. Cargo

`cargo` é o gerenciador de build e dependências do Rust.

### Comandos úteis

- `cargo build`
- `cargo run`
- `cargo test`
- `cargo fmt`
- `cargo clippy`

### Papel do Cargo

- compila o projeto;
- resolve dependências;
- organiza testes e documentação;
- padroniza o fluxo de desenvolvimento.

## 12. Concurrency

Rust favorece concorrência segura.

### Conceitos úteis

- `thread::spawn` para iniciar threads;
- `Arc<T>` para compartilhamento;
- `Mutex<T>` para proteção de dados mutáveis;
- channels para comunicação entre threads.

Exemplo básico:

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("rodando em outra thread");
});

handle.join().unwrap();
```

### Ideia central

O compilador ajuda a evitar data races antes da execução.

## 13. Macros

Macros permitem gerar código.

### Tipos comuns

- `macro_rules!` para macros declarativas;
- proc macros para geração e transformação de código mais avançada.

Exemplo simples:

```rust
macro_rules! diga_ola {
    () => {
        println!("Olá!");
    };
}
```

### Quando valem a pena

- para reduzir boilerplate;
- para APIs flexíveis;
- para geração de código repetitivo.

## 14. Async e await

Rust também suporta programação assíncrona.

```rust
async fn busca_dados() -> String {
    "dados".to_string()
}
```

### Ideia principal

- `async` marca uma função assíncrona;
- `await` espera o resultado;
- normalmente é usado com um runtime, como Tokio ou async-std.

## 15. Unsafe em contexto avançado

`unsafe` não é um recurso para usar cedo, mas é importante em código avançado.

### Pode aparecer em

- ponteiros brutos;
- FFI com C;
- manipulação de memória;
- implementações de baixo nível.

O ideal é isolar o código `unsafe` e manter toda a lógica externa em `safe Rust`.

## 16. FFI

FFI significa *Foreign Function Interface*.

É o mecanismo que permite chamar código de outras linguagens, normalmente C.

### Uso comum

- integração com bibliotecas nativas;
- acesso a APIs do sistema;
- reaproveitamento de código legado.

## 17. `Box`, `Rc`, `Arc`, `RefCell` e `Mutex`

Esses tipos aparecem muito quando o ownership fica mais complexo.

### Resumo rápido

- `Box<T>`: um dono, dados na heap;
- `Rc<T>`: vários donos, thread única;
- `Arc<T>`: vários donos, multi-thread;
- `RefCell<T>`: regras de borrow checadas em runtime;
- `Mutex<T>`: acesso exclusivo com bloqueio.

## 18. Testes

Rust tem suporte forte a testes embutidos.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn soma_ok() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Tipos de teste

- testes unitários;
- testes de integração;
- testes de documentação.

## 19. Documentação

Rust incentiva documentação com `///`.

```rust
/// Soma dois números.
fn soma(a: i32, b: i32) -> i32 {
    a + b
}
```

## 20. Resumo mental

Se você estiver estudando recursos avançados de Rust, foque nesta ordem:

1. ownership e borrowing bem entendidos;
2. traits e generics;
3. lifetimes;
4. iterators e closures;
5. `Result` e tratamento de erros;
6. módulos, crates e Cargo;
7. concorrência com `Arc`, `Mutex` e channels;
8. macros e `unsafe` quando necessário.

---

Esses tópicos cobrem boa parte do Rust avançado usado em projetos reais. Depois disso, vale estudar async mais a fundo, macros procedurais, FFI e padrões de arquitetura da comunidade.
