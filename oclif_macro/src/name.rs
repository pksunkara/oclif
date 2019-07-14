use super::utils::get_doc;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, LitStr};

pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as LitStr);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let (short, long) = get_doc(&attrs);

    let gen = quote! {
        pub struct #self_ty {}

        #(#attrs)*
        impl Command for #self_ty {
            fn name(&self) -> String {
                String::from(#attr_ast)
            }

            #short
            #long

            #(#items)*
        }
    };

    gen.into()
}
