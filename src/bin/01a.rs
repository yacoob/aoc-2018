fn main() {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let part_b = true;
    let starting_frequency = 0;
    let mut current_frequency = starting_frequency;

    // Read puzzle input. Aren't we optimistic that everything will work just fine? :D
    let mut changes: Vec<String> = Vec::new();
    let f = File::open("inputs/01").unwrap();
    let buffered = BufReader::new(&f);
    for line in buffered.lines() {
        changes.push(line.unwrap());
    }

    // Iterate through frequency changes:
    //  part A: find the frequency after applying entire list of adjustments;
    //  part B: find the frequency which is reached for a second time first, provided we keep
    //  applying the changes.
    let mut seen_frequencies: HashMap<i32, bool> = HashMap::new();
    seen_frequencies.insert(starting_frequency, true);
    'outer: loop {
        // "normal" iterator prevented from going over changes more than once.
        for change in &changes {
            let delta: i32 = change.parse().unwrap();
            current_frequency += delta;
            if part_b && seen_frequencies.contains_key(&current_frequency) {
                println!(
                    "Frequency {} reached for the second time!",
                    current_frequency
                );
                break 'outer;
            } else {
                seen_frequencies.insert(current_frequency, true);
            }
        }
        if !part_b {
            println!("Final frequency: {}", current_frequency);
            break 'outer;
        }
    }
}
