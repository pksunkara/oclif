use oclif_macro::*;

/// Generate a new CLI program
#[name(new)]
#[aliases(create)]
/// Use defaults for every setting
#[arg(defaults, bool, short = "d", long = defaults)]
/// Overwrite existing files
#[arg(force, bool, short = f, long = force)]
fn run() {}
