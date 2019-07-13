use oclif_command::Command;

pub struct Hello {}

impl Command for Hello {
    fn name(&self) -> String {
        String::from("hello")
    }
}
