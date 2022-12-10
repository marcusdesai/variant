//! The [`try_variant`](`try_variant`) macro matches an expression against a
//! given pattern returning a [`Result`](`core::result::Result`). If the pattern
//! matches, then the [`Ok`](`core::result::Result::Ok`) branch is returned
//! including any assignments from the pattern (or [`unit`] if none are given).
//! If the match fails then [`Err`](`core::result::Result::Err`) is returned
//! with either a given error, or a default `Box<dyn std::error::Error>`
//! otherwise.
//!
//! The [`get_variant`](`get_variant`) macro works in exactly the same way,
//! except it returns [`Some`](`core::option::Option::Some`) if the pattern
//! matches and [`None`](`core::option::Option::None`), otherwise.
//!
//! ## Simple Example
//!
//! ```
//! use variant::{get_variant, try_variant};
//!
//! let val = Some((0, 1));
//! let res = try_variant!(val, Some((i, _))).expect("i");
//! assert_eq!(res, 0);
//!
//! let res = try_variant!(val, Some((10, j)));
//! assert!(res.is_err());
//!
//! // Using get_variant instead
//! let opt = get_variant!(val, Some((i, _)));
//! assert_eq!(opt, Some(0));
//!
//! let opt = get_variant!(val, Some((10, j)));
//! assert_eq!(opt, None);
//! ```
//!
//! ## Guards
//!
//! Conditional guards work the same as with [`matches!`][matches].
//!
//! ```
//! use variant::try_variant;
//!
//! struct Foo {
//!     a: usize,
//!     b: Option<bool>,
//! }
//!
//! let val = Foo { a: 20, b: None };
//! let res = try_variant!(val, Foo { a, .. } if a == 20).expect("a");
//! assert_eq!(res, 20);
//!
//! let res = try_variant!(val, Foo { b, .. } if b.is_some());
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
//! use variant::try_variant;
//!
//! let val = (Some(10), Some(true));
//! let (a, b) = try_variant!(val, (Some(b), Some(a))).expect("tuple");
//! assert_eq!((a, b), (true, 10));
//! ```
//!
//! ## Custom Errors
//!
//! ```
//! use variant::try_variant;
//!
//! #[derive(Debug)]
//! enum MyError {
//!     Bad,
//!     Worse,
//!     Expensive,
//! }
//!
//! let val = Some(1);
//! let res = try_variant!(val, Some(i), MyError::Bad).expect("i");
//! assert_eq!(res, 1);
//!
//! let res = try_variant!(val, Some(50), MyError::Worse);
//! assert!(matches!(res, Err(MyError::Worse)));
//!
//! // We can also use an error returning closure with the following syntax
//! let err_closure = || MyError::Expensive;
//! let res = try_variant!(val, Some(50), else err_closure);
//! assert!(matches!(res, Err(MyError::Expensive)));
//!
//! // Doesn't have to be a closure, any callable taking no parameters will do
//! fn make_err() -> MyError { MyError::Expensive }
//! let res = try_variant!(val, Some(50), else make_err);
//! assert!(matches!(res, Err(MyError::Expensive)));
//! ```
//!
//! ## Or Patterns
//!
//! Neither macro supports `Or` patterns at any level. This is because there is
//! no simple expected way to signal to the user what values are returned in
//! the case where only some assignments may match. If a pragmatic solution to
//! this problem is found then adding this feature in the future may be possible.
//!
//! [unit]: https://doc.rust-lang.org/std/primitive.unit.html
//! [matches]: https://doc.rust-lang.org/std/macro.matches.html

// Not public API.
#[doc(hidden)]
pub mod __private {
    // Ensures `extract_variant_assign` is callable regardless of where
    // `variant` is called.
    pub use extract_variant_internal::extract_variant_assign;
}

#[macro_export]
macro_rules! try_variant {
    ($expression:expr, $pattern:pat_param $(if $guard:expr)?) => {
        match $expression {
            $pattern $(if $guard)? => {
                core::result::Result::Ok($crate::__private::extract_variant_assign!($pattern))
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
                core::result::Result::Ok($crate::__private::extract_variant_assign!($pattern))
            }
            _ => core::result::Result::Err($err),
        }
    };
    // `err` is callable with no parameters.
    ($expression:expr, $pattern:pat_param $(if $guard:expr)?, else $err:expr) => {
        match $expression {
            $pattern $(if $guard)? => {
                core::result::Result::Ok($crate::__private::extract_variant_assign!($pattern))
            }
            _ => core::result::Result::Err($err()),
        }
    };
}

#[macro_export]
macro_rules! get_variant {
    ($expression:expr, $pattern:pat_param $(if $guard:expr)?) => {
        match $expression {
            $pattern $(if $guard)? => {
                core::option::Option::Some($crate::__private::extract_variant_assign!($pattern))
            }
            _ => core::option::Option::None,
        }
    };
}
