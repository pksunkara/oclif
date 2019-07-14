use super::types::LitStrList;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemImpl;

pub fn aliases(attr: TokenStream, input: TokenStream) -> TokenStream {
    let LitStrList { elems } = syn::parse_macro_input!(attr);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let gen = quote! {
        #(#attrs)*
        impl Command for #self_ty {
            #(#items)*

            fn aliases(&self) -> Vec<String> {
                vec![#(String::from(#elems)),*]
            }
        }
    };

    gen.into()
}
