use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn hidden(_: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse_macro_input!(input);

    let mut named;

    if let Data::Struct(x) = data {
        if let Fields::Named(y) = x.fields {
            named = y.named;
        } else {
            panic!("'hidden' macro is allowed only on structs with named fields");
        }
    } else {
        panic!("'hidden' macro is allowed only on structs");
    }

    let gen = quote! {
        #(#attrs)*
        #[structopt(raw(setting = "AppSettings::Hidden"))]
        pub struct #ident {
            #(#named,)*
        }
    };

    gen.into()
}
