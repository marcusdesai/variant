use variant::variant;

struct Test {
    a: u8,
    b: Option<u8>,
}

fn main() {
    let val = Test { a: 1, b: Some(2) };
    let _ = variant!(val, Test { a, b: Some(1) | Some(2) });
}
