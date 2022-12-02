#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/multiple_args.rs");
}
