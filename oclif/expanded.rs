#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use clap::{AppSettings, Clap};
mod add {
    use clap::{AppSettings, Clap};
    /// Add a command to an existing CLI
    #[clap(name = "add")]
    #[clap(raw(setting = "AppSettings::ColoredHelp"))]
    #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
    pub struct Add {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Add {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Add {} => {
                    let mut debug_trait_builder = f.debug_struct("Add");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for Add {}
    impl ::clap::IntoApp for Add {
        fn into_app<'b>() -> ::clap::App<'b> {
            Self::augment_app(
                ::clap::App::new("add")
                    .version("0.1.0")
                    .about("Rust Open CLI Framework (based on oclif.io)")
                    .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
                    .about("Add a command to an existing CLI")
                    .setting(AppSettings::ColoredHelp)
                    .setting(AppSettings::VersionlessSubcommands),
            )
        }
    }
    impl<'b> Into<::clap::App<'b>> for Add {
        fn into(self) -> ::clap::App<'b> {
            use ::clap::IntoApp;
            <Add as ::clap::IntoApp>::into_app()
        }
    }
    impl ::clap::FromArgMatches for Add {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            Add {}
        }
    }
    impl From<::clap::ArgMatches> for Add {
        fn from(m: ::clap::ArgMatches) -> Self {
            use ::clap::FromArgMatches;
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(dead_code, unreachable_code)]
    #[doc(hidden)]
    impl Add {
        pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
            {
                app
            }
        }
        fn parse() -> Add {
            use ::clap::{FromArgMatches, IntoApp};
            Add::from_argmatches(&Add::into_app().get_matches())
        }
        fn try_parse() -> ::std::result::Result<Add, ::clap::Error> {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(Add::from_argmatches(&Add::into_app().try_get_matches()?))
        }
        fn parse_from<I, T>(itr: I) -> Add
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            Add::from_argmatches(&Add::into_app().get_matches_from(itr))
        }
        fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<Add, ::clap::Error>
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(Add::from_argmatches(
                &Add::into_app().try_get_matches_from(itr)?,
            ))
        }
        pub fn is_subcommand() -> bool {
            false
        }
    }
}
mod hello {
    use clap::{AppSettings, Clap};
    mod user {
        use clap::{AppSettings, Clap};
        #[clap(name = "user")]
        #[clap(raw(setting = "AppSettings::ColoredHelp"))]
        #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
        pub struct User {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for User {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    User {} => {
                        let mut debug_trait_builder = f.debug_struct("User");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(unused_variables)]
        impl ::clap::Clap for User {}
        impl ::clap::IntoApp for User {
            fn into_app<'b>() -> ::clap::App<'b> {
                Self::augment_app(
                    ::clap::App::new("user")
                        .version("0.1.0")
                        .about("Rust Open CLI Framework (based on oclif.io)")
                        .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
                        .setting(AppSettings::ColoredHelp)
                        .setting(AppSettings::VersionlessSubcommands),
                )
            }
        }
        impl<'b> Into<::clap::App<'b>> for User {
            fn into(self) -> ::clap::App<'b> {
                use ::clap::IntoApp;
                <User as ::clap::IntoApp>::into_app()
            }
        }
        impl ::clap::FromArgMatches for User {
            fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
                User {}
            }
        }
        impl From<::clap::ArgMatches> for User {
            fn from(m: ::clap::ArgMatches) -> Self {
                use ::clap::FromArgMatches;
                <Self as ::clap::FromArgMatches>::from_argmatches(&m)
            }
        }
        #[allow(dead_code, unreachable_code)]
        #[doc(hidden)]
        impl User {
            pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
                {
                    app
                }
            }
            fn parse() -> User {
                use ::clap::{FromArgMatches, IntoApp};
                User::from_argmatches(&User::into_app().get_matches())
            }
            fn try_parse() -> ::std::result::Result<User, ::clap::Error> {
                use ::clap::{FromArgMatches, IntoApp};
                Ok(User::from_argmatches(&User::into_app().try_get_matches()?))
            }
            fn parse_from<I, T>(itr: I) -> User
            where
                I: ::std::iter::IntoIterator<Item = T>,
                T: Into<::std::ffi::OsString> + Clone,
            {
                use ::clap::{FromArgMatches, IntoApp};
                User::from_argmatches(&User::into_app().get_matches_from(itr))
            }
            fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<User, ::clap::Error>
            where
                I: ::std::iter::IntoIterator<Item = T>,
                T: Into<::std::ffi::OsString> + Clone,
            {
                use ::clap::{FromArgMatches, IntoApp};
                Ok(User::from_argmatches(
                    &User::into_app().try_get_matches_from(itr)?,
                ))
            }
            pub fn is_subcommand() -> bool {
                false
            }
        }
    }
    mod dog {
        use clap::{AppSettings, Clap};
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
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Dog {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    Dog {
                        name: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Dog");
                        let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(unused_variables)]
        impl ::clap::Clap for Dog {}
        impl ::clap::IntoApp for Dog {
            fn into_app<'b>() -> ::clap::App<'b> {
                Self::augment_app(
                    ::clap::App::new("dog")
                        .version("0.1.0")
                        .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
                        .setting(AppSettings::ColoredHelp)
                        .setting(AppSettings::VersionlessSubcommands),
                )
            }
        }
        impl<'b> Into<::clap::App<'b>> for Dog {
            fn into(self) -> ::clap::App<'b> {
                use ::clap::IntoApp;
                <Dog as ::clap::IntoApp>::into_app()
            }
        }
        impl ::clap::FromArgMatches for Dog {
            fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
                Dog {
                    name: matches
                        .value_of("name")
                        .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                        .unwrap(),
                }
            }
        }
        impl From<::clap::ArgMatches> for Dog {
            fn from(m: ::clap::ArgMatches) -> Self {
                use ::clap::FromArgMatches;
                <Self as ::clap::FromArgMatches>::from_argmatches(&m)
            }
        }
        #[allow(dead_code, unreachable_code)]
        #[doc(hidden)]
        impl Dog {
            pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
                {
                    let app = app . arg ( :: clap :: Arg :: with_name ( "name" ) . takes_value ( true ) . required ( true ) . validator ( | s | { :: std :: str :: FromStr :: from_str ( & s ) . map ( | _ : String | ( ) ) . map_err ( | e | e . to_string ( ) ) } ) . help ( "Name of the dog you are saying hello to\n\nA name is a term used for identification.\n\nExample is Doggy." ) . short ( "n" . chars ( ) . nth ( 0 ) . unwrap ( ) ) . long ( "name" ) ) ;
                    app
                }
            }
            fn parse() -> Dog {
                use ::clap::{FromArgMatches, IntoApp};
                Dog::from_argmatches(&Dog::into_app().get_matches())
            }
            fn try_parse() -> ::std::result::Result<Dog, ::clap::Error> {
                use ::clap::{FromArgMatches, IntoApp};
                Ok(Dog::from_argmatches(&Dog::into_app().try_get_matches()?))
            }
            fn parse_from<I, T>(itr: I) -> Dog
            where
                I: ::std::iter::IntoIterator<Item = T>,
                T: Into<::std::ffi::OsString> + Clone,
            {
                use ::clap::{FromArgMatches, IntoApp};
                Dog::from_argmatches(&Dog::into_app().get_matches_from(itr))
            }
            fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<Dog, ::clap::Error>
            where
                I: ::std::iter::IntoIterator<Item = T>,
                T: Into<::std::ffi::OsString> + Clone,
            {
                use ::clap::{FromArgMatches, IntoApp};
                Ok(Dog::from_argmatches(
                    &Dog::into_app().try_get_matches_from(itr)?,
                ))
            }
            pub fn is_subcommand() -> bool {
                false
            }
        }
    }
    #[clap(name = "hello")]
    #[clap(raw(setting = "AppSettings::ColoredHelp"))]
    #[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
    #[clap(raw(setting = "AppSettings::Hidden"))]
    pub struct Hello {
        #[clap(subcommand)]
        cmd: HelloSubcommand,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Hello {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Hello {
                    cmd: ref __self_0_0,
                } => {
                    let mut debug_trait_builder = f.debug_struct("Hello");
                    let _ = debug_trait_builder.field("cmd", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for Hello {}
    impl ::clap::IntoApp for Hello {
        fn into_app<'b>() -> ::clap::App<'b> {
            Self::augment_app(
                ::clap::App::new("hello")
                    .version("0.1.0")
                    .about("Rust Open CLI Framework (based on oclif.io)")
                    .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
                    .setting(AppSettings::ColoredHelp)
                    .setting(AppSettings::VersionlessSubcommands)
                    .setting(AppSettings::Hidden),
            )
        }
    }
    impl<'b> Into<::clap::App<'b>> for Hello {
        fn into(self) -> ::clap::App<'b> {
            use ::clap::IntoApp;
            <Hello as ::clap::IntoApp>::into_app()
        }
    }
    impl ::clap::FromArgMatches for Hello {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            Hello {
                cmd: <HelloSubcommand>::from_subcommand(matches.subcommand()).unwrap(),
            }
        }
    }
    impl From<::clap::ArgMatches> for Hello {
        fn from(m: ::clap::ArgMatches) -> Self {
            use ::clap::FromArgMatches;
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(dead_code, unreachable_code)]
    #[doc(hidden)]
    impl Hello {
        pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
            {
                let app = <HelloSubcommand>::augment_app(app);
                let app = app.setting(::clap::AppSettings::SubcommandRequiredElseHelp);
                app
            }
        }
        fn parse() -> Hello {
            use ::clap::{FromArgMatches, IntoApp};
            Hello::from_argmatches(&Hello::into_app().get_matches())
        }
        fn try_parse() -> ::std::result::Result<Hello, ::clap::Error> {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(Hello::from_argmatches(
                &Hello::into_app().try_get_matches()?,
            ))
        }
        fn parse_from<I, T>(itr: I) -> Hello
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            Hello::from_argmatches(&Hello::into_app().get_matches_from(itr))
        }
        fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<Hello, ::clap::Error>
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(Hello::from_argmatches(
                &Hello::into_app().try_get_matches_from(itr)?,
            ))
        }
        pub fn is_subcommand() -> bool {
            false
        }
    }
    enum HelloSubcommand {
        #[clap(name = "user")]
        User(user::User),
        #[clap(name = "dog")]
        Dog(dog::Dog),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for HelloSubcommand {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&HelloSubcommand::User(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("User");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&HelloSubcommand::Dog(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Dog");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for HelloSubcommand {}
    impl ::clap::IntoApp for HelloSubcommand {
        fn into_app<'b>() -> ::clap::App<'b> {
            let app = ::clap::App::new("oclif")
                .version("0.1.0")
                .about("Rust Open CLI Framework (based on oclif.io)")
                .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
                .setting(::clap::AppSettings::SubcommandRequiredElseHelp);
            Self::augment_app(app)
        }
    }
    impl<'b> Into<::clap::App<'b>> for HelloSubcommand {
        fn into(self) -> ::clap::App<'b> {
            use ::clap::IntoApp;
            <HelloSubcommand as ::clap::IntoApp>::into_app()
        }
    }
    impl ::clap::FromArgMatches for HelloSubcommand {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            <HelloSubcommand>::from_subcommand(matches.subcommand()).unwrap()
        }
    }
    impl From<::clap::ArgMatches> for HelloSubcommand {
        fn from(m: ::clap::ArgMatches) -> Self {
            use ::clap::FromArgMatches;
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(unused_variables, dead_code, unreachable_code)]
    #[doc(hidden)]
    impl HelloSubcommand {
        pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
            app.subcommand({
                let subcommand = ::clap::App::new("user");
                let subcommand = {
                    let subcommand = <user::User>::augment_app(subcommand);
                    if <user::User>::is_subcommand() {
                        subcommand.setting(::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand
                    .version("0.1.0")
                    .about("Rust Open CLI Framework (based on oclif.io)")
                    .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
            })
            .subcommand({
                let subcommand = ::clap::App::new("dog");
                let subcommand = {
                    let subcommand = <dog::Dog>::augment_app(subcommand);
                    if <dog::Dog>::is_subcommand() {
                        subcommand.setting(::clap::AppSettings::SubcommandRequiredElseHelp)
                    } else {
                        subcommand
                    }
                };
                subcommand
                    .version("0.1.0")
                    .about("Rust Open CLI Framework (based on oclif.io)")
                    .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
            })
        }
        pub fn from_subcommand<'b>(sub: (&'b str, Option<&'b ::clap::ArgMatches>)) -> Option<Self> {
            match sub {
                ("user", Some(matches)) => Some(HelloSubcommand::User(
                    <user::User as ::clap::FromArgMatches>::from_argmatches(matches),
                )),
                ("dog", Some(matches)) => Some(HelloSubcommand::Dog(
                    <dog::Dog as ::clap::FromArgMatches>::from_argmatches(matches),
                )),
                _ => None,
            }
        }
        fn parse() -> HelloSubcommand {
            use ::clap::{FromArgMatches, IntoApp};
            HelloSubcommand::from_argmatches(&HelloSubcommand::into_app().get_matches())
        }
        fn try_parse() -> ::std::result::Result<HelloSubcommand, ::clap::Error> {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(HelloSubcommand::from_argmatches(
                &HelloSubcommand::into_app().try_get_matches()?,
            ))
        }
        fn parse_from<I, T>(itr: I) -> HelloSubcommand
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            HelloSubcommand::from_argmatches(&HelloSubcommand::into_app().get_matches_from(itr))
        }
        fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<HelloSubcommand, ::clap::Error>
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(HelloSubcommand::from_argmatches(
                &HelloSubcommand::into_app().try_get_matches_from(itr)?,
            ))
        }
        pub fn is_subcommand() -> bool {
            true
        }
    }
}
mod new {
    use clap::{AppSettings, Clap};
    /// Generate a new CLI program
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
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for New {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                New {
                    defaults: ref __self_0_0,
                    force: ref __self_0_1,
                } => {
                    let mut debug_trait_builder = f.debug_struct("New");
                    let _ = debug_trait_builder.field("defaults", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("force", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unused_variables)]
    impl ::clap::Clap for New {}
    impl ::clap::IntoApp for New {
        fn into_app<'b>() -> ::clap::App<'b> {
            Self::augment_app(
                ::clap::App::new("new")
                    .version("0.1.0")
                    .about("Rust Open CLI Framework (based on oclif.io)")
                    .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
                    .about("Generate a new CLI program")
                    .setting(AppSettings::ColoredHelp)
                    .setting(AppSettings::VersionlessSubcommands)
                    .alias("create"),
            )
        }
    }
    impl<'b> Into<::clap::App<'b>> for New {
        fn into(self) -> ::clap::App<'b> {
            use ::clap::IntoApp;
            <New as ::clap::IntoApp>::into_app()
        }
    }
    impl ::clap::FromArgMatches for New {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            New {
                defaults: matches.is_present("defaults"),
                force: matches.is_present("force"),
            }
        }
    }
    impl From<::clap::ArgMatches> for New {
        fn from(m: ::clap::ArgMatches) -> Self {
            use ::clap::FromArgMatches;
            <Self as ::clap::FromArgMatches>::from_argmatches(&m)
        }
    }
    #[allow(dead_code, unreachable_code)]
    #[doc(hidden)]
    impl New {
        pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
            {
                let app = app.arg(
                    ::clap::Arg::with_name("defaults")
                        .help("Use defaults for every setting")
                        .short("d".chars().nth(0).unwrap())
                        .long("defaults"),
                );
                let app = app.arg(
                    ::clap::Arg::with_name("force")
                        .help("Overwrite existing files")
                        .short("f".chars().nth(0).unwrap())
                        .long("force"),
                );
                app
            }
        }
        fn parse() -> New {
            use ::clap::{FromArgMatches, IntoApp};
            New::from_argmatches(&New::into_app().get_matches())
        }
        fn try_parse() -> ::std::result::Result<New, ::clap::Error> {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(New::from_argmatches(&New::into_app().try_get_matches()?))
        }
        fn parse_from<I, T>(itr: I) -> New
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            New::from_argmatches(&New::into_app().get_matches_from(itr))
        }
        fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<New, ::clap::Error>
        where
            I: ::std::iter::IntoIterator<Item = T>,
            T: Into<::std::ffi::OsString> + Clone,
        {
            use ::clap::{FromArgMatches, IntoApp};
            Ok(New::from_argmatches(
                &New::into_app().try_get_matches_from(itr)?,
            ))
        }
        pub fn is_subcommand() -> bool {
            false
        }
    }
}
/// oclif: create your own CLI
///
/// oclif is an open source framework for building a command line interface (CLI) in Node.js.
/// Create CLIs with a few flags or advanced CLIs that have subcommands.
/// oclif makes it easy for you to build CLIs for your company, service, or your own development needs.
#[clap(name = "oclif")]
#[clap(raw(setting = "AppSettings::ColoredHelp"))]
#[clap(raw(setting = "AppSettings::VersionlessSubcommands"))]
pub struct Oclif {
    #[clap(subcommand)]
    cmd: OclifSubcommand,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Oclif {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Oclif {
                cmd: ref __self_0_0,
            } => {
                let mut debug_trait_builder = f.debug_struct("Oclif");
                let _ = debug_trait_builder.field("cmd", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(unused_variables)]
impl ::clap::Clap for Oclif {}
impl ::clap::IntoApp for Oclif {
    fn into_app<'b>() -> ::clap::App<'b> {
        Self :: augment_app ( :: clap :: App :: new ( "oclif" ) . version ( "0.1.0" ) . about ( "Rust Open CLI Framework (based on oclif.io)" ) . author ( "Pavan Kumar Sunkara <pavan.sss1991@gmail.com>" ) . about ( "oclif: create your own CLI\n\noclif is an open source framework for building a command line interface (CLI) in Node.js. Create CLIs with a few flags or advanced CLIs that have subcommands. oclif makes it easy for you to build CLIs for your company, service, or your own development needs." ) . setting ( AppSettings :: ColoredHelp ) . setting ( AppSettings :: VersionlessSubcommands ) )
    }
}
impl<'b> Into<::clap::App<'b>> for Oclif {
    fn into(self) -> ::clap::App<'b> {
        use ::clap::IntoApp;
        <Oclif as ::clap::IntoApp>::into_app()
    }
}
impl ::clap::FromArgMatches for Oclif {
    fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
        Oclif {
            cmd: <OclifSubcommand>::from_subcommand(matches.subcommand()).unwrap(),
        }
    }
}
impl From<::clap::ArgMatches> for Oclif {
    fn from(m: ::clap::ArgMatches) -> Self {
        use ::clap::FromArgMatches;
        <Self as ::clap::FromArgMatches>::from_argmatches(&m)
    }
}
#[allow(dead_code, unreachable_code)]
#[doc(hidden)]
impl Oclif {
    pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
        {
            let app = <OclifSubcommand>::augment_app(app);
            let app = app.setting(::clap::AppSettings::SubcommandRequiredElseHelp);
            app
        }
    }
    fn parse() -> Oclif {
        use ::clap::{FromArgMatches, IntoApp};
        Oclif::from_argmatches(&Oclif::into_app().get_matches())
    }
    fn try_parse() -> ::std::result::Result<Oclif, ::clap::Error> {
        use ::clap::{FromArgMatches, IntoApp};
        Ok(Oclif::from_argmatches(
            &Oclif::into_app().try_get_matches()?,
        ))
    }
    fn parse_from<I, T>(itr: I) -> Oclif
    where
        I: ::std::iter::IntoIterator<Item = T>,
        T: Into<::std::ffi::OsString> + Clone,
    {
        use ::clap::{FromArgMatches, IntoApp};
        Oclif::from_argmatches(&Oclif::into_app().get_matches_from(itr))
    }
    fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<Oclif, ::clap::Error>
    where
        I: ::std::iter::IntoIterator<Item = T>,
        T: Into<::std::ffi::OsString> + Clone,
    {
        use ::clap::{FromArgMatches, IntoApp};
        Ok(Oclif::from_argmatches(
            &Oclif::into_app().try_get_matches_from(itr)?,
        ))
    }
    pub fn is_subcommand() -> bool {
        false
    }
}
enum OclifSubcommand {
    #[clap(name = "add")]
    Add(add::Add),
    #[clap(name = "hello")]
    Hello(hello::Hello),
    #[clap(name = "new")]
    New(new::New),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for OclifSubcommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&OclifSubcommand::Add(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Add");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&OclifSubcommand::Hello(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Hello");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&OclifSubcommand::New(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("New");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(unused_variables)]
impl ::clap::Clap for OclifSubcommand {}
impl ::clap::IntoApp for OclifSubcommand {
    fn into_app<'b>() -> ::clap::App<'b> {
        let app = ::clap::App::new("oclif")
            .version("0.1.0")
            .about("Rust Open CLI Framework (based on oclif.io)")
            .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
            .setting(::clap::AppSettings::SubcommandRequiredElseHelp);
        Self::augment_app(app)
    }
}
impl<'b> Into<::clap::App<'b>> for OclifSubcommand {
    fn into(self) -> ::clap::App<'b> {
        use ::clap::IntoApp;
        <OclifSubcommand as ::clap::IntoApp>::into_app()
    }
}
impl ::clap::FromArgMatches for OclifSubcommand {
    fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
        <OclifSubcommand>::from_subcommand(matches.subcommand()).unwrap()
    }
}
impl From<::clap::ArgMatches> for OclifSubcommand {
    fn from(m: ::clap::ArgMatches) -> Self {
        use ::clap::FromArgMatches;
        <Self as ::clap::FromArgMatches>::from_argmatches(&m)
    }
}
#[allow(unused_variables, dead_code, unreachable_code)]
#[doc(hidden)]
impl OclifSubcommand {
    pub fn augment_app<'b>(app: ::clap::App<'b>) -> ::clap::App<'b> {
        app.subcommand({
            let subcommand = ::clap::App::new("add");
            let subcommand = {
                let subcommand = <add::Add>::augment_app(subcommand);
                if <add::Add>::is_subcommand() {
                    subcommand.setting(::clap::AppSettings::SubcommandRequiredElseHelp)
                } else {
                    subcommand
                }
            };
            subcommand
                .version("0.1.0")
                .about("Rust Open CLI Framework (based on oclif.io)")
                .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
        })
        .subcommand({
            let subcommand = ::clap::App::new("hello");
            let subcommand = {
                let subcommand = <hello::Hello>::augment_app(subcommand);
                if <hello::Hello>::is_subcommand() {
                    subcommand.setting(::clap::AppSettings::SubcommandRequiredElseHelp)
                } else {
                    subcommand
                }
            };
            subcommand
                .version("0.1.0")
                .about("Rust Open CLI Framework (based on oclif.io)")
                .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
        })
        .subcommand({
            let subcommand = ::clap::App::new("new");
            let subcommand = {
                let subcommand = <new::New>::augment_app(subcommand);
                if <new::New>::is_subcommand() {
                    subcommand.setting(::clap::AppSettings::SubcommandRequiredElseHelp)
                } else {
                    subcommand
                }
            };
            subcommand
                .version("0.1.0")
                .about("Rust Open CLI Framework (based on oclif.io)")
                .author("Pavan Kumar Sunkara <pavan.sss1991@gmail.com>")
        })
    }
    pub fn from_subcommand<'b>(sub: (&'b str, Option<&'b ::clap::ArgMatches>)) -> Option<Self> {
        match sub {
            ("add", Some(matches)) => Some(OclifSubcommand::Add(
                <add::Add as ::clap::FromArgMatches>::from_argmatches(matches),
            )),
            ("hello", Some(matches)) => Some(OclifSubcommand::Hello(
                <hello::Hello as ::clap::FromArgMatches>::from_argmatches(matches),
            )),
            ("new", Some(matches)) => Some(OclifSubcommand::New(
                <new::New as ::clap::FromArgMatches>::from_argmatches(matches),
            )),
            _ => None,
        }
    }
    fn parse() -> OclifSubcommand {
        use ::clap::{FromArgMatches, IntoApp};
        OclifSubcommand::from_argmatches(&OclifSubcommand::into_app().get_matches())
    }
    fn try_parse() -> ::std::result::Result<OclifSubcommand, ::clap::Error> {
        use ::clap::{FromArgMatches, IntoApp};
        Ok(OclifSubcommand::from_argmatches(
            &OclifSubcommand::into_app().try_get_matches()?,
        ))
    }
    fn parse_from<I, T>(itr: I) -> OclifSubcommand
    where
        I: ::std::iter::IntoIterator<Item = T>,
        T: Into<::std::ffi::OsString> + Clone,
    {
        use ::clap::{FromArgMatches, IntoApp};
        OclifSubcommand::from_argmatches(&OclifSubcommand::into_app().get_matches_from(itr))
    }
    fn try_parse_from<I, T>(itr: I) -> ::std::result::Result<OclifSubcommand, ::clap::Error>
    where
        I: ::std::iter::IntoIterator<Item = T>,
        T: Into<::std::ffi::OsString> + Clone,
    {
        use ::clap::{FromArgMatches, IntoApp};
        Ok(OclifSubcommand::from_argmatches(
            &OclifSubcommand::into_app().try_get_matches_from(itr)?,
        ))
    }
    pub fn is_subcommand() -> bool {
        true
    }
}
fn main() {
    let opt = Oclif::parse();
    {
        ::std::io::_print(::std::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&opt,) {
                (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
            },
        ));
    };
}
