use aoc::*;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

fn part1(polymer: &[char]) -> usize {
    // Go through the polymer left to right, check characters, add them to final_polymer
    // when appropriate.
    let mut final_polymer: Vec<char> = Vec::new();
    for unit in polymer {
        // Check last unit of the final_polymer (will be None on empty polymer), compare to
        // current unit.
        if final_polymer
            .last()
            .map(|&previous| previous.eq_ignore_ascii_case(&unit) && previous != *unit)
            .unwrap_or(false)
        {
            final_polymer.pop();
        } else {
            final_polymer.push(*unit);
        }
    }
    final_polymer.len()
}

fn part2(polymer: &[char]) -> usize {
    let all_units: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut efficiency: HashMap<char, usize> = HashMap::new();
    for removed_unit in all_units {
        // Prepare a new polymer with specific units removed.
        let mut polymer = polymer.to_owned();
        // I wonder if String::replace() on the input would be faster.
        polymer.retain(|&x| !x.eq_ignore_ascii_case(&removed_unit));

        // Process reactions within new polymer.
        //
        // Scan polymer from left to right, keeping track of position and previous unit.
        // Once reaction is detected, remove current and previous unit, move scan position to
        // previous-previous unit and continue scanning.
        let mut n = 1;
        let mut previous_unit = polymer[0];
        while n < polymer.len() {
            let current_unit = polymer[n];
            // Detect and handle a reaction.
            if previous_unit.eq_ignore_ascii_case(&current_unit) && current_unit != previous_unit {
                polymer.remove(n);
                polymer.remove(n - 1);
                n = if n > 1 { n - 2 } else { 0 };
                continue;
            }
            n += 1;
            previous_unit = current_unit;
        }
        // Record new polymer's length.
        efficiency.insert(removed_unit, polymer.len());
    }
    *efficiency.values().min().unwrap()
}

fn main() {
    let input = read_file("inputs/05");
    println!("Welcome to Aperture Science Polymer Reaction, Scanning and Optimisation Chamber!");
    println!(
        "Processing a polymer with starting length of {} units.",
        input.len()
    );
    let polymer = parse_input(&input);
    let final_polymer_length = part1(&polymer);
    assert_eq!(final_polymer_length, 10878);
    println!("Final polymer is {} units long", final_polymer_length);

    let shortest_polymer_length = part2(&polymer);
    assert_eq!(shortest_polymer_length, 6874);
    println!(
        "Shortest polymer is {} units long.",
        shortest_polymer_length
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"dabAcCaCBAcCcaDA"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 4);
    }
}
