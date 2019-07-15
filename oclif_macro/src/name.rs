use super::utils::{get_doc, to_kebab_literal};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, ItemImpl};

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
            docs.push(get_doc(&mut get_doc_attrs_from_bottom(&mut rest_attrs)));
            arg_attrs.push(attr);
        } else {
            rest_attrs.push(attr);
        }
    }

    (docs, arg_attrs, rest_attrs)
}

pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as Ident);
    let ItemImpl {
        mut attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let name = to_kebab_literal(&attr_ast);
    let (short, long) = get_doc(&mut attrs);
    let (arg_docs, arg_attrs, rest_attrs) = gather_arg_docs(attrs);

    let gen = quote! {
        #(#arg_attrs)*
        pub struct #self_ty {
        }

        #(#rest_attrs)*
        impl Command for #self_ty {
            fn name(&self) -> String {
                String::from(#name)
            }

            fn about(&self) -> String {
                String::from(#short)
            }

            fn long_about(&self) -> String {
                String::from(#long)
            }

            #(#items)*
        }
    };

    gen.into()
}
