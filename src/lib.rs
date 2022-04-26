
extern crate computation_expression_macro;

pub use computation_expression_macro::{expr, option, result, seq};

use std::ops::ControlFlow;

#[cfg(test)]
mod tests;


/// example implementation used in
/// usage:
/// ```rs
///     let x = option!(
///         let! x = Some(3);
///         let y = 7;
///         x + y
///     );
///     println!("{:?}", x); // Some(10)
/// ```
#[derive(Default)]
pub struct OptionBuilder{}
impl OptionBuilder {
    pub fn bind<T>(&mut self, o: Option<T>) -> ControlFlow<Option<T>, T> {
        match o {
            None => ControlFlow::Break(None),
            Some(v) => ControlFlow::Continue(v),
        }
    }
    pub fn return_from<T>(&mut self, o: Option<T>) -> Option<T> {
        o
    }
    pub fn ret<T>(&mut self, o: T) -> Option<T> {
        Some(o)
    }
}

/// example implementation used in
/// usage:
/// ```rs
///     let x = result!(&str:
///         let! x = Ok(3);
///         let! y = Err("ups this will always fail");
///         return x + y;
///     );
///     println!("{:?}", x);
/// ```
#[derive(Default)]
pub struct ResultBuilder<ERR>{
    _err: std::marker::PhantomData<ERR>,
}
impl<ERR> ResultBuilder<ERR> {
    pub fn bind<OK>(&mut self, r: Result<OK, ERR>) -> ControlFlow<Result<OK, ERR>, OK> {
        match r {
            Err(e) => ControlFlow::Break(Err(e)),
            Ok(o) => ControlFlow::Continue(o),
        }
    }
    pub fn return_from<OK>(&mut self, r: Result<OK, ERR>) -> Result<OK, ERR> {
        r
    }
    pub fn ret<OK>(&mut self, ok: OK) -> Result<OK,ERR> {
        Ok(ok)
    }
}

/// example implementation used in
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
pub struct SeqBuilder<ITEM: 'static>{
    iter: Box<dyn Iterator<Item = ITEM>>,
}
impl<ITEM: 'static> Default for SeqBuilder<ITEM> {
    fn default() -> Self {
        Self{
            iter: Self::empty()
        }
    }
}
impl<ITEM: 'static> SeqBuilder<ITEM> {
    fn empty() -> Box<dyn Iterator<Item = ITEM>> {
        Box::new(std::iter::empty())
    }
    pub fn yield_it_from(&mut self, iter: impl Iterator<Item = ITEM> + 'static) {
        let mut old = Self::empty();
        std::mem::swap(&mut old, &mut self.iter);
        self.iter = Box::new(old.chain(iter.take_while(|_|true).into_iter()));
    }
    pub fn yield_it(&mut self, item: ITEM) {
        let mut old = Self::empty();
        std::mem::swap(&mut old, &mut self.iter);
        self.iter = Box::new(old.chain([item].into_iter()));
    }
}

impl<ITEM: 'static> Iterator for SeqBuilder<ITEM> {
    type Item = ITEM;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}


