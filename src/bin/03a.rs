extern crate regex;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Rectangle {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

fn main() {
    // Not exactly a file-reading but it'll do here.
    let input = include_str!("../../inputs/03");

    // Read in all data.
    let re = Regex::new(r"#(\d+) +@ +(\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        rectangles.push(Rectangle {
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            w: caps[4].parse().unwrap(),
            h: caps[5].parse().unwrap(),
        });
    }

    // Paint the map; if a square is already painted, add it to overlapping squares set.
    let mut fabric: HashSet<(u16, u16)> = HashSet::new();
    let mut overlaps = fabric.clone();
    for r in rectangles {
        for i in r.x + 1..=r.x + r.w {
            for j in r.y + 1..=r.y + r.h {
                let p = (i, j);
                if fabric.contains(&p) {
                    overlaps.insert(p);
                } else {
                    fabric.insert(p);
                }
            }
        }
    }
    println!("Found {} overlapping square inches.", overlaps.len());
}
