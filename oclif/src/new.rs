use oclif_macro::*;

/// Generate a new CLI program
#[name(new)]
#[aliases(create, start)]
/// Use defaults for every setting
#[arg(defaults, bool)]
/// Overwrite existing files
#[arg(force, bool)]
fn run() {}
