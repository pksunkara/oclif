use inflector::cases::snakecase::to_snake_case;
use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::kebabcase::to_kebab_case;
use syn::{Ident, LitStr};

mod get_doc;

pub use get_doc::get_doc;

pub fn to_kebab_literal(ident: &Ident) -> LitStr {
    LitStr::new(&to_kebab_case(&ident.to_string()), ident.span())
}

pub fn to_pascal_ident(ident: &Ident) -> Ident {
    Ident::new(&to_pascal_case(&ident.to_string()), ident.span())
}

pub fn to_snake_ident(ident: &Ident) -> Ident {
    Ident::new(&to_snake_case(&ident.to_string()), ident.span())
}
