
// as mentioned in try example it works for `Option<T>` and `Result<Ok,Err>`
// because these two types have `Try` implemented in std, but on nightly you can implement it for any type
// and after it will reach stable this will work with any type that implement `Try` trait
fn main() {
    println!("Rust' closure mimic F#'s `option` and `result` Computation Expression as well");
    println!("and it works 😛 and on STABLE Rust 😛");
    let x = {|| {
        let x = Some(3)?;
        let y = 7;
        return Some(x + y);
    }}();
    println!("{:?}", x);

    let x = {|| {
        for i in 0..100u32 {
            if i == 42 {
                return Some(i);
            }
        }
        let x = Some(1)?;
        let mut y = 0;
        for i in 0..10 {
            y += i;
        }
        let z = {
            fn fact(f: u32) -> u32 {
                if f <= 1 {
                    1
                } else {
                    fact(f - 1) + f
                }
            }
            let a = fact(1);
            let b = fact(5);
            let c = fact(6);
            Some(a * b * c)
        }?;
        return Some(x + y + z);
    }}();
    println!("{:?}", x);

    let x = {|| -> Result<i32,()> {
        let x: Result<i32, ()> = Ok(3);
        let x = x?;
        let y = 7;
        return Ok(x + y);
    }}();
    println!("{:?}", x);

    let x = {|| -> Result<i32,&str> {
        let x = Ok(3)?;
        let y : Result<i32,&str> = Err("ups this will always fail");
        let y = y?;
        return Ok(x + y);
    }}();
    println!("{:?}", x);

    let x = {|| -> Result<i32,i32> {
        let x: Result<i32, i32> = Ok(3);
        let x = x?;
        let y : Result<i32,i32> = Err(42);
        let y = y?;
        return Ok(x + y);
    }}();
    println!("{:?}", x);
}
