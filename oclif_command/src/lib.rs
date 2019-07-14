pub use oclif_macro::*;

pub trait Command {
    fn name(&self) -> String;
    fn about(&self) -> String;
    fn long_about(&self) -> String;

    fn subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn is_hidden(&self) -> bool {
        false
    }

    fn usage(&self) -> Option<String> {
        None
    }

    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    // TODO: examples -> after_help

    fn run(&self) {}
}

pub fn run(cmd: &dyn Command) {
    cmd.run();
}
