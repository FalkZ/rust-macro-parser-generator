# Rust Macro Parser Generator

This is a Rust library for generating parsers from macro definitions.
The definitions are inspired by [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

## Example

An implemented toy language that compiles to TypeScript can be found here:

- [Grammar](src/m1n/grammar.rs)
- [Walker & Code Generator](src/m1n/renderer.rs)

## How it works

First we define some kind of grammar in the macro. In this case simple arithmetic expressions:

Lexer:

```rust
Lexer!(
    { // Defining all the tokens of the grammar
        {'0'..='9' =>} => NUMBER, // matching subsequent digits
        {'+'} => PLUS, // matching a single +
        {'-'} => MINUS,
        {'*'} => MUL,
        {'/'} => DIV,
        {' '} => SPACE,
        {'\n'} => NEWLINE
    }

    ( SPACE | NEWLINE ) => _ // Spaces and newlines are skipped when matching with the parser

    {
       // Remapping of token strings from above, not necessary in that case
    });


```

Parser:

```rust
Parser!(
    operator = (  PLUS | MINUS | DIV | MUL ),
    half_statement = [ #operator => operator, NUMBER, *],
    statement = { NUMBER => left, *half_statement => right},
);
```
