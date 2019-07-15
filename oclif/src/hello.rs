use oclif_command::*;

#[name(hello)]
#[hidden]
/// Name of the person you are saying hello to
///
/// A name is a term used for identification.
///
/// Example is Alice.
#[arg(name, String, short = "n", long = name)]
impl Command for Hello {
    fn run(&self) {}
}
