
use variant::variant;

struct S {
    a: i8,
    b: bool,
}

enum Test {
    A(u8),
    B(S),
}

fn work() {
    struct BB {
        v: Test
    }

    let val = BB { v: Test::A(1) };
    variant!(val.v, Test::A(v));
}
