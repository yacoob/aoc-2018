extern crate regex;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Rectangle {
    id: usize,
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
            id: caps[1].parse().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            w: caps[4].parse().unwrap(),
            h: caps[5].parse().unwrap(),
        });
    }

    // Paint the map with rectangle ids. Just like with normal paint, only last id is visible. If
    // we're painting over an existing id, mark both old rectangle id and current one as tainted.
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    let mut tainted: HashMap<usize, bool> = HashMap::new();
    for r in rectangles {
        let mut current_is_tainted = false;
        for i in r.x..r.x + r.w {
            for j in r.y..r.y + r.h {
                let old_id = fabric[i][j];
                if old_id > 0 {
                    tainted.insert(old_id, true);
                    current_is_tainted = true;
                }
                fabric[i][j] = r.id;
            }
        }
        tainted.insert(r.id, current_is_tainted);
    }
    let viable: Vec<usize> = tainted
        .iter()
        .filter_map(|(&id, &tainted)| if !tainted { Some(id) } else { None })
        .collect();
    assert_eq!(viable.len(), 1);
    println!("The only viable rectangle: {}", viable[0]);
}
