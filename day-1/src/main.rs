use clap::{self, App, AppSettings, Arg, SubCommand};
use std::{
    cmp::max,
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

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Advent of Code Day 1")
        .version("1.0")
        .author("Eran Cohen")
        .about("Calculates the fuel requirements for all the modules on a spacecraft")
        .subcommand(
            SubCommand::with_name("algo1")
                .about("Calculates needed fuel according to the algorithm specified in part 1")
                .version("1.0")
                .author("Eran Cohen")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                ),
        )
        .subcommand(
            SubCommand::with_name("algo2")
                .about("Calculates needed fuel according to the algorithm specified in part 2")
                .version("1.0")
                .author("Eran Cohen")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("The path to the input file")
                        .takes_value(true)
                        .required(true)
                        .validator_os(is_valid_path),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn calculate_fuel_p1<T: BufRead>(reader: &mut T) -> i32 {
    let mut fuel_sum = 0;
    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        fuel_sum += (mass / 3) - 2;
    }
    fuel_sum
}

fn calculate_fuel_p12<T: BufRead>(reader: &mut T) -> i32 {
    let mut fuel_sum = 0;
    for line in reader.lines() {
        let mut mass: i32 = line.unwrap().parse().unwrap();
        while mass > 0 {
            let fuel = max((mass / 3) - 2, 0);
            mass = fuel;
            fuel_sum += mass;
        }
    }
    fuel_sum
}
fn main() {
    // get the path to the file
    let matches = app().get_matches();

    // open the file and prepare it for reading
    let file = File::open(
        matches
            .subcommand()
            .1
            .unwrap()
            .value_of_os("input")
            .unwrap(),
    )
    .unwrap();
    let mut reader = BufReader::new(file);

    // calculate the fuel using the asked-for variation
    let fuel_sum = match matches.subcommand_name().unwrap() {
        "algo1" => calculate_fuel_p1(&mut reader),
        "algo2" => calculate_fuel_p12(&mut reader),
        _ => panic!("Whaaaaaaaaaaaat"),
    };

    println!("The calculated fuel sum is: {}", fuel_sum);
}
