use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse, token::Eq, Result};
use syn::{Attribute, LitStr};

#[derive(Debug)]
struct EqLitStr {
    equal_token: Eq,
    str: LitStr,
}

impl parse::Parse for EqLitStr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(EqLitStr {
            equal_token: input.parse()?,
            str: input.parse()?,
        })
    }
}

pub fn get_doc(attrs: &[Attribute]) -> (TokenStream, TokenStream) {
    let mut started_long = false;
    let (mut short_doc, mut long_doc) = (vec![], vec![]);

    for attr in attrs.iter() {
        if !attr.path.is_ident("doc") {
            break;
        }

        let str = syn::parse2::<EqLitStr>(attr.tts.clone()).unwrap().str;

        if str.value().is_empty() {
            started_long = true;
        }

        if started_long {
            long_doc.push(str);
        } else {
            short_doc.push(str);
        }
    }

    let (mut short_doc_string, mut long_doc_string) = (String::from(""), String::from(""));

    for doc in short_doc.iter() {
        short_doc_string.push_str(&doc.value());
    }

    // TODO: New lines in long description
    for doc in long_doc.iter() {
        long_doc_string.push_str(&doc.value());
    }

    short_doc_string = short_doc_string.trim().to_string();
    long_doc_string = long_doc_string.trim().to_string();

    let short = quote! {
        fn about(&self) -> String {
            String::from(#short_doc_string)
        }
    };

    let long = quote! {
        fn long_about(&self) -> String {
            String::from(#long_doc_string)
        }
    };

    (short, long)
}
