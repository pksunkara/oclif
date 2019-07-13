use oclif_command::{run, Command};
use oclif_macro::*;

mod hello;

#[name("oclif")]
#[description("oclif: create your own CLI")]
impl Command for Main {
    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![Box::new(hello::Hello {})]
    }
}

fn main() {
    run(&Main {});
}
