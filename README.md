# Variant: Destructuring Macros

[![docs.rs](https://docs.rs/extract-variant/badge.svg)](https://docs.rs/extract-variant)
[![Crates.io](https://img.shields.io/crates/v/extract-variant.svg)](https://crates.io/crates/extract-variant)

This small crate exports two convenience macros. `try_variant!`, which can be used to destructure an expression into a single pattern, returning a result of assignments in the pattern. `get_variant!`, which works in the same way but returns an `Option`.  

Both macros work similarly to the [`matches!`][matches] macro from the rust std lib.

This macro is mainly useful for reducing matching boilerplate when you just want to try and extract some values from a pattern match (my reason for making this 😉).

## Example

```rust
use variant::try_variant;

let val = Some((0, 1));
let res = try_variant!(val, Some((i, _))).expect("i");
assert_eq!(res, 0);

let res = try_variant!(val, Some((10, j)));
assert!(res.is_err());
```

There are more examples on the [docs][docs] page

[matches]: https://doc.rust-lang.org/std/macro.matches.html
[docs]: https://docs.rs/extract-variant
