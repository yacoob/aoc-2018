use aoc::*;

struct Data {
    length: usize,
}

fn parse_input(input: &str) -> Data {
    Data {
        length: input.len(),
    }
}

fn part1(data: &Data) -> usize {
    data.length
}

fn main() {
    let data = parse_input(&read_file("inputs/01"));

    let answer1 = part1(&data);
    assert_eq!(answer1, 3672);
    println!("Part 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
Well here we are again
It's always such a pleasure
Remember when you tried
to kill me twice?
"#;

    #[test]
    fn test_part1() {
        let lyrics = parse_input(INPUT);
        assert_eq!(part1(&lyrics), 94);
    }
}
