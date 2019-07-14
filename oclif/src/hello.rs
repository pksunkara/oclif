use oclif_command::*;

#[name("hello")]
#[hidden]
impl Command for Hello {
    fn run(&self) {}
}
