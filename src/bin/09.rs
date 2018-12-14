use aoc::*;
use std::collections::VecDeque;

const PUZZLE_INPUT: &str = "473 players; last marble is worth 70904 points";
const LUCKY_NUMBER: usize = 23;

#[derive(Debug, Clone)]
// We will use this struct to keep the game state.
struct Game {
    player_count: usize,
    game_length: usize,
    turn: usize,
    // First implementation used a Vec for circle storage, but also used insert(i)/remove(i), which
    // are O(n-i). VecDeque's push()/pop() are O(1). Let's assert that the
    // current ball is circle[0], and use push/pop for "rotating" VecDeque to reflect current ball
    // changes.
    circle: VecDeque<usize>,
    scores: Vec<usize>,
}

impl Game {
    fn new(player_count: usize, game_length: usize) -> Game {
        let mut g = Game {
            player_count,
            game_length,
            turn: 0,
            circle: VecDeque::with_capacity(game_length),
            scores: vec![0; player_count],
        };
        g.circle.push_front(0);
        g
    }

    // Move elements from front to back of circle.
    fn rotate_cw(&mut self, steps: usize) {
        for _ in 1..=steps {
            let tmp = self.circle.pop_front().unwrap();
            self.circle.push_back(tmp);
        }
    }

    // Move elements from back to front of the circle.
    fn rotate_ccw(&mut self, steps: usize) {
        for _ in 1..=steps {
            let tmp = self.circle.pop_back().unwrap();
            self.circle.push_front(tmp);
        }
    }

    fn place_next_ball(&mut self) {
        // Next turn has started!
        self.turn += 1;
        // Newly placed ball has the same number on its face as the turn number.
        let new_ball = self.turn;
        if new_ball % LUCKY_NUMBER == 0 {
            // Ball being placed is a multiple of LUCKY_NUMBER!
            // Move current 7 positions counterclockwise, remove it.
            self.rotate_ccw(7);
            let removed_ball = self.circle.pop_front().unwrap();
            let lucky_player = (self.turn - 1) % self.player_count;
            self.scores[lucky_player] += new_ball + removed_ball;
        } else {
            // Normal ball: move current 2 positions clockwise, insert new ball at the front.
            self.rotate_cw(2);
            self.circle.push_front(new_ball);
        }
    }

    // https://youtu.be/6_5O-nUiZ_0 :3
    fn play_the(&mut self) -> usize {
        let stopwatch = Stopwatch::start();
        for _ in 1..=self.game_length {
            self.place_next_ball();
        }
        stopwatch.split();
        *self.scores.iter().max().unwrap()
    }
}

fn parse_input(input: &str) -> Game {
    let words: Vec<&str> = input.split_whitespace().collect();
    Game::new(
        words[0].parse::<usize>().unwrap(),
        words[6].parse::<usize>().unwrap(),
    )
}

fn main() {
    let mut game = parse_input(&PUZZLE_INPUT);
    let highscore = game.clone().play_the();
    assert_eq!(highscore, 371_284);
    println!("Winning Elf's high score: {}", highscore);

    game.game_length *= 100;
    let highscore = game.play_the();
    assert_eq!(highscore, 3_038_972_494);
    println!(
        "Winning Elf's high score when we're playing for much longer: {}",
        highscore
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            parse_input("9 players; last marble is worth 25 points").play_the(),
            32,
        );
        assert_eq!(
            parse_input("10 players; last marble is worth 1618 points").play_the(),
            8317,
        );
        assert_eq!(
            parse_input("13 players; last marble is worth 7999 points").play_the(),
            146373,
        );
        assert_eq!(
            parse_input("17 players; last marble is worth 1104 points").play_the(),
            2764,
        );
        assert_eq!(
            parse_input("21 players; last marble is worth 6111 points").play_the(),
            54718,
        );
        assert_eq!(
            parse_input("30 players; last marble is worth 5807 points").play_the(),
            37305,
        );
    }
}
