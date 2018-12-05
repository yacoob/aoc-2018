use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/05").trim();
    let original_polymer: Vec<char> = input.chars().collect();
    println!("Welcome to Aperture Science Polymer Reaction, Scanning and Optimisation Chamber!");
    println!("Processing a polymer with starting length of {} units.", &input.len());

    let all_units: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut efficiency: HashMap<char, usize> = HashMap::new();
    for removed_unit in all_units {
        // Prepare a new polymer with specific units removed.
        let mut polymer = original_polymer.clone();
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
                polymer.remove(n-1);
                n = if n>1 { n-2 } else { 0 };
                continue;
            }
            n += 1;
            previous_unit = current_unit;
        }
        // Record new polymer's length.
        efficiency.insert(removed_unit, polymer.len());
    }

    println!("Shortest polymer is {} units long.", efficiency.values().min().unwrap());
}
