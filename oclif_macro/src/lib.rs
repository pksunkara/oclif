extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemImpl, LitStr};

#[proc_macro_attribute]
pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast: LitStr = syn::parse(attr).unwrap();
    let input_ast: ItemImpl = syn::parse(input).unwrap();

    let attrs = &input_ast.attrs;
    let command_name = &input_ast.self_ty;
    let items = &input_ast.items;

    let gen = quote! {
        struct #command_name {}

        #(#attrs)*
        impl Command for #command_name {
            fn name(&self) -> String {
                String::from(#attr_ast)
            }

            #(#items)*
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn description(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast: LitStr = syn::parse(attr).unwrap();
    let input_ast: ItemImpl = syn::parse(input).unwrap();

    let attrs = &input_ast.attrs;
    let command_name = &input_ast.self_ty;
    let items = &input_ast.items;

    let gen = quote! {
        #(#attrs)*
        impl Command for #command_name {
            fn description(&self) -> String {
                String::from(#attr_ast)
            }

            #(#items)*
        }
    };

    gen.into()
}
