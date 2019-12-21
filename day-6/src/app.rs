use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::path::PathBuf;
use validators::is_valid_path;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 6")
        .about("Commands for working with 'Universal Orbit Maps'")
        .subcommand(
            SubCommand::with_name("checksum")
                .about("Calculates the checksum of a given 'Universal Orbit Map'")
                .arg(
                    Arg::with_name("input")
                        .help("The input file containing the map.")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                ),
        )
        .subcommand(
            SubCommand::with_name("distance")
                .about(
                    "Calculates the distance between two stars in the given 'Universal Orbit Map'",
                )
                .arg(
                    Arg::with_name("input")
                        .help("The input file containing the map.")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path)
                        .display_order(1),
                )
                .arg(
                    Arg::with_name("src")
                        .help("The first star.")
                        .takes_value(true)
                        .required(true)
                        .display_order(2),
                )
                .arg(
                    Arg::with_name("dst")
                        .help("The second star.")
                        .takes_value(true)
                        .required(true)
                        .display_order(3),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

#[derive(Debug)]
pub enum Command {
    Checksum,
    Distance { src: String, dst: String },
}

#[derive(Debug)]
pub struct Opt {
    pub path: PathBuf,
    pub command: Command,
}

impl From<ArgMatches<'_>> for Opt {
    fn from(matches: ArgMatches) -> Self {
        Opt {
            path: matches
                .subcommand()
                .1
                .unwrap()
                .value_of_os("input")
                .unwrap()
                .into(),
            command: match matches.subcommand_name().unwrap() {
                "checksum" => Command::Checksum,
                "distance" => Command::Distance {
                    src: matches
                        .subcommand()
                        .1
                        .unwrap()
                        .value_of("src")
                        .unwrap()
                        .to_owned(),
                    dst: matches
                        .subcommand()
                        .1
                        .unwrap()
                        .value_of("dst")
                        .unwrap()
                        .to_owned(),
                },
                _ => unreachable!(),
            },
        }
    }
}
