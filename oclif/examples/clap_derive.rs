use clap::{AppSettings, Clap};

mod add {
    use clap::{AppSettings, Clap};

    /// Add a command to an existing CLI
    #[derive(Debug, Clap)]
    #[clap(name = "add")]
    #[clap(raw(setting = "AppSettings::ColoredHelp"))]
    #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
    pub struct Add {}
}

mod hello {
    use clap::{AppSettings, Clap};

    mod user {
        use clap::{AppSettings, Clap};

        #[derive(Debug, Clap)]
        #[clap(name = "user")]
        #[clap(raw(setting = "AppSettings::ColoredHelp"))]
        #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
        pub struct User {}
    }

    mod dog {
        use clap::{AppSettings, Clap};

        #[derive(Debug, Clap)]
        #[clap(name = "dog", about = "")]
        #[clap(raw(setting = "AppSettings::ColoredHelp"))]
        #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
        pub struct Dog {
            /// Name of the dog you are saying hello to
            ///
            /// A name is a term used for identification.
            ///
            /// Example is Doggy.
            #[clap(short = "n", long = "name")]
            name: String,
        }
    }

    #[derive(Debug, Clap)]
    #[clap(name = "hello")]
    #[clap(raw(setting = "AppSettings::ColoredHelp"))]
    #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
    #[clap(raw(setting = "AppSettings::Hidden"))]
    pub struct Hello {
        #[clap(subcommand)]
        cmd: HelloSubcommand,
    }

    #[derive(Debug, Clap)]
    enum HelloSubcommand {
        #[clap(name = "user")]
        User(user::User),
        #[clap(name = "dog")]
        Dog(dog::Dog),
    }
}

mod new {
    use clap::{AppSettings, Clap};

    /// Generate a new CLI program
    #[derive(Debug, Clap)]
    #[clap(name = "new")]
    #[clap(raw(setting = "AppSettings::ColoredHelp"))]
    #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
    #[clap(alias = "create")]
    pub struct New {
        /// Use defaults for every setting
        #[clap(short = "d", long = "defaults")]
        defaults: bool,
        /// Overwrite existing files
        #[clap(short = "f", long = "force")]
        force: bool,
    }
}

/// oclif: create your own CLI
///
/// oclif is an open source framework for building a command line interface (CLI) in Node.js.
/// Create CLIs with a few flags or advanced CLIs that have subcommands.
/// oclif makes it easy for you to build CLIs for your company, service, or your own development needs.
#[derive(Debug, Clap)]
#[clap(name = "oclif")]
#[clap(raw(setting = "AppSettings::ColoredHelp"))]
#[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
pub struct Oclif {
    #[clap(subcommand)]
    cmd: OclifSubcommand,
}

#[derive(Debug, Clap)]
enum OclifSubcommand {
    #[clap(name = "add")]
    Add(add::Add),
    #[clap(name = "hello")]
    Hello(hello::Hello),
    #[clap(name = "new")]
    New(new::New),
}

fn main() {
    let opt = Oclif::parse();
    println!("{:?}", opt);
}
