#[macro_export]
macro_rules! variant {
    ($exp:expr, $pat:pat_param) => {
        match $exp {
            $pat => Ok(extract_impl::extract_variant_assign!($pat)),
            _ => {
                let msg = "variant does not match".into();
                core::result::Result::<_, std::boxed::Box<dyn std::error::Error>>::Err(msg)
            }
        }
    };
    ($exp:expr, $pat:pat_param, $err:expr) => {
        match $exp {
            $pat => Ok(extract_impl::extract_variant_assign!($pat)),
            _ => Err($err),
        }
    };
    // `err` is callable with no parameters.
    ($exp:expr, $pat:pat_param, else $err:expr) => {
        match $exp {
            $pat => Ok(extract_impl::extract_variant_assign!($pat)),
            _ => Err($err()),
        }
    };
}
