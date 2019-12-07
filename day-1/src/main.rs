use clap::{self, App, Arg};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn is_valid_path(path: &OsStr) -> Result<(), OsString> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(OsString::from("The given path does not exist"))
    }
}

fn main() {
    // get the path to the file
    let matches = App::new("Advent of Code Day 1")
        .version("1.0")
        .author("Eran Cohen")
        .about("Calculates the fuel requirements for all the modules on a spacecraft")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("The path to the input file")
                .takes_value(true)
                .required(true)
                .validator_os(is_valid_path),
        )
        .get_matches();

    // open the file and prepare it for reading
    let file = File::open(matches.value_of_os("input").unwrap()).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    let mut fuel_sum = 0;
    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        fuel_sum += (mass / 3) - 2;
    }

    println!("The calculated fuel sum is: {}", fuel_sum);
}
