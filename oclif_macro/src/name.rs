use super::utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, ItemFn};

fn get_doc_attrs_from_bottom(attrs: &mut Vec<Attribute>) -> Vec<Attribute> {
    let mut doc_attrs = vec![];

    while !attrs.is_empty() {
        if !attrs.last().unwrap().path.is_ident("doc") {
            break;
        }

        doc_attrs.push(attrs.pop().unwrap());
    }

    doc_attrs.reverse();
    doc_attrs
}

fn gather_arg_docs(
    attrs: Vec<Attribute>,
) -> (Vec<(String, String)>, Vec<Attribute>, Vec<Attribute>) {
    let (mut docs, mut arg_attrs, mut rest_attrs) = (vec![], vec![], vec![]);

    for attr in attrs.into_iter() {
        if attr.path.is_ident("arg") {
            docs.push(utils::get_doc(&mut get_doc_attrs_from_bottom(
                &mut rest_attrs,
            )));
            arg_attrs.push(attr);
        } else {
            rest_attrs.push(attr);
        }
    }

    (docs, arg_attrs, rest_attrs)
}

pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as Ident);
    let ItemFn {
        mut attrs, ident, ..
    } = syn::parse_macro_input!(input);

    let name = utils::to_kebab_literal(&attr_ast);
    let ty = utils::to_pascal_ident(&attr_ast);
    let (short, long) = utils::get_doc(&mut attrs);
    // let (arg_docs, arg_attrs, rest_attrs) = gather_arg_docs(attrs);

    let gen = quote! {
        use structopt::{StructOpt, clap::AppSettings};
        // #(#arg_attrs)*
        // #(#rest_attrs)*
        #(#attrs)*
        #[derive(Debug, StructOpt)]
        #[structopt(name = #name, about = #short, long_about = #long)]
        #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
        pub struct #ty {
        }

        pub fn #ident() {}
    };

    gen.into()
}
