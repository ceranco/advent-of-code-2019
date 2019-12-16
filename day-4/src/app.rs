use clap::{App, AppSettings, Arg, ArgMatches};
use validators::is_six_digit;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 4")
        .about("Calculates the number of 6-digit passwords that exist in the given range")
        .arg(
            Arg::with_name("min")
                .help("The minimum 6-digit number in the range of possible passwords")
                .takes_value(true)
                .required(true)
                .validator(is_six_digit)
                .display_order(1),
        )
        .arg(
            Arg::with_name("max")
                .help("The maximum 6-digit number in the range of possible passwords")
                .takes_value(true)
                .required(true)
                .validator(is_six_digit),
        )
        .arg(
            Arg::with_name("repeat-limit")
                .short("r")
                .long("repeat-limit")
                .help("This specifies that the possible passwords will be validated using the algorithm specified in part two.")
        )
        .setting(AppSettings::ArgRequiredElseHelp)
}

#[derive(Debug)]
pub struct Opt {
    pub min: i32,
    pub max: i32,
    pub repeat_limit: bool,
}

impl From<ArgMatches<'_>> for Opt {
    fn from(matches: ArgMatches) -> Self {
        Self {
            min: matches.value_of("min").unwrap().parse().unwrap(),
            max: matches.value_of("max").unwrap().parse().unwrap(),
            repeat_limit: matches.is_present("repeat-limit"),
        }
    }
}
