use clap::{App, AppSettings, Arg, ArgMatches};
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

fn is_valid_path(path: &OsStr) -> Result<(), OsString> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(OsString::from("The given path does not exist"))
    }
}

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 5")
        .author("Eran Cohen")
        .about("Runs the given program on an Intcode computer")
        .arg(
            Arg::with_name("input")
                .help("The input file containing the program.")
                .takes_value(true)
                .required(true)
                .validator_os(is_valid_path),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
}

#[derive(Debug)]
pub struct Opt {
    pub path: PathBuf,
}

impl From<ArgMatches<'_>> for Opt {
    fn from(matches: ArgMatches) -> Self {
        Self {
            path: matches.value_of_os("input").unwrap().into(),
        }
    }
}
