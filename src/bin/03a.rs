extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Rectangle {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
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
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    for r in rectangles {
        for i in r.x..r.x + r.w {
            for j in r.y..r.y + r.h {
                fabric[i][j] += 1;
            }
        }
    }
    let overlaps = fabric.iter().flatten().filter(|&&x| x > 1).count();
    println!("Found {} overlapping square inches.", overlaps);
}
