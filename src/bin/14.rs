use std::char;

struct Kitchen {
    scores: Vec<usize>,
    favourites: Vec<usize>,
}

impl Kitchen {
    fn new() -> Kitchen {
        Kitchen {
            scores: vec![3, 7],
            favourites: vec![0, 1],
        }
    }

    fn brainstorm(&mut self) {
        let favourite1 = self.scores[self.favourites[0]];
        let favourite2 = self.scores[self.favourites[1]];
        let mix = favourite1 + favourite2;
        assert!(mix < 19);
        if mix / 10 > 0 {
            self.scores.push(mix / 10);
        }
        self.scores.push(mix % 10);
        self.favourites[0] = (self.favourites[0] + favourite1 + 1) % self.scores.len();
        self.favourites[1] = (self.favourites[1] + favourite2 + 1) % self.scores.len();
    }
}

fn part1(input: usize) -> String {
    let mut kitchen = Kitchen::new();
    while kitchen.scores.len() < input + 10 {
        kitchen.brainstorm();
    }
    kitchen.scores[input..input + 10]
        .iter()
        .map(|&s| char::from_digit(s as u32, 10).unwrap())
        .collect()
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
