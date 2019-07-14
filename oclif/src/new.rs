use oclif_command::*;

/// Generate a new CLI program
#[name("new")]
#[aliases("create", "start")]
/// This doc comment is ignored
impl Command for New {
    fn run(&self) {}
}
