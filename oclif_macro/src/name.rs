use super::utils::{get_doc, to_kebab_literal};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl};

pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as Ident);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let (short, long) = get_doc(&attrs);
    let name = to_kebab_literal(&attr_ast);

    let gen = quote! {
        pub struct #self_ty {}

        #(#attrs)*
        impl Command for #self_ty {
            fn name(&self) -> String {
                String::from(#name)
            }

            #short
            #long

            #(#items)*
        }
    };

    gen.into()
}
