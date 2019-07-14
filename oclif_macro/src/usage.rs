use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, LitStr};

pub fn usage(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as LitStr);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let gen = quote! {
        #(#attrs)*
        impl Command for #self_ty {
            fn usage(&self) -> Option<String> {
                Some(String::from(#attr_ast))
            }

            #(#items)*
        }
    };

    gen.into()
}
