use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::visit::Visit;
use syn::{parse_macro_input, Error, Pat, PatIdent, PatOr};

#[derive(Default, Debug)]
struct VisitPatIdent<'a> {
    idents: Vec<&'a Ident>,
    pat_or: Option<proc_macro2::TokenStream>,
}

impl<'ast> Visit<'ast> for VisitPatIdent<'ast> {
    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        // Since `None` cannot be distinguish from an ident, we check for and
        // ignore it here.
        if node.ident != "None" {
            self.idents.push(&node.ident);
        }
        syn::visit::visit_pat_ident(self, node)
    }

    fn visit_pat_or(&mut self, node: &'ast PatOr) {
        self.pat_or = Some(node.to_token_stream())
        // Since we're going to error the macro we can stop visiting.
    }
}

#[proc_macro]
pub fn extract_variant_assign(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Pat);
    let mut visitor = VisitPatIdent::default();
    visitor.visit_pat(&input);

    if let Some(tokens) = visitor.pat_or {
        let msg = "`variant` does not support `or` patterns";
        let err = Error::new_spanned(tokens, msg).to_compile_error();
        return TokenStream::from(err);
    }

    let tokens = match visitor.idents.as_slice() {
        [id] => quote! {
            #id
        },
        ids => quote! {
            (#(#ids),*)
        },
    };

    TokenStream::from(tokens)
}
