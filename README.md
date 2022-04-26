# POC F#'s computation expression in Rust
Simple proof of concept crate to show that F#'s computation expressions can be done in Rust,
including the `let!`/`return!`/`yield!` syntax

## Why it is just Proof of concept, and **NOT** production ready?
- `computation_expression_macro::expr::rustify_input` this is just first reason why all the line information are instantly dropped
- many other places drop line information too, by just reformatting whatever was there
- So any mistake you do will have useless error pointing to whole macro
- Some edge cases are not handled

## What can it do already
- `option!(..)` macro provides superior functionality to F# OptionBuilder, because it works seamlessly with if statements
- `result!(ERROR_TYPE:..)` unfortunately can not infer whole type, so the error needs to be provided
- `seq!(ITEM_TYPE:..)` unfortunately can not infer the item type

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
computation_expression = { path="Whereever you placed it" }
```
Because I decided to not publish it on crates.io (yet) you need to use path for now.