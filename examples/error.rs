use clap::Clap;
use oclif::{finish, term::ERR_YELLOW, CliError};
use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
enum Error {
    #[error("no key {0} found in config")]
    NoConfig(String),
    #[error("{0} is not a directory")]
    NotDir(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl CliError for Error {
    fn color(self) -> Self {
        match self {
            Self::NoConfig(key) => Self::NoConfig(ERR_YELLOW.apply_to(key).to_string()),
            Self::NotDir(path) => Self::NotDir(ERR_YELLOW.apply_to(path).to_string()),
            _ => self,
        }
    }
}

#[derive(Clap, Debug)]
struct Config {}

impl Config {
    fn run(self) -> Result<(), Error> {
        Err(Error::NoConfig("password".into()))
    }
}

#[derive(Clap, Debug)]
struct New {}

impl New {
    fn run(self) -> Result<(), Error> {
        Err(Error::NotDir("config".into()))
    }
}

#[derive(Clap, Debug)]
struct Tool {
    #[clap(subcommand)]
    cmd: ToolSubcommand,
}

#[derive(Clap, Debug)]
enum ToolSubcommand {
    Config(Config),
    New(New),
}

fn main() {
    let program = Tool::parse();

    let result = match program.cmd {
        ToolSubcommand::Config(x) => x.run(),
        ToolSubcommand::New(x) => x.run(),
    };

    finish(result);
}
