use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

pub fn read_file(path: &str) -> String {
    let mut input = String::new();
    // Read the input.
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut input).unwrap();
    input.to_string()
}

pub struct Stopwatch {
    clock: Instant,
}

impl Stopwatch {
    pub fn start() -> Stopwatch {
        Stopwatch {
            clock: Instant::now(),
        }
    }

    pub fn split(self) {
        let d = self.clock.elapsed();
        eprintln!("Done in {}.{:06} seconds", d.as_secs(), d.subsec_micros());
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();
        let x = parts[0].trim().parse::<usize>()?;
        let y = parts[1].trim().parse::<usize>()?;
        Ok(Point { x, y })
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }

    pub fn distance_to(&self, other: &Point) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }
}
