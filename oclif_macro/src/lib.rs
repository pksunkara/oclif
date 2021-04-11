extern crate proc_macro;

use proc_macro::TokenStream;

mod types;
mod utils;

// TODO: Don't use derive input unless it's struct

macro_rules! build_macro {
    ($name:ident) => {
        mod $name;

        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, input: TokenStream) -> TokenStream {
            $name::$name(attr, input)
        }
    };
}

build_macro!(name);
build_macro!(subcommands);
build_macro!(hidden);
build_macro!(usage);
build_macro!(aliases);
build_macro!(arg);
