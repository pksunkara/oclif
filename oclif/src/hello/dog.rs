use oclif_macro::*;

#[name(dog)]
/// Name of the dog you are saying hello to
///
/// A name is a term used for identification.
///
/// Example is Doggy.
#[arg(name, String, short = n, long = name)]
fn run() {}
