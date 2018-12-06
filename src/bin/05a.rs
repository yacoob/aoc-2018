fn main() {
    let input = include_str!("../../inputs/05").trim();
    println!("Welcome to Aperture Science Polymer Reaction and Scanning Chamber!");
    println!("Processing a polymer with starting length of {} units.", input.len());

    // Go through the polymer left to right, check characters, add them to final_polymer as
    // if appropriate.
    let mut final_polymer: Vec<char> = Vec::new();
    for unit in input.chars() {
        // Check last unit of the final_polymer (will be None on empty polymer), compare to
        // current unit.
        if final_polymer.last().map(|&previous| previous.eq_ignore_ascii_case(&unit) && previous != unit).unwrap_or(false) {
            final_polymer.pop();
        } else {
            final_polymer.push(unit);
        }
    }

    println!("Final polymer is {} units long", final_polymer.len());
}
