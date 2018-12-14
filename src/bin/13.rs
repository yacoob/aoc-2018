use self::Direction::*;
use aoc::*;
use std::cmp::Ordering;
use std::collections::hash_map::{Entry, HashMap};

const TRACKS_SIZE: usize = 150;
const TURNS: [Direction; 4] = [Up, Right, Down, Left];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cart {
    id: usize,
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: Option<Direction>,
}

#[derive(Clone, Debug)]
struct Mine {
    clock: i32,
    tracks: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            id,
            x,
            y,
            direction,
            next_turn: Some(Direction::Left),
        }
    }

    fn turn(&mut self) {
        // Rotate the cart.
        let direction_change = match &self.next_turn {
            None => 0,
            Some(Left) => 3,
            Some(Right) => 1,
            Some(d) => panic!("next_turn={:?} and it really shouldn't!", d),
        };
        let idx = TURNS.iter().position(|x| x == &self.direction).unwrap();
        self.direction = TURNS[(idx + direction_change) % TURNS.len()];

        self.next_turn = match self.next_turn {
            Some(Left) => None,
            None => Some(Right),
            Some(Right) => Some(Left),
            Some(d) => panic!("Cart wants to turn {:?} and it really shouldn't!", d),
        }
    }
}

// Comparison functions for carts; we use cart's coords to establish sorting order.
impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        let c = self.y.cmp(&other.y);
        match c {
            Ordering::Equal => self.x.cmp(&other.x),
            _ => c,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Mine {
    let mut mine = Mine {
        clock: 0,
        tracks: vec![vec![' '; TRACKS_SIZE]; TRACKS_SIZE],
        carts: vec![],
    };
    let mut x;
    for (y, line) in input.lines().enumerate() {
        x = 0;
        for c in line.chars() {
            match c {
                '^' => {
                    mine.tracks[x][y] = '|';
                    mine.carts.push(Cart::new(mine.carts.len(), x, y, Up))
                }
                '>' => {
                    mine.tracks[x][y] = '-';
                    mine.carts.push(Cart::new(mine.carts.len(), x, y, Right))
                }
                'v' => {
                    mine.tracks[x][y] = '|';
                    mine.carts.push(Cart::new(mine.carts.len(), x, y, Down))
                }
                '<' => {
                    mine.tracks[x][y] = '-';
                    mine.carts.push(Cart::new(mine.carts.len(), x, y, Left))
                }
                c => mine.tracks[x][y] = c,
            }
            x += 1;
        }
    }
    mine
}

fn joyride(mine: &mut Mine, stop_at_first_crash: bool) -> (usize, usize) {
    // Save current cart positions; we'll use it for crash detection later.
    // We also use cart_positions to cache removal of crashed carts. We can't do that straight away
    // while looping through carts, as borrow checker protests loudly about borrowing mine.carts
    // mutably twice.
    let mut cart_positions: HashMap<(usize, usize), usize> =
        mine.carts.iter().map(|c| ((c.x, c.y), c.id)).collect();
    loop {
        // Tick goes the clock.
        mine.clock += 1;
        // Remove carts that crashed in previous tick from mine.carts. They're still present there,
        // but are already gone from cart_positions.
        mine.carts
            .retain(|c| cart_positions.contains_key(&(c.x, c.y)));
        assert_eq!(cart_positions.len(), mine.carts.len());
        // Are we down to last cart (part B)?
        if mine.carts.len() == 1 {
            return (mine.carts[0].x, mine.carts[0].y);
        }
        // Ensure we're reviewing carts top to bottom, left to right.
        mine.carts.sort();
        // Move all zig. :3
        for cart in &mut mine.carts {
            // Skip carts that got deleted earlier in this cycle, and that change isn't yet
            // reflected in mine.carts.
            if !cart_positions.contains_key(&(cart.x, cart.y)) {
                continue;
            }
            // What's the next tile for this cart?
            let (new_x, new_y) = match cart.direction {
                Up => (cart.x, cart.y - 1),
                Down => (cart.x, cart.y + 1),
                Left => (cart.x - 1, cart.y),
                Right => (cart.x + 1, cart.y),
            };
            let new_tile = mine.tracks[new_x][new_y];
            match new_tile {
                // Travelling horizontally. Verify.
                '-' => assert!(cart.direction == Left || cart.direction == Right),
                // Travelling vertically. Verify.
                '|' => assert!(cart.direction == Up || cart.direction == Down),
                // Forced turn.
                '\\' => {
                    cart.direction = match cart.direction {
                        Left => Up,
                        Right => Down,
                        Down => Right,
                        Up => Left,
                    }
                }
                // Forced turn.
                '/' => {
                    cart.direction = match cart.direction {
                        Left => Down,
                        Right => Up,
                        Up => Right,
                        Down => Left,
                    }
                }
                '+' => cart.turn(),
                // Out of tracks - shouldn't happen.
                ' ' => panic!(
                    "Cart {} fell out of tracks at ({},{})!",
                    cart.id, cart.x, cart.y
                ),
                // Something else entirely instead of empty space or tracks.
                c => panic!("Unexpected tile {} at ({},{})!", c, new_x, new_y),
            }
            // Collision check.
            // First, remove current cart from position cache. It's either moving to a new
            // position, or crashing into other cart.
            cart_positions.remove(&(cart.x, cart.y));
            // Is there another cart in the new tile?
            match cart_positions.entry((new_x, new_y)) {
                // Yes; remove the cart we've crashed into.
                Entry::Occupied(o) => {
                    o.remove();
                    // Maybe return, because we've only been waiting for a first crash (part A)?
                    if stop_at_first_crash {
                        return (new_x, new_y);
                    }
                }
                // No; update current cart's position, and insert it into cache.
                Entry::Vacant(o) => {
                    cart.x = new_x;
                    cart.y = new_y;
                    o.insert(cart.id);
                }
            };
        }
    }
}

fn main() {
    let mine = parse_input(&read_file("inputs/13"));
    let crash1 = joyride(&mut mine.clone(), true);
    assert_eq!(crash1, (83, 49));
    println!("First collision detected at: {:?}", crash1);

    let last1 = joyride(&mut mine.clone(), false);
    assert_eq!(last1, (73, 36));
    println!("Last cart standing at: {:?}", last1);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
"#;
    #[test]
    fn test_joyride() {
        let mine = parse_input(INPUT);
        assert_eq!(joyride(&mut mine.clone(), true), (7, 3));
        assert_eq!(joyride(&mut mine.clone(), false), (6, 4));
    }
}
