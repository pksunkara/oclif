use structopt::{StructOpt, clap::AppSettings};

#[derive(Debug, StructOpt)]
/// oclif is an open source framework for building a command line interface (CLI) in Node.js.
/// Create CLIs with a few flags or advanced CLIs that have subcommands.
/// oclif makes it easy for you to build CLIs for your company, service, or your own development needs.
#[structopt(name = "oclif", about = "oclif: create your own CLI", version = "0.1.0")]
#[structopt(raw(setting = "AppSettings::ColoredHelp"))]
#[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
#[structopt(raw(setting = "AppSettings::GlobalVersion"))]
struct Oclif {
    #[structopt(subcommand)]
    cmd: OclifSubcommand
}

#[derive(Debug, StructOpt)]
enum OclifSubcommand {
    /// Generate a new CLI program
    #[structopt(name = "new", version = "0.2.0")]
    #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
    New {
        /// Use defaults for every setting
        #[structopt(short = "d", long = "defaults")]
        defaults: bool,
        /// Overwrite existing files
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
    /// Add a command to an existing CLI
    #[structopt(name = "add")]
    #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
    Add {
    },
    #[structopt(name = "hello")]
    #[structopt(raw(setting = "AppSettings::Hidden"))]
    #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
    Hello {
        /// Name of the person you are saying hello to
        ///
        /// A name is a term used for identification.
        ///
        /// Example is Alice.
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
    },
}

fn main() {
    let opt = Oclif::from_args();
    println!("{:?}", opt);
}
