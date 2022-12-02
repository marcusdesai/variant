
#[macro_export]
macro_rules! variant {
    ($exp:expr, $pat:pat_param) => {
        match $exp {
            $pat => Ok(extract_impl::extract_variant_assign!($pat)),
            _ => Err("err".into())
        }
    };
    ($exp:expr, $pat:pat_param, $err:expr) => {
        match $exp {
            $pat => Ok(extract_impl::extract_variant_assign!($pat)),
            _ => Err($err)
        }
    };
}
