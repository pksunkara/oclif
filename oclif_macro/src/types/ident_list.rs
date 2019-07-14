use syn::{parse, punctuated::Punctuated, token::Comma, Ident, Result};

#[derive(Debug)]
pub struct IdentList {
    pub elems: Punctuated<Ident, Comma>,
}

impl parse::Parse for IdentList {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(IdentList {
            elems: input.parse_terminated(Ident::parse)?,
        })
    }
}
