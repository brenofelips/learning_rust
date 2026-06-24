# Estudos de Rust

Repositório de anotações e exemplos práticos criados durante meus estudos de **Rust**
(cursos da Alura + material complementar). O objetivo é servir como uma base de
conhecimento pessoal: cada conceito da linguagem é explicado em um arquivo de
documentação (`docs/`) e demonstrado com código executável (`resources/`).

## 🎯 Objetivo

Aprender os fundamentos de Rust de forma incremental, combinando **teoria** (resumos em
Markdown para consulta rápida) com **prática** (funções pequenas e comentadas que rodam
a partir de um único `main.rs`).

## 📁 Estrutura do projeto

```
.
├── main.rs                  # Ponto de entrada: importa e executa cada exemplo
├── resources/               # Código Rust por tema
│   ├── repeticoes.rs        # while, loop, for e match
│   ├── escopo.rs            # variáveis, tipos primitivos, const e static
│   ├── sombra.rs            # shadowing e escopo de variáveis
│   ├── ownership.rs         # ownership, borrowing e referências (mutáveis/imutáveis)
│   ├── soma.rs              # função com parâmetros e retorno
│   └── condicionais.rs      # if / else if / else e if como expressão
├── docs/                    # Anotações teóricas (resumos de estudo)
│   ├── rust_base_de_conhecimento.md   # Resumo dos fundamentos da linguagem
│   ├── rust_recursos_avancados.md     # Generics, traits, lifetimes, async, etc.
│   ├── repeticoes.md        # Explicação detalhada dos laços
│   ├── const.md             # let, const e static
│   └── unsafe.md            # O que é unsafe e quando usar
└── youtube/
    └── yt.md                # Links de vídeos complementares (gestão de memória)
```

## 📚 Conteúdo abordado

**Fundamentos** (`docs/rust_base_de_conhecimento.md`):
variáveis e mutabilidade, funções, tipos primitivos e não primitivos, ownership,
borrowing, slices, escopo, `Option` e `Result`.

**Exemplos executáveis** (`resources/`):

| Tema | Arquivo | O que demonstra |
|------|---------|-----------------|
| Repetições | `repeticoes.rs` | Tabuada com `while`, `loop` e `for`, mais `match` |
| Escopo e tipos | `escopo.rs` | `let`, `mut`, tipos primitivos, `const` e `static` |
| Shadowing | `sombra.rs` | Reutilização de nomes e escopo de blocos |
| Ownership | `ownership.rs` | Transferência vs. empréstimo de posse |
| Funções | `soma.rs` | Parâmetros tipados e valor de retorno |
| Condicionais | `condicionais.rs` | `if`/`else` e `if` como expressão |

**Recursos avançados** (`docs/rust_recursos_avancados.md`):
generics, traits, lifetimes, closures, iterators, pattern matching, tratamento de erros,
smart pointers, módulos/crates, Cargo, concorrência, macros, `async`/`await`, `unsafe` e FFI.

## ▶️ Como executar

O projeto usa apenas o compilador (`rustc`), sem Cargo. Cada módulo é carregado em
`main.rs` via o atributo `#[path = "..."]`.

```bash
# Compilar
rustc main.rs

# Executar o binário gerado
./main
```

> A função `ownership::ownership_error` em `resources/ownership.rs` é um exemplo
> **intencional** de erro de compilação (uso de valor após mover a posse) e por isso
> não é chamada em `main.rs`.

## 📝 Observações

- As anotações estão em português e servem como material de revisão.
- O foco é didático: os exemplos são pequenos e fortemente comentados.
