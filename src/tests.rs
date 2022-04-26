
use crate as computation_expression;
use computation_expression::*;

#[test]
fn test_option_some(){
    let x = option!(
        let! x = Some(3);
        let y = 7;
        return x + y;
    );
    assert_eq!(x, Some(10))
}

#[test]
fn test_option_from_ouside_trough_fn(){
    let outer = 42;
    let get_the_option = || -> Option<u32> {
        Some(outer)
    };
    let x = option!(
        let! x = get_the_option();
        let y = 7;
        return x + y;
    );
    assert_eq!(x, Some(49))
}

#[test]
fn test_option_none(){
    let x = option!(
        let! x = Some(3);
        let! y = None;
        return x + y;
    );
    assert_eq!(x, None)
}

#[test]
fn test_result_ok(){
    let x = result!(():
        let! x = Ok(3);
        let y = 7;
        return! Ok(x + y);
    );
    assert_eq!(x, Ok(10))
}

#[test]
fn test_result_err(){
    const ERROR_MESSAGE : &str = "ups this will always fail";
    let x = result!(&str:
        let! x = Ok(3);
        let! y = Err(ERROR_MESSAGE);
        return! Ok(x + y);
    );
    assert_eq!(x, Err(ERROR_MESSAGE))
}

#[test]
fn test_result_option_combined(){
    let x = result!(():
        let! x = Ok(30);
        let y = option!(
            let! x = Some(5);
            let y = 7;
            return x + y;
        );
        let! y = match y {
            None => Err(()),
            Some(y) => Ok(y),
        };
        return! Ok(x + y);
    );
    assert_eq!(x, Ok(42))
}

#[test]
fn test_seq(){
    let mut x = seq!(i32:
        yield! [1,2,3].into_iter();
        let x = 3;
        let mut y = 1;
        yield x + y;
        y += 1;
        yield x + y;
        y += 1;
        yield! [x+y,x+y+1];
    );
    assert_eq!(x.next(), Some(1));
    assert_eq!(x.next(), Some(2));
    assert_eq!(x.next(), Some(3));
    assert_eq!(x.next(), Some(4));
    assert_eq!(x.next(), Some(5));
    assert_eq!(x.next(), Some(6));
    assert_eq!(x.next(), Some(7));
    assert_eq!(x.next(), None);
    assert_eq!(x.next(), None);
}

#[test]
fn test_lazy_seq(){
    let mut x = seq!(u128:
        yield! (0u128..).into_iter();
    );
    const CHECK_FIRST_N : usize = 1_000;
    for i in 0..CHECK_FIRST_N {
        assert_eq!(x.next(), Some(i as u128));
    }

    let mut x = seq!(f64:
        yield! x.take(10).map(|i| i as f64);
        yield std::f64::consts::PI;
    );
    for i in CHECK_FIRST_N..CHECK_FIRST_N+10 {
        assert_eq!(x.next(), Some(i as f64));
    }
    assert_eq!(x.next(), Some(std::f64::consts::PI));
    assert_eq!(x.next(), None);
}
