use aoc::*;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().map(|l| l.trim()).collect()
}

fn part1(box_ids: &[&str]) -> (i32, i32) {
    let mut two_repeats = 0;
    let mut three_repeats = 0;
    for id in box_ids.iter() {
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
    (two_repeats, three_repeats)
}

fn contain_prototype_fabric(a: &str, b: &str) -> Option<char> {
    assert_eq!(a.len(), b.len());
    let mut different_character: char = '!';
    let mut number_of_differences = 0;
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char != b_char {
            number_of_differences += 1;
            different_character = a_char;
        }
    }
    match number_of_differences {
        0 => panic!("Boxes {} and {} seem to be equal!", a, b),
        1 => Some(different_character),
        _ => None,
    }
}

fn part2(box_ids: &[&str]) -> Option<String> {
    for (pos, id_a) in box_ids.iter().enumerate() {
        for id_b in box_ids[pos + 1..].iter() {
            match contain_prototype_fabric(id_a, id_b) {
                None => continue,
                Some(c) => {
                    let tmp = id_a.replace(c, "").clone();
                    return Some(tmp);
                }
            }
        }
    }
    None
}

fn main() {
    let input = &read_file("inputs/02");
    let box_ids = parse_input(input);
    let (two_repeats, three_repeats) = part1(&box_ids);
    let checksum = two_repeats * three_repeats;
    assert_eq!(checksum, 8398);
    println!("Checksum: {}", checksum);

    let remaining_characters = part2(&box_ids).unwrap();
    assert_eq!(remaining_characters, "hhvsdkatysmiqjxunezgwcdpr");
    println!(
        "Found two boxes containing the suit. Common characters between their ids: {}",
        remaining_characters
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"#;
        assert_eq!(part1(&parse_input(input)), (4, 3));
    }

    #[test]
    fn test_part2() {
        let input = r#"
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"#;
        println!("{}", part2(&parse_input(input)).unwrap());
        assert_eq!(part2(&parse_input(input)).unwrap(), "fgij");
    }
}
