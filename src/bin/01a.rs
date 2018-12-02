fn main() {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
    for change in changes {
        let delta: i32 = change.parse().unwrap();
        current_frequency += delta;
    }
    println!("Final frequency: {}", current_frequency);
}
