use clap::{App, ArgMatches, Arg, SubCommand, AppSettings};

pub(crate) fn get_cmdline<'a>() -> ArgMatches<'a> {
    App::new("vlados")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("build")
                        .about("Build the design.")
                        .arg(Arg::with_name("top_file")
                            .short("tf")
                            .long("top_file")
                            .help("Specify the top-level file")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("top_mod")
                            .short("tm")
                            .long("top_mod")
                            .help("Specify the top-level module within `top_file`")
                            .takes_value(true)
                            .required(true)))
        .get_matches()
}