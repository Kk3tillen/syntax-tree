# 🌳 Syntax Tree 

This project implements an **Abstract Syntax Tree** to represent simple mathematical expressions in Rust.

## The supported operations are:
*   **`Addition`** (`+`)
*   **`Subtraction`** (`-`)
*   **`Multiplication`** (`*`)
*   **`Division`** (`/`)
*   **`Remainder`** (`%`)
*   **`Negation`** (unary `-`)

## Installation

Make sure you have Rust installed. Then clone and build:

```bash
git clone https://github.com/Kk3tillen/syntax-tree.git
cd lexical-analyzer
```

## Usage

### Running the Analyzer

```bash
cargo run
```

### Interaction Example

```
=== Calculadora de Expressões ===
Digite uma expressão matemática (ou 'sair' para encerrar)
Exemplos: 10 + 20, (10 + 20) * 30

Expressão: (10 + 5) * -2

Expressão simplificada:
(10 + 5) * -2

Árvore sintática:
*
├ +
│ ├ 10
│ └ 5
└ -
  └ 2

Resultado: -30

Expressão: 10 / 0

Expressão simplificada:
10 / 0

Árvore sintática:
/
├ 10
└ 0

Resultado: none

Expressão: sair
```
## Author

| [![kézia ketillen santos lima](https://avatars3.githubusercontent.com/u/88369589?s=100&v=4)](https://github.com/kk3tillen) |
| :---: |
| **kézia ketillen santos lima** |
---
