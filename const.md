# Constantes, variáveis imutáveis e `static` em Rust

Este arquivo resume os conceitos apresentados sobre variáveis imutáveis, constantes e variáveis estáticas em Rust.

## Variáveis imutáveis com `let`

Pense em uma variável imutável como uma caixa com um rótulo. Você coloca algo dentro dela, por exemplo, `300 balões`. Depois de colocar os balões, você fecha a caixa e cola um aviso grande dizendo que o conteúdo é fixo.

### Ideia principal

- a caixa representa o endereço de memória;
- o conteúdo representa o valor;
- depois de criada, a variável não pode ser alterada.

Exemplo:

```rust
let baloes = 300;
```

Se você precisar de outro valor, crie outra variável.

## Constantes com `const`

Agora pense em uma receita de bolo que sempre pede `3.14` xícaras de farinha. Esse valor não muda.

### Ideia principal

- a constante representa um valor fixo;
- ela é resolvida em tempo de compilação;
- o valor é substituído diretamente no código quando necessário;
- não há uma “caixa” separada sendo consultada em tempo de execução.

Exemplo:

```rust
const PI: f64 = 3.14;
```

### Resumo

O compilador vê `const PI = 3.14;` e trata esse valor como algo fixo no programa final.

## Variáveis estáticas com `static`

Agora pense em algo global, como um placar que existe durante toda a festa.

### Ideia principal

- a variável estática tem um endereço fixo;
- existe durante toda a execução do programa;
- pode ser acessada de forma global;
- precisa de tipo definido.

Exemplo:

```rust
static PONTOS_GLOBAIS: u32 = 0;
```

### `static mut` e `unsafe`

Se a variável estática for mutável, o Rust exige cuidado extra.

Variáveis globais mutáveis podem gerar problemas quando mais de uma parte do programa tenta alterá-las ao mesmo tempo.

Exemplo:

```rust
static mut GLOBAL: u32 = 0;
```

Para acessar ou modificar esse tipo de valor, normalmente você precisa de `unsafe`.

```rust
unsafe {
    GLOBAL += 1;
}
```

## Resumo final

- `let`: cria uma variável, que por padrão é imutável;
- `const`: define um valor fixo em tempo de compilação;
- `static`: define um valor com vida global durante o programa;
- `static mut`: permite mutação, mas exige `unsafe`.

## Observação

Use `const` sempre que o valor for realmente fixo. Use `static` apenas quando fizer sentido manter um valor global durante toda a execução do programa.
