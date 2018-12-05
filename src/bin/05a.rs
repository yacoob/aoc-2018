fn main() {
    let input = include_str!("../../inputs/05");
    let mut polymer: Vec<char> = input.trim().chars().collect();
    println!("Welcome to Aperture Science Polymer Reaction and Scanning Chamber!");
    println!("Processing a polymer with starting length of {} units.", &polymer.len());

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
        // No reaction at this position, advance scan position.
        n += 1;
        previous_unit = current_unit;
    }

    let final_polymer: String = polymer.iter().collect();
    println!("Final polymer is {} units long", final_polymer.len());
}
