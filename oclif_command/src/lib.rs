pub trait Command {
    fn name(&self) -> String;

    fn description(&self) -> String {
        String::from("")
    }

    fn is_hidden(&self) -> bool {
        false
    }

    fn commands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn run(&self) {}
}

pub fn run(cmd: &dyn Command) {
    cmd.run();
}
