use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("oclif")
        .about("oclif: create your own CLI")
        .long_about("oclif: create your own CLI\n\noclif is an open source framework for building a command line interface (CLI) in Node.js. Create CLIs with a few flags or advanced CLIs that have subcommands. oclif makes it easy for you to build CLIs for your company, service, or your own development needs.")
        .version(clap::crate_version!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("add")
                .about("Add a command to an existing CLI")
                .version(clap::crate_version!())
                .setting(AppSettings::ColoredHelp)
                .setting(AppSettings::VersionlessSubcommands)
        )
        .subcommand(
            App::new("hello")
                .version(clap::crate_version!())
                .setting(AppSettings::ColoredHelp)
                .setting(AppSettings::VersionlessSubcommands)
                .setting(AppSettings::Hidden)
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("user")
                        .version(clap::crate_version!())
                        .setting(AppSettings::ColoredHelp)
                        .setting(AppSettings::VersionlessSubcommands)
                )
                .subcommand(
                    App::new("dog")
                        .version(clap::crate_version!())
                        .setting(AppSettings::ColoredHelp)
                        .setting(AppSettings::VersionlessSubcommands)
                        .arg(
                            Arg::with_name("name")
                               .short('n')
                               .long("name")
                               .help("Name of the dog you are saying hello to")
                               .long_help("Name of the dog you are saying hello to\n\nA name is a term used for identification.\n\nExample is Doggy.")
                               .takes_value(true)
                               .required(true)
                        )
                )
        )
        .subcommand(
            App::new("new")
                .about("Generate a new CLI program")
                .version(clap::crate_version!())
                .setting(AppSettings::ColoredHelp)
                .setting(AppSettings::VersionlessSubcommands)
                .arg(
                    Arg::with_name("defaults")
                        .short('d')
                        .long("defaults")
                        .help("Use defaults for every setting")
                )
                .arg(
                    Arg::with_name("force")
                        .short('f')
                        .long("force")
                        .help("Overwrite existing files")
                )
        )
        .get_matches();

    println!("{:?}", matches);
}
