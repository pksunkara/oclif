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

pub fn get_doc(attrs: &mut Vec<Attribute>) -> (String, String) {
    let mut started_long = false;
    let (mut short_doc, mut long_doc) = (vec![], vec![]);

    while !attrs.is_empty() {
        if !attrs.first().unwrap().path.is_ident("doc") {
            break;
        }

        // TODO: Improve performance since this is O(n)
        let attr = attrs.remove(0);
        let str = syn::parse2::<EqLitStr>(attr.tts).unwrap().str;

        if !started_long && str.value().is_empty() {
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

    for doc in long_doc.iter() {
        if doc.value().is_empty() {
            long.push_str("\n");
        } else {
            long.push_str(&doc.value());
        }
    }

    (short, long)
}
