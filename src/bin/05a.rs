fn main() {
    let input = include_str!("../../inputs/05");
    let mut polymer: Vec<char> = input.trim().chars().collect();
    println!("Welcome to Aperture Science Polymer Reaction and Scanning Chamber!");
    println!("Processing a polymer with starting length of {} units.", &polymer.len());

    let mut n = 1;
    let mut previous_unit = polymer[0];
    while n < polymer.len() {
        // eprintln!("> Polymer is currently {} units long.", polymer.len());
        // eprintln!(">  and looks like this: {}", polymer.iter().collect::<String>());
        // eprintln!("> Scanning position {}", n);
        let current_unit = polymer[n];
        // eprintln!("> previous unit: {}, current unit: {}", previous_unit, current_unit);
        if previous_unit.to_lowercase().to_string() == current_unit.to_lowercase().to_string() {
            if previous_unit.is_lowercase() && current_unit.is_uppercase() || previous_unit.is_uppercase() && current_unit.is_lowercase() {
                // eprintln!("!!!Reaction detected at position {}", n);
                polymer.remove(n);
                polymer.remove(n-1);
                n = if n>1 { n-2 } else { 0 };
                continue;
            }
        }
        n += 1;
        previous_unit = current_unit;
    }

    let final_polymer: String = polymer.iter().collect();
    println!("Final polymer is {} units long", final_polymer.len());
    // println!("and looks like this:\n{}", final_polymer);
}
