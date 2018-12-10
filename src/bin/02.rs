use aoc::*;

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(foo: &i32) -> i32 {
    *foo
}

// fn part2(foo: &i32) -> i32 {
//     *foo
// }

fn main() {
    let filename = "inputs/02";
    let input = read_file(filename);
    let foo = parse_input(&input);

    let checksum = part1(parse_input(&read_file("inputs/02"));
    // assert_eq!(answer1, 3671);
    println!("Part 1: {}", answer1);

    // let answer2 = part2(&foo);
    // assert_eq!(answer2, 3671);
    // println!("Part 2: {}", answer2);
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

    // #[test]
    // fn test_part2() {
    //     let lyrics = parse_input(INPUT);
    //     assert_eq!(part2(&lyrics), 94);
    // }
}
