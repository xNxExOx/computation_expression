
use computation_expression::*;

fn main() {
    println!("Rust macros can mimic F#'s `option` Computation Expression, and it works 😛");
    let x = option!(
        let! x = Some(3);
        let y = 7;
        // I do not even need the forced return syntax 😛
        x + y
    );
    println!("{:?}", x);

    let x = option!(
        for i in 0..100u32 {
            if i == 42 {
                // I guess I beaten F# by being able to return from if branch 😛
                return i;
            }
        }
        let! x = Some(1);
        let mut y = 0;
        for i in 0..10 {
            y += i;
        }
        // I can go crazy
        let! z = {
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
        };
        return! Some(x + y + z);
    );
    println!("{:?}", x);
}
