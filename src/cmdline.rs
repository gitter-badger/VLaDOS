use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub(crate) fn get_cmdline<'a>() -> ArgMatches<'a> {
    App::new("vlados")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("build")
                .about("Build the design.")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("top_file")
                        .help("Specify the top-level file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches()
}
