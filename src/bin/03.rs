extern crate regex;
use aoc::*;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Rectangle {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn parse_input(input: &str) -> Vec<Rectangle> {
    let re = Regex::new(r"#(\d+) +@ +(\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for line in input.trim().lines() {
        let caps = re.captures(line).unwrap();
        rectangles.push(Rectangle {
            id: caps[1].parse().unwrap(),
            x: caps[2].parse().unwrap(),
            y: caps[3].parse().unwrap(),
            w: caps[4].parse().unwrap(),
            h: caps[5].parse().unwrap(),
        });
    }
    rectangles
}

fn part1(rectangles: &Vec<Rectangle>) -> usize {
    // Paint the fabric; add 1 for every rectangle covering given square.
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    for r in rectangles {
        for i in r.x..r.x + r.w {
            for j in r.y..r.y + r.h {
                fabric[i][j] += 1;
            }
        }
    }
    fabric.iter().flatten().filter(|&&x| x > 1).count()
}

fn part2(rectangles: &Vec<Rectangle>) -> usize {
    // Paint the fabric with rectangle ids. Just like with normal paint, only last (topmost) id is
    // visible. If we're painting over an existing id, mark both old rectangle id and current one
    // as tainted.
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    let mut tainted: HashSet<usize> = HashSet::new();
    for r in rectangles {
        let mut current_is_tainted = false;
        for i in r.x..r.x + r.w {
            for j in r.y..r.y + r.h {
                let old_id = fabric[i][j];
                if old_id > 0 {
                    tainted.insert(old_id);
                    current_is_tainted = true;
                }
                fabric[i][j] = r.id;
            }
        }
        if current_is_tainted {
            tainted.insert(r.id);
        }
    }
    let all_ids: HashSet<usize> = rectangles.iter().map(|r| r.id).collect();
    let viable: Vec<usize> = all_ids.difference(&tainted).cloned().collect();
    assert_eq!(viable.len(), 1);
    viable[0]
}

fn main() {
    let rectangles = parse_input(&read_file("inputs/03"));
    let overlaps = part1(&rectangles);
    assert_eq!(overlaps, 111326);
    println!("Found {} overlapping square inches.", overlaps);

    let viable = part2(&rectangles);
    assert_eq!(viable, 1019);
    println!("The only viable rectangle: {}", viable);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 3);
    }
}
