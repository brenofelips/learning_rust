# Rust - Base de Conhecimento

Este arquivo reúne os pontos mais importantes para quem está começando com Rust. A ideia é servir como um resumo de consulta rápida, com conceitos centrais e exemplos curtos.

## 1. O que é Rust

Rust é uma linguagem de programação focada em:

- segurança de memória;
- desempenho;
- controle fino sobre recursos;
- prevenção de erros comuns em tempo de compilação.

O grande diferencial do Rust é combinar performance de linguagem de baixo nível com regras rígidas que evitam muitos bugs antes do programa rodar.

## 2. Variáveis

Em Rust, a forma mais comum de criar uma variável é com `let`.

```rust
let idade = 20;
```

Por padrão, variáveis são **imutáveis**. Isso significa que, depois de atribuída, o valor não pode ser alterado.

```rust
let idade = 20;
idade = 21; // erro
```

Se você quiser permitir alteração, use `mut`.

```rust
let mut idade = 20;
idade = 21;
```

### Variável x constante

Uma variável com `let` guarda um valor que pode ser imutável ou mutável, dependendo de `mut`.

Uma constante é declarada com `const` e sempre precisa de um tipo explícito.

```rust
const PI: f64 = 3.14159;
```

Principais diferenças:

- `let` cria variáveis;
- `const` cria constantes;
- `const` não usa `mut`;
- `const` precisa de tipo explícito;
- `const` é avaliada em tempo de compilação;
- `let` é usada para valores comuns do programa.

### Shadowing

Rust permite reutilizar o nome de uma variável para criar uma nova versão dela.

```rust
let idade = 20;
let idade = idade + 1;
```

Isso é chamado de *shadowing*.

## 3. Funções

Funções são declaradas com `fn`.

```rust
fn soma(a: i32, b: i32) -> i32 {
    a + b
}
```

Pontos importantes:

- parâmetros têm tipo obrigatório;
- o retorno vem após `->`;
- a última expressão sem `;` é o valor retornado;
- `return` também pode ser usado.

```rust
fn dobro(n: i32) -> i32 {
    return n * 2;
}
```

### Expressão x instrução

Em Rust, isso importa bastante:

- **expressão** produz um valor;
- **instrução** executa uma ação e normalmente não retorna valor útil.

Exemplo:

```rust
let x = {
    let y = 3;
    y + 1
};
```

O bloco retorna `4`.

## 4. Tipos primitivos

Rust tem tipos simples embutidos na linguagem.

### Inteiros

