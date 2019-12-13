use clap::{App, Arg};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Mul},
    path::Path,
    str::FromStr,
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

/// Represents a point in a 2-dimensional space.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

/// Represents a vector in a 2-dimensional space.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, num: i32) -> Self::Output {
        Self {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, vec: Vector) -> Self::Output {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

impl FromStr for Vector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = match s.chars().nth(0) {
            Some('U') => Ok(Vector { x: 0, y: 1 }),
            Some('R') => Ok(Vector { x: 1, y: 0 }),
            Some('L') => Ok(Vector { x: -1, y: 0 }),
            Some('D') => Ok(Vector { x: 0, y: -1 }),
            _ => Err(String::from("Could not parse the direction")),
        }?;
        let multiplier = s[1..].parse::<i32>().map_err(|e| e.to_string())?;

        Ok(unit * multiplier)
    }
}

#[derive(Debug)]
struct Line {
    beginning: Point,
    end: Point,
}

impl Line {
    fn new(beginning: Point, end: Point) -> Self {
        Self { beginning, end }
    }

    fn intersects(&self, other: &Line) -> Option<Point> {
        let self_horizontal = self.beginning.y == self.end.y;
        let other_horizontal = other.beginning.y == other.end.y;

        // if both of the lines are horizontal or vertical, then they won't intersect
        if self_horizontal == other_horizontal {
            None
        } else {
            // order the lines
            let (horizontal, vertical) = if self_horizontal {
                (self, other)
            } else {
                (other, self)
            };

            // get the horizontal and vertical ranges
            let (min_x, max_x) = if horizontal.beginning.x < horizontal.end.x {
                (horizontal.beginning.x, horizontal.end.x)
            } else {
                (horizontal.end.x, horizontal.beginning.x)
            };
            let (min_y, max_y) = if vertical.beginning.y < vertical.end.y {
                (vertical.beginning.y, vertical.end.y)
            } else {
                (vertical.end.y, vertical.beginning.y)
            };

            // check the ranges
            let (x, y) = (vertical.beginning.x, horizontal.beginning.y);
            if min_x <= x && x <= max_x && min_y <= y && y <= max_y {
                Some(Point { x, y })
            } else {
                None
            }
        }
    }
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
