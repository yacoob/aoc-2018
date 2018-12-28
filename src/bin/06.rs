use aoc::*;

fn parse_input(input: &str) -> Vec<Point> {
    input.trim().lines().map(|s| Point::from_str(s)).collect()
}

fn part_ab(seeds: &Vec<Point>, safe_area_within: usize) -> (usize, usize) {
    // Go through entire square piece of the map; for every point calculate distance to all seed
    // points and:
    // * for part A: pick closest seed, score one more point for that seed.
    let mut areas = vec![None; seeds.len()];
    // * for part B: sum those up, determine whether that point qualifies as safe.
    let mut safe_area = 0;
    // Determine the piece of map we're scanning.
    let x_min = seeds.iter().map(|p| p.x).min().unwrap();
    let y_min = seeds.iter().map(|p| p.y).min().unwrap();
    let x_max = seeds.iter().map(|p| p.x).max().unwrap();
    let y_max = seeds.iter().map(|p| p.y).max().unwrap();
    for j in y_min..=y_max {
        for i in x_min..=x_max {
            // Point being considered.
            let p = Point::new(i, j);
            // p_distances will contain tuples: (distance, seed_number)
            let mut p_distances = Vec::with_capacity(seeds.len());
            for (n, seed) in seeds.iter().enumerate() {
                p_distances.push((p.distance_to(seed), n));
            }
            // part B: sum all distances, check if the sum is under desired threshold.
            let p_total_distance:usize = p_distances.iter().map(|x| x.0).sum();
            if p_total_distance < safe_area_within {
                safe_area += 1;
            }
            // part A:
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
            if p.x == x_min || p.y == y_min || p.x == x_max || p.y == y_max {
                areas[closest_seed] = None;
                continue;
            }
            // Looks like it's a legitimate point, which has a closest_seed. Add one point for
            // that seed/
            areas[closest_seed] = match areas[closest_seed] {
                None => Some(1usize),
                Some(x) => Some(x+1),
            }
        }
    }
    let largest_size = areas.into_iter().filter_map(|x| x).max().unwrap();
    (largest_size, safe_area)
}

fn main() {
    let seeds = parse_input(&read_file("inputs/06"));
    let (largest_area_size, safe_area_size) = part_ab(&seeds, 10_000);
    assert_eq!(largest_area_size, 3894);
    println!("Largest area (except infinite ones) is {}", largest_area_size);
    assert_eq!(safe_area_size, 39398);
    println!("Safe area size: {}", safe_area_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
"#;

    #[test]
    fn test_part1() {
        let seeds = parse_input(INPUT);
        assert_eq!(part_ab(&seeds, 32), (17, 16));
    }
}
