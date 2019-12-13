use std::{
    ops::{Add, Mul},
    str::FromStr,
};

/// Represents a point in a 2-dimensional space.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Represents a vector in a 2-dimensional space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
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

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub beginning: Point,
    pub end: Point,
}

impl Line {
    pub fn new(beginning: Point, end: Point) -> Self {
        Self { beginning, end }
    }

    pub fn len(&self) -> i32 {
        (self.beginning.x - self.end.x).abs() + (self.beginning.y - self.end.y).abs()
    }

    pub fn intersects(&self, other: &Line) -> Option<Point> {
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
