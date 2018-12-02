fn main() {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    //
    // Probably worth carving those out to a shared piece; except I don't know how to do it yet :D
    let mut box_ids: Vec<String> = Vec::new();
    let f = File::open("inputs/02").expect("Can't open input file!");
    let buffered = BufReader::new(&f);
    for line in buffered.lines() {
        box_ids.push(line.unwrap());
    }

    let mut two_repeats = 0;
    let mut three_repeats = 0;
    for id in box_ids {
        let mut seen_letters: HashMap<char, i32> = HashMap::new();
        for letter in id.chars() {
            let tmp = seen_letters.entry(letter).or_insert(0);
            *tmp += 1;
        }
        // I feel so procedural here. :D
        if seen_letters.values().any(|&x| x == 2) {
            two_repeats += 1;
        }
        if seen_letters.values().any(|&x| x == 3) {
            three_repeats += 1;
        }
    }
    println!(
        "{} boxes with two repeats, {} boxes with three repeats",
        two_repeats, three_repeats
    );
    println!("Checksum: {}", two_repeats * three_repeats);
}
