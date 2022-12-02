use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Pat};

// todo: use visitor.

#[proc_macro]
pub fn extract_variant_assign(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Pat);

    panic!("{:?}", input.to_token_stream());
}
