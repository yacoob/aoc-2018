use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance_to(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn from_str(s: &str) -> Point {
        let p: Vec<i32> = s.split(", ").map(|s| s.parse::<i32>().unwrap()).collect();
        Point { x: p[0], y: p[1] }
    }
}

impl fmt::Debug for Point {
    // My debug print is more concise :3
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    // Read in all seed points.
    let input = include_str!("../../inputs/06small").trim();
    let seeds: Vec<Point> = input.lines().map(|s| Point::from_str(s)).collect();

    // Go through entire square piece of the map; for every point calculate distance to all seed
    // points, sum those up, determine whether that point qualifies as safe.
    let safe_area_within = 10000;
    let max_size = 400; // eyeballed based on the numbers in the input.
    let mut safe_area = 0;
    for i in 0..max_size {
        for j in 0..max_size {
            // Point being considered.
            let p = Point::new(i, j);
            let p_total_distance: i32 = seeds.iter().map(|seed| p.distance_to(seed)).sum();
            if p_total_distance < safe_area_within {
                safe_area += 1;
            }
        }
    }
    // What, just like this?
    println!("Safe area size: {}", safe_area);
}
