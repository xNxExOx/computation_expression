
use computation_expression::*;

fn main(){
    println!("Rust macros can mimic F#'s `seq` Computation Expression, and it works 😛");
    println!("but you need to specify the item type :(");
    let s = seq!{u8:
        yield! [1,2,3].into_iter();
        yield 4;
        yield 5;
        yield! [6,7];
    };
    println!("1..7:");
    for i in s {
        println!("{}", i)
    }
    println!();

    let s1 = seq!{u8:
        yield! [1,2,3].into_iter();
        yield 4;
        yield 5;
        yield! [6,7];
    };
    let s = seq!{(i16, f32):
        for i in s1 {
            yield (i as i16, i as f32 * std::f32::consts::PI);
        }
    };
    println!("1..7 and multiples of PI");
    for (i, f) in s {
        println!("{} {}", i, f)
    }
}
