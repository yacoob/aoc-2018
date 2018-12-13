use aoc::*;
use std::collections::HashMap;
use std::fmt;

// We need to consider more space than just the starting state (pots 0 and above). Initially I was
// trying to pad the state dynamically from left and right with enough falses to have four of them,
// but then I've noticed that the state eventually stabilises and starts to "creep" towards one
// side of the array. As a result, I've used a simple Vec<bool> for the state, added a fixed
// padding in front (determined by OFFSET) and set the vector size to an eyeballed "big enough"
// number (MAX_POT_COUNT) to fit 20 gens or so.
const OFFSET: usize = 6;
const MAX_POT_COUNT: usize = 300; // to fit nicely on my terminal :3
const PATTERN_SIZE: usize = 5;

#[derive(Clone)]
struct Pots {
    state: Vec<bool>,
    offset: usize,
    growth: HashMap<[bool; 5], bool>,
}

impl fmt::Debug for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for &s in self.state.iter() {
            let c = if s { "#" } else { "." };
            result = result.or(write!(f, "{}", c))
        }
        result
    }
}

fn parse_input(input: &str) -> Pots {
    let mut i = input.trim().lines();
    // First line of input contains the state.
    let state_line = i.next().unwrap().split_whitespace().nth(2).unwrap();
    let mut state = vec![false; MAX_POT_COUNT];
    for (i, c) in state_line.chars().enumerate() {
        state[i + OFFSET] = c == '#'
    }
    i.next();

    // Third and below - growth patterns.
    let mut patterns = HashMap::new();
    for line in i {
        let words: Vec<_> = line.split(" => ").collect();
        let mut key = [false; PATTERN_SIZE];
        for (i, c) in words[0].chars().enumerate() {
            key[i] = c == '#';
        }
        patterns.insert(key, words[1] == "#");
    }
    // There should be exactly 2^5 patterns in the input, to cover all possibilities.
    assert_eq!(patterns.keys().len(), 32);
    Pots {
        state: state,
        offset: OFFSET,
        growth: patterns,
    }
}

fn grow(pots: &mut Pots, iterations: usize) -> usize {
    let mut sum = 0;
    let mut seen_patterns = HashMap::new();
    for generation in 1..=iterations {
        sum = 0;
        let mut first_true = None;
        let mut last_true = None;
        let mut next_state = vec![false; MAX_POT_COUNT];
        for (i, window) in pots.state.windows(PATTERN_SIZE).enumerate() {
            let output_position = i + PATTERN_SIZE / 2;
            let bloom = pots.growth[window];
            if bloom {
                sum += output_position - pots.offset;
                first_true = first_true.or(Some(i));
                last_true = Some(i);
            }
            next_state[output_position] = bloom;
        }

        // Puzzle input eventually generates a creeper that crawls towards right hand side. It has
        // constant shape, the only thing that changes is its positions - it's moving right. Carve
        // out the "relevant" pattern - from first true, to last true.
        let key = next_state[*first_true.get_or_insert(0)..*last_true.get_or_insert(MAX_POT_COUNT)]
            .to_owned();
        // Have we seen it yet?
        if !seen_patterns.contains_key(&key) {
            // No; record it together with a sum.
            seen_patterns.insert(key, sum);
        } else {
            // Yes; calculate the difference between current and past state.
            let difference = sum - seen_patterns.get(&key).unwrap();
            // Calculate the final result.
            sum += (iterations - generation) * difference;
            break;
        }

        pots.state = next_state;
    }
    sum
}

fn main() {
    let mut pots = parse_input(&read_file("inputs/12"));
    let sum20 = grow(&mut pots.clone(), 20);
    assert_eq!(sum20, 3248);
    println!(
        "Sum of numbers on pots with plants after 20 generations: {}",
        sum20
    );
    let sum_eternal = grow(&mut pots, 50_000_000_000);
    // let sum_eternal = grow(&mut pots, 110);
    assert_eq!(sum_eternal, 4_000_000_000_000);
    println!(
        "Sum of numbers on pots after a really long time: {}",
        sum_eternal
    );
}

// No tests, I'm too tired today. :E
