use std::error::Error;
use variant::variant;

struct Test {
    a: u8,
    b: Option<u8>,
}

fn main() {
    let val = Test { a: 0, b: None };
    let a = variant!(val, Test { a, b: None } if a < 100).expect("a");
    assert_eq!(a, 0);

    let err = variant!(val, Test { b: Some(num), .. } if num > 0);
    assert!(err.is_err());

    let val = Test { a: 10, b: Some(90) };
    let (a, b) = variant!(val, Test { a, b: Some(b) } if a != b).expect("b");
    assert_eq!(a, 10);
    assert_eq!(b, 90);

    let err_closure = || Box::<dyn Error>::from("err");
    let err = variant!(val, Test { a, b } if b.is_none(), else err_closure);
    assert!(err.is_err());
}
