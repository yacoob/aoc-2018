use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/05");
    let original_polymer: Vec<char> = input.trim().chars().collect();
    println!("Welcome to Aperture Science Polymer Reaction, Scanning and Optimisation Chamber!");
    println!("Processing a polymer with starting length of {} units.", &input.len());

    let all_units: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut efficiency: HashMap<char, usize> = HashMap::new();
    for removed_unit in all_units {
        // Prepare a new polymer with specific units removed.
        let uppercase_removed_unit = removed_unit.to_ascii_uppercase();
        let mut polymer = original_polymer.clone();
        // I wonder if String::replace() on the input would be faster.
        polymer.retain(|&x| x != removed_unit && x != uppercase_removed_unit);

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
            // I wonder if there's a more concise way to write the comparisons here.
            if previous_unit.to_lowercase().to_string() == current_unit.to_lowercase().to_string() {
                if previous_unit.is_lowercase() && current_unit.is_uppercase() || previous_unit.is_uppercase() && current_unit.is_lowercase() {
                    polymer.remove(n);
                    polymer.remove(n-1);
                    n = if n>1 { n-2 } else { 0 };
                    continue;
                }
            }
            n += 1;
            previous_unit = current_unit;
        }
        // Record new polymer's length.
        efficiency.insert(removed_unit, polymer.len());
    }

    println!("Shortest polymer is {} units long.", efficiency.values().min().unwrap());
}
