use super::types::IdentList;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl};

pub fn subcommands(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as IdentList);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let subcmds = &attr_ast.elems;
    let subcmds_values: Vec<Ident> = subcmds
        .iter()
        .map(|x| {
            let ident = inflector::cases::pascalcase::to_pascal_case(x.to_string().as_str());
            Ident::new(&ident, x.span())
        })
        .collect();

    let gen = quote! {
        #(mod #subcmds;)*
        #(#attrs)*
        impl Command for #self_ty {
            fn subcommands(&self) -> Vec<Box<dyn Command>> {
                vec![
                    #(Box::new(#subcmds::#subcmds_values {})),*
                ]
            }

            #(#items)*
        }
    };

    gen.into()
}
