use super::types::IdentList;
use super::utils::to_kebab_literal;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, LitStr};

pub fn aliases(attr: TokenStream, input: TokenStream) -> TokenStream {
    let IdentList { elems } = syn::parse_macro_input!(attr);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let aliases: &Vec<LitStr> = &elems.iter().map(|x| to_kebab_literal(x)).collect();

    let gen = quote! {
        #(#attrs)*
        impl Command for #self_ty {
            #(#items)*

            fn aliases(&self) -> Vec<String> {
                vec![#(String::from(#aliases)),*]
            }
        }
    };

    gen.into()
}
