use clap::{App, AppSettings, Arg, ArgMatches};
use std::path::PathBuf;
use validators::is_valid_path;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 7")
        .about("Finds the largest output signal that can be sent to the thrusters using the given 'Amplifier Controller Software'")
        .arg(
            Arg::with_name("input")
                .help("The input file containing the 'Amplifier Controller Software'")
                .takes_value(true)
                .required(true)
                .validator_os(is_valid_path),
        )
        .arg(
            Arg::with_name("feedback")
                .help("Runs the amplifiers using a feedback loop")
                .short("f")
                .long("feedback")
        )
        .setting(AppSettings::ArgRequiredElseHelp)
}

#[derive(Debug)]
pub struct Opt {
    pub path: PathBuf,
    pub feedback: bool,
}

impl From<ArgMatches<'_>> for Opt {
    fn from(matches: ArgMatches) -> Self {
        Self {
            path: matches.value_of_os("input").unwrap().into(),
            feedback: matches.is_present("feedback"),
        }
    }
}
