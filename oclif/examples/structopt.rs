use structopt::{clap::AppSettings, StructOpt};

mod add {
    use structopt::{clap::AppSettings, StructOpt};

    /// Add a command to an existing CLI
    #[derive(Debug, StructOpt)]
    #[structopt(name = "add")]
    #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
    #[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
    pub struct Add {}
}

mod hello {
    use structopt::{clap::AppSettings, StructOpt};

    mod user {
        use structopt::{clap::AppSettings, StructOpt};

        #[derive(Debug, StructOpt)]
        #[structopt(name = "user")]
        #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
        #[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
        pub struct User {}
    }

    mod dog {
        use structopt::{clap::AppSettings, StructOpt};

        #[derive(Debug, StructOpt)]
        #[structopt(name = "dog")]
        #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
        #[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
        pub struct Dog {
            /// Name of the dog you are saying hello to
            ///
            /// A name is a term used for identification.
            ///
            /// Example is Doggy.
            #[structopt(short = "n", long = "name")]
            name: String,
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(name = "hello")]
    #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
    #[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
    #[structopt(raw(setting = "AppSettings::Hidden"))]
    pub struct Hello {
        #[structopt(subcommand)]
        cmd: HelloSubcommand,
    }

    #[derive(Debug, StructOpt)]
    enum HelloSubcommand {
        #[structopt(name = "user")]
        User(user::User),
        #[structopt(name = "dog")]
        Dog(dog::Dog),
    }
}

mod new {
    use structopt::{clap::AppSettings, StructOpt};

    /// Generate a new CLI program
    #[derive(Debug, StructOpt)]
    #[structopt(name = "new")]
    #[structopt(raw(setting = "AppSettings::ColoredHelp"))]
    #[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
    #[structopt(alias = "create")]
    pub struct New {
        /// Use defaults for every setting
        #[structopt(short = "d", long = "defaults")]
        defaults: bool,
        /// Overwrite existing files
        #[structopt(short = "f", long = "force")]
        force: bool,
    }
}

/// oclif: create your own CLI
///
/// oclif is an open source framework for building a command line interface (CLI) in Node.js.
/// Create CLIs with a few flags or advanced CLIs that have subcommands.
/// oclif makes it easy for you to build CLIs for your company, service, or your own development needs.
#[derive(Debug, StructOpt)]
#[structopt(name = "oclif")]
#[structopt(raw(setting = "AppSettings::ColoredHelp"))]
#[structopt(raw(setting = "AppSettings::VersionlessSubcommands"))]
struct Oclif {
    #[structopt(subcommand)]
    cmd: OclifSubcommand,
}

#[derive(Debug, StructOpt)]
enum OclifSubcommand {
    #[structopt(name = "add")]
    Add(add::Add),
    #[structopt(name = "hello")]
    Hello(hello::Hello),
    #[structopt(name = "new")]
    New(new::New),
}

fn main() {
    let opt = Oclif::from_args();
    println!("{:?}", opt);
}
