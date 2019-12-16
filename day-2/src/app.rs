use clap::{App, AppSettings, Arg, SubCommand};
use validators::{is_valid_i32, is_valid_path};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 2")
        .about("Emulates an Intcode computer")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs an Intcode program and prints the output")
                .arg(
                    Arg::with_name("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                )
                .arg(
                    Arg::with_name("alarm")
                        .short("a")
                        .long("alarm")
                        .help("Runs the program in the '1202 program alarm' state")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("reverse")
                .about("Finds the noun and verb for a given output and Intcode program")
                .arg(
                    Arg::with_name("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                )
                .arg(
                    Arg::with_name("output")
                        .help("The output that the program will search for")
                        .takes_value(true)
                        .required(true)
                        .validator(is_valid_i32),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
