use proc_macro2::Span;
use syn::{parse, punctuated::Punctuated, token::Comma, Error, Lit, LitStr, Result};

#[derive(Debug)]
pub struct LitStrList {
    pub elems: Vec<LitStr>,
}

fn get_span(lit: Lit) -> Span {
    match lit {
        Lit::Str(x) => x.span(),
        Lit::ByteStr(x) => x.span(),
        Lit::Byte(x) => x.span(),
        Lit::Char(x) => x.span(),
        Lit::Int(x) => x.span(),
        Lit::Float(x) => x.span(),
        Lit::Bool(x) => x.span,
        Lit::Verbatim(x) => x.span(),
    }
}

impl parse::Parse for LitStrList {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let mut error = None;
        let lits: Punctuated<Lit, Comma> = input.parse_terminated(Lit::parse)?;

        let elems: Vec<LitStr> = lits
            .into_iter()
            .map(|lit| {
                if let Lit::Str(x) = lit {
                    return Some(x);
                }

                error = Some(Error::new(get_span(lit), "expected string literal"));
                None
            })
            .flatten()
            .collect();

        if error.is_some() {
            return Err(error.unwrap());
        }

        Ok(LitStrList { elems })
    }
}
