# Repetições (Loops) em Rust

Este documento explica as estruturas de repetição utilizadas na função `repeticoes()`
do arquivo `main.rs`. Em Rust existem três formas principais de repetir um bloco de
código: `while`, `loop` e `for`. Além delas, o exemplo também usa `match`, que não é um
laço, mas aparece na mesma função e é explicado ao final.

Em todos os exemplos abaixo usamos a montagem de uma **tabuada** como caso prático,
com um `multiplicador` (5) e um `contador`.

```rust
let multiplicador: u8 = 5;
let mut contador: u8 = 0;
```

> Note o `mut` em `contador`: ele precisa ser mutável porque seu valor muda a cada
> iteração. Já o `multiplicador` é imutável, pois nunca é alterado.

---

## 1. `while` — repete enquanto a condição for verdadeira

O laço `while` executa o bloco **enquanto** a condição informada continuar sendo
`true`. A condição é testada **antes** de cada iteração.

```rust
let multiplicador: u8 = 5;
let mut contador: u8 = 0;

println!("Tabuada do {} usando while:", multiplicador);
while contador < 10 {
    contador += 1;
    println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
}
```

**Como funciona, passo a passo:**

1. Rust verifica se `contador < 10`.
2. Se for verdadeiro, entra no bloco, incrementa `contador` e imprime a linha.
3. Volta ao passo 1. Quando `contador` chega a 10, a condição vira `false` e o laço termina.

**Saída:**

```
Tabuada do 5 usando while:
5 x 1 = 5
5 x 2 = 10
...
5 x 10 = 50
```

**Quando usar:** quando você não sabe de antemão quantas vezes o laço vai rodar, mas
sabe a **condição de parada**.

---

## 2. `loop` — repete para sempre até um `break`

O `loop` cria um laço **infinito**. Ele só termina quando um `break` é executado
explicitamente dentro do bloco. É útil quando a condição de saída fica no meio ou no
fim do processamento.

```rust
println!("Tabuada do {} usando loop:", multiplicador);
contador = 0; // Resetando o contador para a próxima repetição
loop {
    contador += 1;
    println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    if contador >= 10 {
        break; // Saindo do loop quando o contador atingir 10
    }
}
```

**Como funciona, passo a passo:**

1. O bloco executa incondicionalmente.
2. A cada volta, incrementa e imprime.
3. Quando `contador >= 10`, o `break` interrompe o laço.

> Observe que foi necessário **resetar** `contador = 0` antes de iniciar, pois ele já
> valia 10 após o laço `while` anterior.

**Detalhe extra:** diferente de `while` e `for`, o `loop` pode **retornar um valor**
através do `break`, por exemplo: `let x = loop { break 42; };`.

**Quando usar:** quando você precisa de um laço que roda indefinidamente até um evento
interno (ex.: ler entrada do usuário até um comando de saída).

---

## 3. `for` — itera sobre uma sequência/intervalo

O `for` percorre os elementos de um intervalo ou coleção. É a forma mais segura e
idiomática em Rust, pois não exige controlar um contador manualmente (evitando erros
de índice e a necessidade de `mut`).

```rust
println!("Tabuada do {} usando for:", multiplicador);
for i in 1..=10 {
    println!("{} x {} = {}", multiplicador, i, multiplicador * i);
}
```

**Sobre o intervalo (`range`):**

- `1..=10` → intervalo **inclusivo**: vai de 1 até 10 (incluindo o 10).
- `1..10`  → intervalo **exclusivo**: vai de 1 até 9 (o 10 fica de fora).

**Como funciona:** a variável `i` assume cada valor do intervalo, uma vez por iteração,
automaticamente. Não há `contador` nem `mut`.

**Quando usar:** quando você sabe exatamente sobre **o que** vai iterar (um intervalo
fixo ou uma coleção). É a opção preferida na maioria dos casos.

---

## Comparativo rápido

| Estrutura | Testa a condição | Precisa de contador manual | Caso de uso típico                          |
|-----------|------------------|----------------------------|---------------------------------------------|
| `while`   | Antes de cada volta | Sim                     | Repetir enquanto uma condição for verdadeira |
| `loop`    | Não (infinito)   | Sim                        | Repetir até um evento interno (`break`)      |
| `for`     | —                | Não                        | Iterar sobre intervalo ou coleção            |

Todas as três produzem **a mesma tabuada** no exemplo — a escolha depende da clareza e
da intenção do código.

---

## Bônus: `match` (não é repetição)

Ao final da função `repeticoes()` há um `match`, que é uma estrutura de **decisão**
(parecida com `switch` de outras linguagens), e não um laço:

```rust
let linguagem = "Rust";
let proposito = match linguagem {
    "Rust" => "sistemas e segurança",
    "Python" => "desenvolvimento web e ciência de dados",
    "JavaScript" => "desenvolvimento web",
    _ => "Propósito desconhecido",
};
println!("O propósito da linguagem {} é: {}", linguagem, proposito);
```

O `match` compara o valor de `linguagem` com cada padrão (`"Rust"`, `"Python"`, ...) e
executa o braço correspondente. O `_` é o caso **padrão** (default), obrigatório aqui
para cobrir qualquer outro valor — em Rust o `match` precisa ser **exaustivo**.
