use aoc::*;
use std::collections::HashSet;

fn part1(input: &str) -> i32 {
    input.lines().map(|line| line.parse::<i32>().unwrap()).sum()
}

fn part2(input: &str) -> i32 {
    let mut current_frequency = 0;
    // Iterate through frequency changes:
    //  part B: find the frequency which is reached for a second time first, provided we keep
    //  applying the changes.
    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    seen_frequencies.insert(current_frequency);
    'outer: loop {
        for change in input.lines() {
            let delta: i32 = change.parse().unwrap();
            current_frequency += delta;
            if seen_frequencies.contains(&current_frequency) {
                break 'outer;
            } else {
                seen_frequencies.insert(current_frequency);
            }
        }
    }
    current_frequency
}

fn main() {
    let input = read_file("inputs/01");

    let final_frequency = part1(&input);
    assert_eq!(final_frequency, 590);
    println!("Final frequency after one pass: {}", final_frequency);

    let stable_frequency = part2(&input);
    assert_eq!(stable_frequency, 83445);
    println!(
        "Frequency {} reached for the second time.",
        stable_frequency
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("+1\n+1\n+1"), 3);
        assert_eq!(part1("+1\n+1\n-2"), 0);
        assert_eq!(part1("-1\n-2\n-3"), -6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("+1\n-1"), 0);
        assert_eq!(part2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(part2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(part2("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
