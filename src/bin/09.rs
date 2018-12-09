use ansi_term::Colour::Red;
use std::fmt;

const PUZZLE_INPUT: &str = "473 players; last marble is worth 70904 points";
const LUCKY_NUMBER: usize = 23;

#[derive(Debug, Clone)]
// We will use this struct to keep the game state.
struct Game {
    player_count: usize,
    game_length: usize,
    turn: usize,
    current: usize,
    circle: Vec<usize>,
    scores: Vec<usize>,
}

impl Game {
    fn new(player_count: usize, game_length: usize) -> Game {
        let mut g = Game {
            player_count,
            game_length,
            turn: 0,
            current: 0,
            circle: Vec::with_capacity(game_length),
            scores: vec![0; player_count],
        };
        g.circle.push(0);
        g
    }

    fn place_next_ball(&mut self) {
        // Next turn has started!
        self.turn += 1;
        // Newly placed ball has the same number on its face as the turn number.
        let new_ball = self.turn;
        if new_ball % LUCKY_NUMBER == 0 {
            // Ball being placed is a multiple of LUCKY_NUMBER!
            let lucky_player = (self.turn - 1) % self.player_count;
            self.scores[lucky_player] += new_ball;
            let mut ball_to_remove: i32 = self.current as i32 - 7;
            if ball_to_remove < 0 {
                ball_to_remove += self.circle.len() as i32;
            }
            assert!(ball_to_remove > 0);
            let ball_to_remove = ball_to_remove as usize;
            self.scores[lucky_player] += self.circle[ball_to_remove];
            self.circle.remove(ball_to_remove);
            self.current = ball_to_remove;
        } else {
            // Normal ball: go 1 position right, insert after that.
            let left_neighbour = (self.current + 1) % self.circle.len();
            if left_neighbour == self.circle.len() - 1 {
                self.circle.push(new_ball);
                self.current = self.circle.len() - 1;
            } else {
                self.circle.insert(left_neighbour + 1, new_ball);
                self.current = left_neighbour + 1;
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        result = result.or(write!(f, "[{:>2?}]   ", self.turn % self.player_count));
        for (position, marble) in self.circle.iter().enumerate() {
            result = result.or(write!(
                f,
                "{}  ",
                if position == self.current {
                    Red.paint(marble.to_string()).to_string()
                } else {
                    marble.to_string()
                }
            ));
        }
        result
    }
}

fn parse_input(input: &str) -> Game {
    let words: Vec<&str> = input.split_whitespace().collect();
    Game::new(
        words[0].parse::<usize>().unwrap(),
        words[6].parse::<usize>().unwrap(),
    )
}

fn part1(game: &mut Game) -> usize {
    let mut progress = 0;
    let tenth = game.game_length / 10;
    for i in 1..=game.game_length {
        let new_progress = i / tenth;
        if new_progress != progress {
            progress = new_progress;
            eprint!("...{}0%", progress);
        }
        game.place_next_ball();
    }
    eprintln!();
    *game.scores.iter().max().unwrap()
}

fn main() {
    let mut game = parse_input(&PUZZLE_INPUT);
    let answer1 = part1(&mut game.clone());
    assert_eq!(answer1, 371284);
    println!("Winning Elf's high score: {}", answer1);

    game.game_length *= 100;
    let answer2 = part1(&mut game);
    // assert_eq!(answer2, 3671);
    println!("What if?: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&mut parse_input(
                "9 players; last marble is worth 25 points"
            )),
            32,
        );
        assert_eq!(
            part1(&mut parse_input(
                "10 players; last marble is worth 1618 points"
            )),
            8317,
        );
        assert_eq!(
            part1(&mut parse_input(
                "13 players; last marble is worth 7999 points"
            )),
            146373,
        );
        assert_eq!(
            part1(&mut parse_input(
                "17 players; last marble is worth 1104 points"
            )),
            2764,
        );
        assert_eq!(
            part1(&mut parse_input(
                "21 players; last marble is worth 6111 points"
            )),
            54718,
        );
        assert_eq!(
            part1(&mut parse_input(
                "30 players; last marble is worth 5807 points"
            )),
            37305,
        );
    }
}
