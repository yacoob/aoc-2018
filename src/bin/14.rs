use std::char;

fn part1(input: usize) -> String {
    let mut scores = vec![3, 7];
    let mut favourites = vec![0, 1];

    while scores.len() < input + 10 {
        let favourite1 = scores[favourites[0]];
        let favourite2 = scores[favourites[1]];
        let mix = favourite1 + favourite2;
        assert!(mix < 19);
        if mix / 10 > 0 {
            scores.push(mix / 10);
        }
        scores.push(mix % 10);
        favourites[0] = (favourites[0] + favourite1 + 1) % scores.len();
        favourites[1] = (favourites[1] + favourite2 + 1) % scores.len();
    }
    let mut answer = String::new();
    for i in input..input + 10 {
        answer.push(char::from_digit(scores[i] as u32, 10).unwrap());
    }
    answer
}

fn main() {
    let input = 74501;
    let answer1 = part1(input);
    assert_eq!(answer1, "1464411010");
    println!("Part 1: {}", answer1);

    // let answer2 = part2(&foo);
    // assert_eq!(answer2, 3671);
    // println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(9), "5158916779");
        assert_eq!(part1(5), "0124515891");
        assert_eq!(part1(18), "9251071085");
        assert_eq!(part1(2018), "5941429882");
    }

    // #[test]
    // fn test_part2() {
    //     let lyrics = parse_input(INPUT);
    //     assert_eq!(part2(&lyrics), 94);
    // }
}
