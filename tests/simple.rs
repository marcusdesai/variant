use variant::variant;

enum Test {
    A(u8),
    B(bool),
}

fn main() {
    let val_a = Test::A(1);
    let a = variant!(val_a, Test::A(a)).expect("a");
    assert_eq!(a, 1);

    let val_b = Test::B(true);
    let b = variant!(val_b, Test::B(b)).expect("b");
    assert_eq!(b, true);
}
