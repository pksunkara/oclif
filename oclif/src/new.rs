use oclif_command::*;

/// Generate a new CLI program
#[name(new)]
#[aliases(create, start)]
/// Use defaults for every setting
#[arg(defaults, bool)]
/// Overwrite existing files
#[arg(force, bool)]
#[arg(test, Option<String>)]
impl Command for New {
    fn run(&self) {}
}
