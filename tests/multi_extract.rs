use variant::variant;

struct Foo {
    a: Bar,
    b: Option<i8>,
}

enum Bar {
    A(u8),
    B(bool),
}

fn main() {
    let val = Foo { a: Bar::B(false), b: Some(-5) };
    let (i, j) = variant!(val, Foo { a: Bar::B(i), b: Some(j) }).expect("(i, j)");
    assert_eq!(i, false);
    assert_eq!(j, -5);

    let res = variant!(val, Foo { a: Bar::A(i), b: Some(j) });
    assert!(res.is_err());

    let _: () = variant!(val, Foo { a: Bar::B(_), .. }).expect("unit");
}
