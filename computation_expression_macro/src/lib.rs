extern crate proc_macro;
use proc_macro::TokenStream;

mod expr;

/// macro to generate your own computation expression
/// recommendad way it copy pasting the implementation if you need something to work differently
/// You can take a look at `option!`, `result!` and `seq!` how this macro can be used directly
#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    expr::imp(input.into()).unwrap_or_else(|err| err.to_compile_error()).into()
}

/// simple macro to implement F#'s `let!`, and `return!` in `option` computation expression
/// usage:
/// ```rs
///     let x = option!(
///         let! x = Some(3);
///         let y = 7;
///         x + y
///     );
///     println!("{:?}", x); // Some(10)
/// ```
#[proc_macro]
pub fn option(input: TokenStream) -> TokenStream {
    expr::option(input.into()).unwrap_or_else(|err| err.to_compile_error()).into()
}


/// simple macro to implement F#'s `let!`, and `return!` in non standard, but still common `result` computation expression
/// usage:
/// ```rs
///     let x = result!(&str:
///         let! x = Ok(3);
///         let! y = Err("ups this will always fail");
///         return x + y;
///     );
///     println!("{:?}", x);
/// ```
#[proc_macro]
pub fn result(input: TokenStream) -> TokenStream {
    expr::result(input.into()).unwrap_or_else(|err| err.to_compile_error()).into()
}


/// simple macro to implement F#'s `yield!`, and `yield` in `seq` computation expression
/// usage:
/// ```rs
///     let s = seq!{u8:
///         yield! [1,2,3].into_iter();
///         yield 4;
///         yield 5;
///         yield! [6,7];
///     };
///     println!("1..7:");
///     for i in s {
///         println!("{}", i)
///     }
/// ```
#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    expr::seq(input.into()).unwrap_or_else(|err| err.to_compile_error()).into()
}
