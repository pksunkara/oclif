use super::types::MetaProperty;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, punctuated, token::Comma, Ident, Result, Type};
use syn::{Data, DeriveInput, Fields};

#[derive(Debug)]
struct Arg {
    ident: Ident,
    comma_one: Comma,
    ty: Type,
    comma_two: Option<Comma>,
    meta: punctuated::Punctuated<MetaProperty, Comma>,
}

impl parse::Parse for Arg {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(Arg {
            ident: input.parse()?,
            comma_one: input.parse()?,
            ty: input.parse()?,
            comma_two: input.parse()?,
            meta: input.parse_terminated(MetaProperty::parse)?,
        })
    }
}

pub fn arg(attr: TokenStream, input: TokenStream) -> TokenStream {
    let Arg {
        ident: name, ty, ..
    } = syn::parse_macro_input!(attr);
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse_macro_input!(input);

    let mut named;

    if let Data::Struct(x) = data {
        if let Fields::Named(y) = x.fields {
            named = y.named;
        } else {
            panic!("'arg' macro is allowed only on structs with named fields");
        }
    } else {
        panic!("'arg' macro is allowed only on structs");
    }

    let gen = quote! {
        #(#attrs)*
        pub struct #ident {
            #(#named,)*
            #name: #ty,
        }
    };

    gen.into()
}
