use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::visit::Visit;
use syn::{parse_macro_input, Pat, PatIdent};

#[derive(Default, Debug)]
struct VisitPatIdent {
    idents: Vec<Ident>,
}

impl<'ast> Visit<'ast> for VisitPatIdent {
    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        self.idents.push(node.ident.clone());
        syn::visit::visit_pat_ident(self, node)
    }
}

#[proc_macro]
pub fn extract_variant_assign(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Pat);
    let mut visitor = VisitPatIdent::default();
    visitor.visit_pat(&input);

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
