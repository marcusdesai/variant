use std::error::Error;
use variant::{try_variant, get_variant};

struct Test {
    a: u8,
    b: Option<u8>,
}

fn main() {
    let val = Test { a: 0, b: None };
    let res = try_variant!(val, Test { a, b: None } if a < 100).expect("a");
    assert_eq!(res, 0);
    let res = get_variant!(val, Test { a, b: None } if a < 100).unwrap();
    assert_eq!(res, 0);

    let err = try_variant!(val, Test { b: Some(num), .. } if num > 0);
    assert!(err.is_err());
    let none = get_variant!(val, Test { b: Some(num), .. } if num > 0);
    assert_eq!(none, None);

    let val = Test { a: 10, b: Some(90) };
    let (a, b) = try_variant!(val, Test { a, b: Some(b) } if a != b).expect("b");
    assert_eq!(a, 10);
    assert_eq!(b, 90);

    let err_closure = || Box::<dyn Error>::from("err");
    let err = try_variant!(val, Test { a, b } if b.is_none(), else err_closure);
    assert!(err.is_err());
}
