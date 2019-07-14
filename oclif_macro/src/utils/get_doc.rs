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

pub fn get_doc(attrs: Vec<Attribute>) -> (String, String, Vec<Attribute>) {
    let (mut started_long, mut save) = (false, false);
    let (mut short_doc, mut long_doc, mut new_attrs) = (vec![], vec![], vec![]);

    for attr in attrs.into_iter() {
        if !save && !attr.path.is_ident("doc") {
            save = true;
        }

        if save {
            new_attrs.push(attr);
            continue;
        }

        let str = syn::parse2::<EqLitStr>(attr.tts).unwrap().str;

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

    (short, long, new_attrs)
}
