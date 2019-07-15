use oclif_macro::*;

#[name(hello)]
#[hidden]
/// Name of the person you are saying hello to
///
/// A name is a term used for identification.
///
/// Example is Alice.
#[arg(name, Option<String>, short = "n", long = name)]
fn run() {}
