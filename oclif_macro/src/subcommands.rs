use super::types::IdentList;
use super::utils::{to_pascal_ident, to_snake_ident};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl};

pub fn subcommands(attr: TokenStream, input: TokenStream) -> TokenStream {
    let IdentList { elems } = syn::parse_macro_input!(attr);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let subcmds_mods: &Vec<Ident> = &elems.iter().map(|x| to_snake_ident(x)).collect();
    let subcmds_values: &Vec<Ident> = &elems.iter().map(|x| to_pascal_ident(x)).collect();

    let gen = quote! {
        #(mod #subcmds_mods;)*
        #(#attrs)*
        impl Command for #self_ty {
            #(#items)*

            fn subcommands(&self) -> Vec<Box<dyn Command>> {
                vec![
                    #(Box::new(#subcmds_mods::#subcmds_values {})),*
                ]
            }
        }
    };

    gen.into()
}
