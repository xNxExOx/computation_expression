
use computation_expression::*;

fn main() {
    println!("Rust macros can mimic F#'s (non standard) `result` Computation Expression, and it works");
    println!("but you need to specify the error type");
    let x = result!(():
        let! x = Ok(3);
        let y = 7;
        return! Ok(x + y);
    );
    println!("{:?}", x);

    let x = result!(&str:
        let! x = Ok(3);
        let! y = Err("ups this will always fail");
        return x + y;
    );
    println!("{:?}", x);

    let x = result!(u8:
        let! x = Ok(3);
        let! y = Err(42);
        return x + y;
    );
    println!("{:?}", x);
}
