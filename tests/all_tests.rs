#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple.rs");
    t.pass("tests/guard.rs");
    t.compile_fail("tests/or_pattern.rs")
}
