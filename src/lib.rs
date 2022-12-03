#[macro_export]
macro_rules! variant {
    ($expression:expr, $pattern:pat_param $(if $guard: expr)?) => {
        match $expression {
            $pattern $(if $guard)? => Ok(extract_impl::extract_variant_assign!($pattern)),
            _ => {
                let msg = "pattern does not match, or guard not satisfied".into();
                core::result::Result::<_, std::boxed::Box<dyn std::error::Error>>::Err(msg)
            }
        }
    };
    ($expression:expr, $pattern:pat_param $(if $guard: expr)?, $err:expr) => {
        match $expression {
            $pattern $(if $guard)? => Ok(extract_impl::extract_variant_assign!($pattern)),
            _ => Err($err),
        }
    };
    // `err` is callable with no parameters.
    ($expression:expr, $pattern:pat_param $(if $guard: expr)?, else $err:expr) => {
        match $expression {
            $pattern $(if $guard)? => Ok(extract_impl::extract_variant_assign!($pattern)),
            _ => Err($err()),
        }
    };
}
