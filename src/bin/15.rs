use aoc::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
enum Faction {
    Elf,
    Goblin,
}

impl Faction {
    fn as_char(&self) -> char {
        match self {
            Faction::Elf => 'E',
            Faction::Goblin => 'G',
        }
    }

    fn from_char(c: char) -> Faction {
        match c {
            'E' => Faction::Elf,
            'G' => Faction::Goblin,
            _ => panic!("Only Elves and Goblins are allowed in the arena!"),
        }
    }
}

#[derive(PartialEq)]
struct Combatant {
    hp: isize,
    ap: usize,
    faction: Faction,
}

impl Combatant {
    fn new(c: char) -> Combatant {
        Combatant {
            faction: Faction::from_char(c),
            hp: 200,
            ap: 3,
        }
    }

    fn maybe_kill(&self, other: &mut Combatant) -> bool {
        // No friendly fire!
        assert!(self.faction != other.faction);
        other.hp -= self.ap as isize;
        if other.hp < 0 {
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Combatant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: ({:3>})]", self.faction.as_char(), self.hp)
    }
}

struct Arena {
    field: Vec<Vec<char>>,
    units: HashMap<(usize, usize), Combatant>,
}

impl Arena {
    fn from_str(input: &str) -> Arena {
        let mut field = vec![];
        let mut units = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            field.push(vec![]);
            let current_line = &mut field[y];
            for (x, c) in line.chars().enumerate() {
                match c {
                    c @ 'E' | c @ 'G' => {
                        current_line.push('.');
                        units.insert((x, y), Combatant::new(c));
                    }
                    c @ '#' | c @ '.' => current_line.push(c),
                    c => panic!("unexpected character {} seen in arena", c),
                }
            }
        }
        Arena { field, units }
    }
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for (y, line) in self.field.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                // let output = *c;
                let output = if self.units.contains_key(&(x, y)) {
                    self.units.get(&(x, y)).unwrap().faction.as_char()
                } else {
                    *c
                };
                result = result.or(write!(f, "{}", output));
            }
            result = result.or(write!(f, "\n"));
        }
        for unit in self.units.values() {
            result = result.or(write!(f, "{:?} ", unit))
        }
        result
    }
}

fn part1(foo: &i32) -> i32 {
    *foo
}

// fn part2(foo: &i32) -> i32 {
//     *foo
// }

fn main() {
    let arena = Arena::from_str(&read_file("inputs/15"));
    println!("{:?}", arena);
    // let answer1 = part1(&foo);
    // assert_eq!(answer1, 3671);
    // println!("Part 1: {}", answer1);

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
