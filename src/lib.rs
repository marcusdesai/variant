//! The [`variant`](`variant`) macro matches an expression against a given
//! pattern returning a [`Result`](`core::result::Result`). If the pattern
//! matches, then the [`Ok`](`core::result::Result::Ok`) branch is returned
//! including any assignments from the pattern (or [`unit`] if none are given).
//! If the match fails then [`Err`](`core::result::Result::Err`) is returned
//! with either a given error, or defaulting to `Box<dyn std::error::Error>`
//! otherwise.
//!
//! ## Simple Example
//!
//! ```
//! use variant::variant;
//!
//! let val = Some((0, 1));
//! let res = variant!(val, Some((i, _))).expect("i");
//! assert_eq!(res, 0);
//!
//! let res = variant!(val, Some((10, j)));
//! assert!(res.is_err());
//! ```
//!
//! ## Guards
//!
//! Conditional guards work the same as with [`matches!`][matches].
//!
//! ```
//! use variant::variant;
//!
//! struct Foo {
//!     a: usize,
//!     b: Option<bool>,
//! }
//!
//! let val = Foo { a: 20, b: None };
//! let res = variant!(val, Foo { a, .. } if a == 20).expect("a");
//! assert_eq!(res, 20);
//!
//! let res = variant!(val, Foo { b, .. } if b.is_some());
//! assert!(res.is_err());
//! ```
//!
//! ## Multiple Assignments
//!
//! When there is more than one assignment within a matching pattern all
//! assignments are returned in a tuple. Since assignments in a pattern may not
//! be ordered linearly, multiple assignments will be returned in lexicographic
//! order.
//!
//! ```
//! use variant::variant;
//!
//! let val = (Some(10), Some(true));
//! let (a, b) = variant!(val, (Some(b), Some(a))).expect("tuple");
//! assert_eq!((a, b), (true, 10));
//! ```
//!
//! ## Custom Errors
//!
//! ```
//! use variant::variant;
//!
//! #[derive(Debug)]
//! enum MyError {
//!     Bad,
//!     Worse,
//!     Expensive,
//! }
//!
//! let val = Some(1);
//! let res = variant!(val, Some(i), MyError::Bad).expect("i");
//! assert_eq!(res, 1);
//!
//! let res = variant!(val, Some(50), MyError::Worse);
//! assert!(matches!(res, Err(MyError::Worse)));
//!
//! // We can also use an error returning closure with the following syntax
//! let err_closure = || MyError::Expensive;
//! let res = variant!(val, Some(50), else err_closure);
//! assert!(matches!(res, Err(MyError::Expensive)));
//! ```
//!
//! ## Or Patterns
//!
//! The [`variant`](`variant`) macro does not support `Or` patterns at any
//! level. This is because there is no simple expected way to signal to the
//! user what values are returned in the case where only some assignments may
//! match. If a pragmatic solution to this problem is found then adding this
//! feature in the future may be possible.
//!
//! [unit]: https://doc.rust-lang.org/std/primitive.unit.html
//! [matches]: https://doc.rust-lang.org/std/macro.matches.html

#[macro_export]
macro_rules! variant {
    ($expression:expr, $pattern:pat_param $(if $guard:expr)?) => {
        match $expression {
            $pattern $(if $guard)? => {
                core::result::Result::Ok(extract_variant_internal::extract_variant_assign!($pattern))
            }
            _ => {
                let msg = "pattern does not match, or guard not satisfied".into();
                core::result::Result::<_, std::boxed::Box<dyn std::error::Error>>::Err(msg)
            }
        }
    };
    ($expression:expr, $pattern:pat_param $(if $guard:expr)?, $err:expr) => {
        match $expression {
            $pattern $(if $guard)? => {
                core::result::Result::Ok(extract_variant_internal::extract_variant_assign!($pattern))
            }
            _ => core::result::Result::Err($err),
        }
    };
    // `err` is callable with no parameters.
    ($expression:expr, $pattern:pat_param $(if $guard:expr)?, else $err:expr) => {
        match $expression {
            $pattern $(if $guard)? => {
                core::result::Result::Ok(extract_variant_internal::extract_variant_assign!($pattern))
            }
            _ => core::result::Result::Err($err()),
        }
    };
}
