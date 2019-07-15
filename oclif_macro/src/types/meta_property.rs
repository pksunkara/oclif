use syn::{parse, token::Eq, Ident, Lit, MetaNameValue, Result};

#[derive(Debug)]
pub struct MetaNameIdent {
    pub ident: Ident,
    pub eq_token: Eq,
    pub value: Ident,
}

impl parse::Parse for MetaNameIdent {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(MetaNameIdent {
            ident: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum MetaProperty {
    Value(MetaNameValue),
    Ident(MetaNameIdent),
}

impl parse::Parse for MetaProperty {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        if input.peek3(Lit) {
            input.parse().map(MetaProperty::Value)
        } else {
            input.parse().map(MetaProperty::Ident)
        }
    }
}
