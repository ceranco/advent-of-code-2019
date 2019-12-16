mod app;
mod math;

use app::*;
use math::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

struct Intersection {
    point: Point,
    line1: Line,
    line1_idx: usize,
    line2: Line,
    line2_idx: usize,
}

fn main() {
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
                if (idx1, idx2) != (0, 0) {
                    intersections.push(Intersection {
                        point,
                        line1: *line1,
                        line1_idx: idx1,
                        line2: *line2,
                        line2_idx: idx2,
                    });
                }
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
                if curr.1 < lowest.1 {
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
    // use the steps algorithm
    else {
        let (point, steps) = intersections
            .iter()
            .map(|intersection| {
                // check the number of steps
                let end_line1 = Line::new(intersection.line1.beginning, intersection.point);
                let steps1 = (&lines[0])
                    .iter()
                    .take(intersection.line1_idx)
                    .chain(once(&end_line1))
                    .fold(0, |ctr, line| ctr + line.len());

                let end_line2 = Line::new(intersection.line2.beginning, intersection.point);
                let steps2 = (&lines[1])
                    .iter()
                    .take(intersection.line2_idx)
                    .chain(once(&end_line2))
                    .fold(0, |ctr, line| ctr + line.len());
                (intersection.point, steps1 + steps2)
            })
            .fold((Point { x: 0, y: 0 }, std::i32::MAX), |lowest, curr| {
                if curr.1 < lowest.1 {
                    curr
                } else {
                    lowest
                }
            });
        println!("Closest intersection point: {:?}, steps: {}", point, steps);
    }
}
