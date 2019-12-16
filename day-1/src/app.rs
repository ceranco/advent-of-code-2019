use clap::{App, AppSettings, Arg, SubCommand};
use validators::is_valid_path;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 1")
        .about("Calculates the fuel requirements for all the modules on a spacecraft")
        .subcommand(
            SubCommand::with_name("algo1")
                .about("Calculates needed fuel according to the algorithm specified in part 1")
                .arg(
                    Arg::with_name("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                ),
        )
        .subcommand(
            SubCommand::with_name("algo2")
                .about("Calculates needed fuel according to the algorithm specified in part 2")
                .arg(
                    Arg::with_name("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
