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
computation_expression = { git = "https://github.com/xNxExOx/computation_expression" }
```
Because I decided to not publish it on crates.io (yet) you need to use git dependency for now.
```rs
    let x = option!(
        let! x = Some(3);
        let y = 7;
        x + y
    ); // x == Some(10)
```

## Examples
I have examples for all of the above, and few extra:
- `try` unfortunately not yet stable, but once it will stabilize, making equivalent of any computation expression,
that uses only `let!`, and `return!` will be super easy by implementing `Try` trait, which is how it now works for
`Option`, and `Result`
- `closure` this example show again how to replace `option`, `result`,  and `lazy` computation expression,
but without any macro