- `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

`i` significa com sinal. `u` significa sem sinal.

Exemplos:

```rust
let a: i32 = -10;
let b: u32 = 10;
```

### Ponto flutuante

- `f32`
- `f64`

```rust
let preco: f64 = 19.90;
```

### Booleano

```rust
let ativo: bool = true;
```

### Caractere

```rust
let letra: char = 'R';
```

`char` em Rust representa um caractere Unicode, não apenas ASCII.

### Tuplas

Tuplas agrupam valores de tipos diferentes.

```rust
let pessoa: (&str, i32, bool) = ("Ana", 25, true);
```

Você pode acessar por índice:

```rust
let nome = pessoa.0;
let idade = pessoa.1;
```

### Arrays

Arrays têm tamanho fixo e os elementos precisam ser do mesmo tipo.

```rust
let numeros: [i32; 3] = [1, 2, 3];
```

## 5. Tipos não primitivos

Rust usa estruturas de dados mais ricas para organizar informações.

### String e `&str`

Esses dois aparecem muito no início.

#### `&str`

É uma *string slice*, geralmente uma referência para texto.

```rust
let texto: &str = "Olá";
```

#### `String`

É um texto alocado na heap e mutável, quando declarado com `mut`.

```rust
let mut nome = String::from("Rust");
nome.push_str(" lang");
```

Diferença prática:

- `&str` costuma ser uma referência a texto;
- `String` é uma string proprietária;
- `String` cresce;
- `&str` é muito usada em parâmetros e leitura de texto.

### Structs

Structs agrupam dados relacionados.

```rust
struct Usuario {
    nome: String,
    idade: u32,
}
```

### Enums

Enums representam uma variável que pode assumir uma entre várias formas.

```rust
enum Status {
    Ativo,
    Inativo,
    Pendente,
}
```

Enums são muito importantes em Rust, especialmente com `Option` e `Result`.

### `Option`

Usada quando algo pode existir ou não.

```rust
let valor: Option<i32> = Some(10);
let vazio: Option<i32> = None;
```

### `Result`

Usada para tratar sucesso ou erro.

```rust
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Divisão por zero"))
    } else {
        Ok(a / b)
    }
}
```

## 6. Ownership

Ownership é um dos conceitos centrais de Rust.

Regras básicas:

1. cada valor tem um único dono;
2. quando o dono sai de escopo, o valor é liberado;
3. um valor pode ter apenas um dono por vez.

Exemplo:

```rust
let s1 = String::from("oi");
let s2 = s1;
```

Depois dessa atribuição, `s1` deixa de ser válido, porque a posse foi movida para `s2`.

### Move

Em muitos casos, atribuir uma `String` faz o valor ser movido, não copiado.

### Copy

Tipos simples como inteiros geralmente implementam `Copy`.

```rust
let a = 10;
let b = a;
```

Aqui `a` continua válido, porque inteiros são copiados.

### Funções e ownership

Passar valores para funções também pode mover a posse.

```rust
fn recebe_texto(texto: String) {
    println!("{}", texto);
}
```

Se você passar uma `String`, ela pode ser movida para a função.

## 7. Borrowing

Borrowing é o ato de emprestar um valor sem transferir a posse.

### Referência imutável

```rust
fn tamanho(texto: &String) -> usize {
    texto.len()
}
```

Isso permite usar o valor sem tomá-lo como dono.

### Referência mutável

```rust
fn acrescenta(texto: &mut String) {
    texto.push_str("!");
}
```

Regras importantes:

- só pode haver uma referência mutável por vez;
- ou várias referências imutáveis, ou uma mutável;
- não pode misturar mutável e imutável ao mesmo tempo no mesmo dado.

Essas regras evitam corrida de dados e estados inconsistentes.

## 8. Slices

Slices são visões parciais de uma coleção ou string.

```rust
let texto = String::from("Rust");
let parte = &texto[0..2];
```

Slices não possuem os dados; elas apontam para uma parte deles.

## 9. Escopo

Uma variável só existe dentro do escopo em que foi declarada.

```rust
{
    let nome = "Ana";
}
// nome não existe aqui
```

Quando o escopo termina, Rust libera os recursos associados automaticamente.

## 10. `let`, `mut` e `const`

Resumo rápido:

- `let`: cria variável;
- `mut`: permite modificar a variável;
- `const`: cria constante;
- `const` é fixo e definido em tempo de compilação.

Exemplo:

```rust
let mut contador = 0;
contador += 1;

const MAX_TENTATIVAS: u32 = 3;
```

## 11. Conversões e tipagem

Rust é uma linguagem fortemente tipada. Muitas vezes você precisa declarar tipos explicitamente ou converter valores.

```rust
let n: i32 = 10;
let m: f64 = n as f64;
```

O compilador costuma inferir tipos, mas em vários cenários a anotação explícita deixa o código mais claro.

## 12. Boas práticas iniciais

- prefira imutabilidade por padrão;
- use `mut` apenas quando for necessário;
- use `String` quando precisar de texto alocado e modificável;
- use `&str` quando só precisar ler texto;
- entenda ownership antes de tentar abstrações maiores;
- use `Option` e `Result` em vez de assumir que tudo sempre existe ou sempre dá certo;
- leia os erros do compilador com atenção, porque eles costumam apontar exatamente o que precisa ser corrigido.

## 13. Resumo mental

Se você quiser guardar a ideia central de Rust, pense assim:

- valor tem dono;
- dono controla a vida do valor;
- referências permitem uso sem transferir posse;
- mutabilidade é controlada;
- o compilador protege contra muitos erros;
- `const` é fixo;
- `let` cria variáveis;
- tipos primitivos cobrem números, booleanos, caracteres, tuplas e arrays;
- tipos mais comuns no dia a dia incluem `String`, `&str`, `struct`, `enum`, `Option` e `Result`.

## 14. Mini glossário

- **Ownership**: regra de posse dos valores.
- **Borrowing**: empréstimo de valor via referência.
- **Mutabilidade**: capacidade de alterar um valor.
- **Imutabilidade**: valor não muda depois de criado.
- **Slice**: recorte/referência parcial para dados.
- **Shadowing**: reutilização do nome de uma variável.
- **Heap**: área de memória usada para dados dinâmicos.
- **Stack**: área de memória usada para dados mais simples e controle de chamadas.

---

Este arquivo pode ser expandido com tópicos como `match`, `loop`, `if let`, `traits`, `lifetimes`, módulos e crates.
