# `unsafe` em Rust

Este arquivo resume o conceito de `unsafe` em Rust e quando ele costuma aparecer.

## O que é `unsafe`

Rust é uma linguagem muito cuidadosa com segurança de memória e concorrência. Em `safe Rust`, o compilador verifica várias regras para evitar erros comuns.

O bloco `unsafe` é uma forma de dizer ao compilador:

> eu sei que estou saindo das garantias normais de segurança e assumo a responsabilidade por isso.

Exemplo:

```rust
unsafe {
    // código com responsabilidade manual de segurança
}
```

## Analogias úteis

Pense no Rust como um construtor de casas extremamente cuidadoso. Ele segue regras rígidas para evitar desabamentos e problemas estruturais.

Quando você usa `unsafe`, é como pedir uma licença especial para executar algo que o compilador não consegue validar totalmente.

## Por que `unsafe` existe

`unsafe` existe porque algumas tarefas exigem acesso mais direto ao sistema.

### Casos comuns

- interoperabilidade com outras linguagens, como C e C++;
- manipulação de ponteiros brutos;
- acesso a APIs de baixo nível;
- algumas otimizações específicas;
- implementação de estruturas avançadas.

## `static mut` e `unsafe`

Um exemplo clássico de uso de `unsafe` é o acesso a variáveis globais mutáveis.

```rust
static mut GLOBAL: u32 = 0;
```

Esse tipo de valor é perigoso porque pode ser acessado por partes diferentes do programa ao mesmo tempo.

Exemplo de acesso:

```rust
unsafe {
    GLOBAL += 1;
}
```

## O que `unsafe` não faz

`unsafe` não desliga toda a segurança do Rust.

Ele apenas remove algumas proteções específicas, transferindo a responsabilidade para o programador naquele trecho de código.

## Boas práticas

- prefira `safe Rust` sempre que possível;
- use `unsafe` só quando houver necessidade real;
- isole o código inseguro no menor trecho possível;
- documente claramente por que ele é seguro naquele contexto;
- revise com mais cuidado qualquer código que use `unsafe`.

## Resumo

Use `unsafe` quando precisar fazer algo que o Rust não consegue garantir sozinho. Ele é poderoso, mas deve ser usado com muita cautela.
