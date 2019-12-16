use clap::{App, AppSettings, Arg, SubCommand};
use validators::is_valid_path;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 3")
        .about("Returns the nearest intersection point of two wires relative to their source")
        .subcommand(
            SubCommand::with_name("manhattan")
                .about("Calculates distance using the Manhattan distance algorithm (part 1)")
                .arg(
                    Arg::with_name("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                ),
        )
        .subcommand(
            SubCommand::with_name("steps")
                .about("Calculates distance using the steps algorithm (part 2)")
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
