use proc_macro::TokenStream;
use quote::quote;
use syn::ItemImpl;

pub fn hidden(_: TokenStream, input: TokenStream) -> TokenStream {
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

            fn is_hidden(&self) -> bool {
                true
            }
        }
    };

    gen.into()
}
