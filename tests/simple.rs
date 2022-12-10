use std::error::Error;
use variant::try_variant;

enum Test {
    A(u8),
    B(bool),
}

fn main() {
    let val_a = Test::A(1);
    let a = try_variant!(val_a, Test::A(a)).expect("a");
    assert_eq!(a, 1);

    let a_err = try_variant!(val_a, Test::B(_));
    assert!(a_err.is_err());

    let val_b = Test::B(true);
    let err = Box::<dyn Error>::from("err");
    let b = try_variant!(val_b, Test::B(b), err).expect("b");
    assert_eq!(b, true);

    let err_closure = || Box::<dyn Error>::from("err");
    let b_err = try_variant!(val_b, Test::A(_), else err_closure);
    assert!(b_err.is_err());
}
