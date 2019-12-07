use clap::{self, App, Arg};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn is_valid_path(path: &OsStr) -> Result<(), OsString> {
    if Path::new(path).exists() {
        Ok(())
    } else {
        Err(OsString::from("The given path does not exist"))
    }
}

fn calculate_fuel_p1<T: BufRead>(reader: &mut T) -> i32 {
    let mut fuel_sum = 0;
    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        fuel_sum += (mass / 3) - 2;
    }
    fuel_sum
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
    let mut reader = BufReader::new(file);

    // calculate the fueld using the asked-for variation
    let fuel_sum = calculate_fuel_p1(&mut reader);

    println!("The calculated fuel sum is: {}", fuel_sum);
}
