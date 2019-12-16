mod app;
use app::*;
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

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
        _ => unreachable!(),
    };

    println!("The calculated fuel sum is: {}", fuel_sum);
}
