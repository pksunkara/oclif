extern crate proc_macro;

use proc_macro::TokenStream;

mod types;
mod utils;

mod aliases;
mod hidden;
mod name;
mod subcommands;
mod usage;

#[proc_macro_attribute]
pub fn name(attr: TokenStream, input: TokenStream) -> TokenStream {
    name::name(attr, input)
}

#[proc_macro_attribute]
pub fn subcommands(attr: TokenStream, input: TokenStream) -> TokenStream {
    subcommands::subcommands(attr, input)
}

#[proc_macro_attribute]
pub fn hidden(attr: TokenStream, input: TokenStream) -> TokenStream {
    hidden::hidden(attr, input)
}

#[proc_macro_attribute]
pub fn usage(attr: TokenStream, input: TokenStream) -> TokenStream {
    usage::usage(attr, input)
}

#[proc_macro_attribute]
pub fn aliases(attr: TokenStream, input: TokenStream) -> TokenStream {
    aliases::aliases(attr, input)
}

#[proc_macro_attribute]
pub fn dummy(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
