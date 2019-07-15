use super::types::IdentList;
use super::utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr};

pub fn aliases(attr: TokenStream, input: TokenStream) -> TokenStream {
    let IdentList { elems } = syn::parse_macro_input!(attr);
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse_macro_input!(input);

    let mut named;

    if let Data::Struct(x) = data {
        if let Fields::Named(y) = x.fields {
            named = y.named;
        } else {
            panic!("'aliases' macro is allowed only on structs with named fields");
        }
    } else {
        panic!("'aliases' macro is allowed only on structs");
    }

    let aliases: &Vec<LitStr> = &elems.iter().map(|x| utils::to_kebab_literal(x)).collect();

    let gen = quote! {
        #(#attrs)*
        #(#[structopt(alias = #aliases)])*
        pub struct #ident {
            #(#named,)*
        }
    };

    gen.into()
}
