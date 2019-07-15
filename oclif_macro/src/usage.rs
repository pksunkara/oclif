use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr};

pub fn usage(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as LitStr);
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse_macro_input!(input);

    let mut named;

    if let Data::Struct(x) = data {
        if let Fields::Named(y) = x.fields {
            named = y.named;
        } else {
            panic!("'usage' macro is allowed only on structs with named fields");
        }
    } else {
        panic!("'usage' macro is allowed only on structs");
    }

    let gen = quote! {
        #(#attrs)*
        #[structopt(usage = #attr_ast)]
        pub struct #ident {
            #(#named,)*
        }
    };

    gen.into()
}
