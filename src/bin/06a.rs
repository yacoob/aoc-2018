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
    let input = include_str!("../../inputs/06").trim();
    let seeds: Vec<Point> = input.lines().map(|s| Point::from_str(s)).collect();

    // Go through entire square piece of the map; for every point calculate distance to all seed
    // points, pick closest seed, score one more point for that seed.
    //
    // This is a brute force approach. Computers are fast, and I'm here to play with Rust, and not
    // to gracefully implement Fortune's algorithm
    // https://en.wikipedia.org/wiki/Fortune%27s_algorithm
    let mut areas = vec![0; seeds.len()];
    let max_size = 400; // eyeballed based on the numbers in the input.
    for i in 0..max_size {
        for j in 0..max_size {
            // Point being considered.
            let p = Point::new(i, j);
            // p_distances will contain tuples: (distance, seed_number)
            let mut p_distances = Vec::with_capacity(seeds.len());
            for (n, seed) in seeds.iter().enumerate() {
                p_distances.push((p.distance_to(seed), n));
            }
            p_distances.sort();
            // Is there a contest between at least 2 seeds for the smallest distance from p? If
            // yes, this map point is contested and doesn't belong to any of the seeds' areas.
            if p_distances[0].0 == p_distances[1].0 {
                continue;
            }
            // Check whether closest seed is really eligible.
            let closest_seed = p_distances[0].1;
            // If p is on the edge of the map, it means the seed it's closest to will claim an
            // infinite amount of points outside of the area of the map we're inspecting.
            if p.x == 0 || p.y == 0 || p.x == max_size - 1 || p.y == max_size - 1 {
                areas[closest_seed] = std::i32::MIN;
            }
            // Looks like it's a legitimate point, which has a closest_seed. Add one point for
            // that seed.
            areas[closest_seed] += 1;
        }
    }
    println!(
        "Largest area (except infinite ones) is {}.",
        areas.iter().max().unwrap()
    );
}
