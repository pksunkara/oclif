use oclif_command::*;

/// oclif: create your own CLI
///
/// oclif is an open source framework for building a command line interface (CLI) in Node.js.
/// Create CLIs with a few flags or advanced CLIs that have subcommands.
/// oclif makes it easy for you to build CLIs for your company, service, or your own development needs.
#[name(oclif)]
#[subcommands(hello, new)]
impl Command for Main {
    fn run(&self) {}
}

fn main() {
    run(&Main {});
}
