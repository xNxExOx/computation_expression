use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, Block, Stmt, Expr, Attribute, Lifetime, Ident, ExprBreak, ExprForLoop, ExprIf, ExprWhile, ExprLoop, ExprUnsafe, Type, Token};
use syn::Result;
use syn::spanned::Spanned;

pub (crate) fn option(input: TokenStream) -> Result<TokenStream> {
    let imp = imp(quote!(ob{#input}))?;
    Ok(quote!({
        let mut ob = computation_expression::OptionBuilder::default();
        #imp
    }))
}

pub (crate) fn result(input: TokenStream) -> Result<TokenStream> {
    let TypedBuilder { generic_type, block} = syn::parse2(input)?;
    //panic!("{}", block.to_string());
    let imp = imp(quote!(rb {#block}))?;
    Ok(quote!({
        let mut rb = computation_expression::ResultBuilder::<#generic_type>::default();
        #imp
    }))
}

pub (crate) fn seq(input: TokenStream) -> Result<TokenStream> {
    let TypedBuilder { generic_type, block} = syn::parse2(input)?;
    //panic!("{}", block.to_string());
    let imp = imp(quote!(sb {#block break sb.into_iter();}))?;
    Ok(quote!({
        let mut sb = computation_expression::SeqBuilder::<#generic_type>::default();
        #imp
    }))
}

const BIND_ATTR : &str = "bind";
const RETURN_ATTR : &str = "ret";
const YIELD_ATTR : &str = "unroll";

static COUNTER : std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// This function convers the F#'s syntax of `let!` to rust friendly attributes
/// **BUT** it lose all the line information in the process
/// for production use this should be rewritten to not use string, but directly manipulate the stream
/// to create the attributes, and remove the `!`
fn rustify_input(input: TokenStream) -> Result<TokenStream> {
    Ok(input.to_string()
         .replace("let!", "#[bind]let")
         .replace("return!", "#[ret]return")
         .replace("yield!", "#[unroll]yield")
         .parse()?)
}

pub(crate) fn imp(input: TokenStream) -> Result<TokenStream> {
    let input = rustify_input(input)?;
    //panic!("{}", input.to_string());
    let ExprBuilder{builder, mut block} = syn::parse2(input)?;
    let span = block.stmts[0].span();
    let line_column = span.start();
    let span_name = format!("'__loop_line_{}_column_{}__{}", line_column.line, line_column.column, COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
    let lifetime = Lifetime::new(&span_name, span);

    let mut ret = quote!();

    let loop_break = quote!(break #lifetime);
    let last = block.stmts.remove(block.stmts.len()-1);
    for stmt in block.stmts {
        //panic!("{:?}", stmt);
        ret.extend(stmt_to_expr(stmt, &builder, &lifetime, &loop_break)?);
    }
    let last = match last {
        Stmt::Semi(Expr::Break(b), _) |
        Stmt::Expr(Expr::Break(b)) => {
            let semi = syn::parse2(quote!(;))?;
            Stmt::Semi(Expr::Break(b), semi).into_token_stream()
        }
        Stmt::Semi(Expr::Return(r), s) => {
            let stmt = Stmt::Semi(Expr::Return(r), s);
            stmt_to_expr(stmt, &builder, &lifetime, &loop_break)?
        }
        Stmt::Expr(expr) => {
            let break_token = syn::token::Break(expr.span());
            let expr : Expr = syn::parse2(quote!(#builder.ret(#expr)))?;
            Expr::Break(ExprBreak{
                attrs:vec![],
                break_token,
                label: Some(lifetime.clone()),
                expr: Some(Box::new(expr)),
            }).into_token_stream()
        }
        other => {
            let other = other.into_token_stream().to_string();
            let error =
                format!("unexpected expresion at the end of block, did you forget `return`? expresion: `{:?}`", other);
            quote!( compile_error!(#error) )
        }
    };
    ret.extend(last);
    let ret = quote!(#lifetime: loop{ #ret  });
    //panic!("{}", ret.to_string());
    Ok(ret)
}

fn stmt_to_expr(stmt:Stmt, builder: &Ident, lifetime: &Lifetime, loop_break: &TokenStream) -> Result<TokenStream> {
    Ok(match stmt {
        Stmt::Local(l) => {
            if let Some(attrs) = without_attr(&l.attrs, BIND_ATTR) {

                let pat = l.pat;
                let expr = l.init.map(|(_, e)| e);
                quote!(
                        #(#attrs)*
                        let #pat = match #builder.bind(#expr) {
                            std::ops::ControlFlow::Break(b) => #loop_break b,
                            std::ops::ControlFlow::Continue(c) => c,
                        };
                    )
            } else {
                Stmt::Local(l).to_token_stream()
            }
        }
        Stmt::Semi(expr, _) |
        Stmt::Expr(expr) => {
            let expr = erxpr_to_expr(expr, builder, lifetime, loop_break)?;
            let expr : Expr = syn::parse2(quote!(#expr))?;
            let semi = quote!(;);
            Stmt::Semi(expr, syn::token::Semi(semi.span())).into_token_stream()
        }
        _ => stmt.into_token_stream(),
    })
}

fn erxpr_to_expr(expr:Expr, builder: &Ident, lifetime: &Lifetime, loop_break: &TokenStream) -> Result<TokenStream> {
    Ok(match expr {
        Expr::Return(r) => {
            let break_token = syn::token::Break(r.span());
            let expr = r.expr;
            let (attrs, expr) =
                if let Some(attrs) = without_attr(&r.attrs, RETURN_ATTR)
                {
                    let expr : Expr = syn::parse2(quote!(#builder.return_from(#expr)))?;
                    (attrs, expr)
                } else {
                    let expr : Expr = syn::parse2(quote!(#builder.ret(#expr)))?;
                    (r.attrs, expr)
                };
            Expr::Break(ExprBreak{
                attrs,
                break_token,
                label: Some(lifetime.clone()),
                expr: Some(Box::new(expr)),
            }).into_token_stream()
        }
        Expr::If(expr_if) => {
            if_to_expr(expr_if, builder, lifetime, loop_break)?
        }
        Expr::Unsafe(u) => {
            let new_block = block_to_expr(u.block.stmts, builder, lifetime, loop_break)?;
            let b = syn::parse2(quote!({#new_block}))?;
            Expr::Unsafe(ExprUnsafe{block: b, ..u}).into_token_stream()
        }
        Expr::Loop(l) => {
            let new_block = block_to_expr(l.body.stmts, builder, lifetime, loop_break)?;
            let b = syn::parse2(quote!({#new_block}))?;
            Expr::Loop(ExprLoop{body: b, ..l}).into_token_stream()
        }
        Expr::While(w) => {
            let new_block = block_to_expr(w.body.stmts, builder, lifetime, loop_break)?;
            let b = syn::parse2(quote!({#new_block}))?;
            Expr::While(ExprWhile{body: b, ..w}).into_token_stream()
        }
        Expr::ForLoop(f) => {
            let new_block = block_to_expr(f.body.stmts, builder, lifetime, loop_break)?;
            let b = syn::parse2(quote!({#new_block}))?;
            Expr::ForLoop(ExprForLoop{body: b, ..f}).into_token_stream()
        }
        Expr::Yield(y) => {
            let expr = y.expr;
            if let Some(_attrs) = without_attr(&y.attrs, YIELD_ATTR)
            {
                quote!(#builder.yield_it_from(#expr.into_iter()))
            } else {
                quote!(#builder.yield_it(#expr))
            }.into_token_stream()
        }
        _ => expr.to_token_stream()
    })
}

fn if_to_expr(expr_if:ExprIf, builder: &Ident, lifetime: &Lifetime, loop_break: &TokenStream) -> Result<TokenStream> {
    let then_branch = {
        let then_branch = block_to_expr(expr_if.then_branch.stmts, builder, lifetime, loop_break)?;
        syn::parse2(quote!({#then_branch}))?
    };
    let else_branch = match expr_if.else_branch {
        None => None,
        Some((else_kw, else_branch)) => {
            let else_branch = *else_branch;
            match else_branch {
                Expr::Block(b) => {
                    let attrs = b.attrs;
                    let else_branch = block_to_expr(b.block.stmts, builder, lifetime, loop_break)?;
                    let else_branch = Box::new(syn::parse2(quote!(#(#attrs)* #else_branch))?);
                    Some((else_kw, else_branch))
                }
                _ => unreachable!("Expected all else branches to be of type block, but this one is: {:?}", else_branch)
            }
        }
    };
    Ok(Stmt::Expr(Expr::If(ExprIf{then_branch, else_branch, ..expr_if})).into_token_stream())
}

fn block_to_expr(stmts:Vec<Stmt>, builder: &Ident, lifetime: &Lifetime, loop_break: &TokenStream) -> Result<TokenStream> {
    let mut ret = quote!();
    for stmt in stmts {
        ret.extend(stmt_to_expr(stmt, builder, lifetime, loop_break)?);
    }
    Ok(ret)
}

fn without_attr(attrs: &Vec<Attribute>, attr_to_remove: &'static str) -> Option<Vec<Attribute>> {
    match attrs.iter().enumerate()
               .find(|(_, a)| a.path.is_ident(attr_to_remove))
               .map(|(i,_)| i) {
        None => None,
        Some(i) => {
            let mut attrs = attrs.clone();
            attrs.remove(i);
            Some(attrs)
        }
    }
}

struct ExprBuilder {
    builder: Ident,
    block: Block,
}

impl Parse for ExprBuilder {
    fn parse(input: ParseStream) -> Result<Self> {
        let builder = input.parse()?;
        let block = input.parse()?;
        Ok(Self{builder, block})
    }
}


struct TypedBuilder {
    generic_type: Type,
    block: TokenStream,
}

impl Parse for TypedBuilder {
    fn parse(input: ParseStream) -> Result<Self> {
        let err = input.parse()?;
        let _colon : Token!(:) = input.parse()?;
        let block = input.parse::<TokenStream>()?;
        Ok(Self{ generic_type: err, block})
    }
}
