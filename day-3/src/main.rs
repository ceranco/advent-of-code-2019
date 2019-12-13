mod math;

use crate::math::*;
use clap::{App, AppSettings, Arg, SubCommand};
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
        .about("Returns the nearest intersection point of two wires relative to their source")
        .subcommand(
            SubCommand::with_name("manhattan")
                .about("Calculates distance using the Manhattan distance algorithm (part 1)")
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

fn main() {
    struct Intersection {
        point: Point,
        line1_idx: usize,
        line2_idx: usize,
    };

    // get the path to the input file
    let matches = app().get_matches();

    // prepare the file
    let file = File::open(
        matches
            .subcommand()
            .1
            .unwrap()
            .value_of_os("input")
            .unwrap(),
    )
    .unwrap();
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
    let mut intersections: Vec<Intersection> = vec![];
    for (idx1, line1) in (&lines[0]).iter().enumerate() {
        for (idx2, line2) in (&lines[1]).iter().enumerate() {
            if let Some(point) = line1.intersects(line2) {
                intersections.push(Intersection {
                    point,
                    line1_idx: idx1,
                    line2_idx: idx2,
                });
            }
        }
    }

    // use the manhattan distance algorithm
    if matches.subcommand_name().unwrap() == "manhattan" {
        let (point, distance) = intersections
            .iter()
            .map(|intersection| {
                (
                    intersection.point,
                    intersection.point.x.abs() + intersection.point.y.abs(),
                )
            })
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
}
