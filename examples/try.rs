#![feature(try_blocks)]

fn main() {
    // UNSTABLE: https://github.com/rust-lang/rust/issues/31436
    // github have 10 of 11 tasks done, and once it is stable you can implement it for any type and use like this
    // it is merged: https://github.com/rust-lang/rfcs/pull/1859 so you can try it at least on nightly
    // you can check my example: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=56824f6a65c26b488f426140d050a6ef
    println!("Rust's `try` (which is still unstable) can do most of option and result too 😛");
    let x : Option<i32> = try {
        let x = Some(3)?;
        let y = 7;
        x + y
    };
    println!("{:?}", x);

    let x : Result<i32, ()> = try {
        let x = Ok(3)?;
        let y = 7;
        Ok(x + y)?
    };
    println!("{:?}", x);

    let x : Result<i32, &str> = try {
        let x = Ok(3)?;
        let y : i32 = Err("ups this will always fail")?;
        x + y
    };
    println!("{:?}", x);
}
