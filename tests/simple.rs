use std::error::Error;
use variant::variant;

enum Test {
    A(u8),
    B(bool),
}

fn main() {
    let val_a = Test::A(1);
    let a = variant!(val_a, Test::A(a)).expect("a");
    assert_eq!(a, 1);

    let a_err = variant!(val_a, Test::B(_));
    assert!(a_err.is_err());

    let val_b = Test::B(true);
    let b = variant!(val_b, Test::B(b)).expect("b");
    assert_eq!(b, true);

    let err_closure = || Box::<dyn Error>::from("err");
    let b_err = variant!(val_b, Test::A(_), else err_closure);
    assert!(b_err.is_err());
}
