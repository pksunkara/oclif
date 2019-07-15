use super::types::IdentList;
use super::utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr};

pub fn subcommands(attr: TokenStream, input: TokenStream) -> TokenStream {
    let IdentList { elems } = syn::parse_macro_input!(attr);
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse_macro_input!(input);

    let mut named;

    if let Data::Struct(x) = data {
        if let Fields::Named(y) = x.fields {
            named = y.named;
        } else {
            panic!("'subcommands' macro is allowed only on structs with named fields");
        }
    } else {
        panic!("'subcommands' macro is allowed only on structs");
    }

    let subcmds_mods: &Vec<Ident> = &elems.iter().map(|x| utils::to_snake_ident(x)).collect();
    let subcmds_values: &Vec<Ident> = &elems.iter().map(|x| utils::to_pascal_ident(x)).collect();
    let subcmds_lits: &Vec<LitStr> = &elems.iter().map(|x| utils::to_kebab_literal(x)).collect();
    let subcmd_ident = Ident::new(&format!("{}Subcommand", ident), ident.span());
    let subcmds_values2 = subcmds_values.clone();

    let gen = quote! {
        #(mod #subcmds_mods;)*

        #[derive(Debug, StructOpt)]
        enum #subcmd_ident {
            #(#[structopt(name = #subcmds_lits)]
            #subcmds_values2(#subcmds_mods::#subcmds_values),)*
        }

        #(#attrs)*
        pub struct #ident {
            #(#named,)*
            #[structopt(subcommand)]
            cmd: #subcmd_ident,
        }
    };

    gen.into()
}
