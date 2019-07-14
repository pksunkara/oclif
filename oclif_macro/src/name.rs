use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{parse, token::Eq, Result};
use syn::{Attribute, ItemImpl, LitStr};

#[derive(Debug)]
struct EqLitStr {
    equal_token: Eq,
    str: LitStr,
}

impl parse::Parse for EqLitStr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        Ok(EqLitStr {
            equal_token: input.parse()?,
            str: input.parse()?,
        })
    }
}

fn get_all_doc(attrs: &[Attribute]) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut started_long = false;
    let (mut short_doc, mut long_doc) = (vec![], vec![]);

    for attr in attrs.iter() {
        if attr.path.is_ident("doc") {
            let str = syn::parse2::<EqLitStr>(attr.tts.clone()).unwrap().str;

            if str.value().is_empty() {
                started_long = true;
            }

            if started_long {
                long_doc.push(str);
            } else {
                short_doc.push(str);
            }
        }
    }

    let (mut short_doc_string, mut long_doc_string) = (String::from(""), String::from(""));

    for doc in short_doc.iter() {
        short_doc_string.push_str(&doc.value());
    }

    // TODO: New lines in long description
    for doc in long_doc.iter() {
        long_doc_string.push_str(&doc.value());
    }

    let short = quote! {
        fn about(&self) -> String {
            String::from(#short_doc_string)
        }
    };

    let long = quote! {
        fn long_about(&self) -> String {
            String::from(#long_doc_string)
        }
    };

    (short, long)
}

pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ast = syn::parse_macro_input!(attr as LitStr);
    let ItemImpl {
        attrs,
        self_ty,
        items,
        ..
    } = syn::parse_macro_input!(input);

    let (short, long) = get_all_doc(&attrs);

    let gen = quote! {
        pub struct #self_ty {}

        #(#attrs)*
        impl Command for #self_ty {
            fn name(&self) -> String {
                String::from(#attr_ast)
            }

            #short
            #long

            #(#items)*
        }
    };

    gen.into()
}
