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

pub fn get_doc(attrs: &[Attribute]) -> (String, String) {
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

    let (mut short, mut long) = (String::from(""), String::from(""));

    for doc in short_doc.iter() {
        short.push_str(&doc.value());
    }

    // TODO: New lines in long description
    for doc in long_doc.iter() {
        long.push_str(&doc.value());
    }

    short = short.trim().to_string();
    long = long.trim().to_string();

    (short, long)
}
