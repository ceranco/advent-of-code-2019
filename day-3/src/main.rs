mod math;

use crate::math::*;
use clap::{App, Arg};
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

fn main() {
    // get the path to the input file
    let matches = app().get_matches();

    // prepare the file
    let file = File::open(matches.value_of_os("input").unwrap()).unwrap();
    let reader = BufReader::new(file);

    // parse the file
    let lines: Vec<Vec<Line>> = reader
        .lines()
        .take(2) // take the two first lines
        .map(|line| {
            line.unwrap()
                .split(",") // split each line into tokens
                .map(|token| token.parse::<Vector>().unwrap()) // convert into vectors
                .fold(vec![Point { x: 0, y: 0 }], |mut acc, vec| {
                    acc.push(*acc.last().unwrap() + vec);
                    acc
                }) // convert the vectors into a list of points
                .iter()
                .fold((None, vec![]), |mut acc, point| {
                    if let Some(prev) = acc.0 {
                        acc.1.push(Line::new(prev, *point));
                    }

                    acc.0 = Some(*point);
                    acc
                })
                .1 // convert the points into lines
        })
        .collect();

    // check for intersections
    let mut intersections: Vec<Point> = vec![];
    for line1 in &lines[0] {
        for line2 in &lines[1] {
            if let Some(point) = line1.intersects(line2) {
                intersections.push(point);
            }
        }
    }

    let (point, distance) = intersections
        .iter()
        .map(|point| (*point, point.x.abs() + point.y.abs()))
        .fold((Point { x: 0, y: 0 }, std::i32::MAX), |lowest, curr| {
            if curr.1 != 0 && curr.1 < lowest.1 {
                curr
            } else {
                lowest
            }
        });

    println!(
        "Closest intersection point: {:?}, Manhattan distance: {}",
        point, distance
    );
}
