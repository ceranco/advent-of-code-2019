use clap::{App, Arg};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
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
    App::new("Advent of Code Day 3")
        .version("1.0")
        .author("Eran Cohen")
        .about("Returns the nearest intersection point of two wires to their source")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("The path to the input file")
                .takes_value(true)
                .required(true)
                .validator_os(is_valid_path),
        )
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // get the path to the input file
    let matches = app().get_matches();

    // prepare the file
    let file = File::open(matches.value_of_os("input").unwrap()).unwrap();
    

    // let lines: (Vec<Point>, Vec<Point>) = file.
    
    println!("Hello, world!");
}
