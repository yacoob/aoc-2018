fn contain_prototype_fabric(a: &String, b: &String) -> Option<char> {
    assert_eq!(a.len(), b.len());
    let mut different_character: char = '!';
    let mut number_of_differences = 0;
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char != b_char {
            number_of_differences += 1;
            different_character = a_char;
        }
        if number_of_differences > 1 {
            return None;
        }
    }
    assert_eq!(number_of_differences, 1);
    return Some(different_character);
}

fn main() {
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

    'outer: for (pos, id_a) in box_ids.iter().enumerate() {
        for id_b in box_ids[pos + 1..].iter() {
            let z = contain_prototype_fabric(id_a, id_b);
            if z.is_some() {
                println!("Found boxes with the suit:\n{}\n{}", id_a, id_b);
                println!("They differ by character '{}'", z.unwrap());
                println!("Remaining characters: {}", id_a.replace(z.unwrap(), ""));
                break 'outer;
            }
        }
    }
}
