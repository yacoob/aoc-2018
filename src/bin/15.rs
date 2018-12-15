use aoc::*;
use std::fmt;

#[derive(Debug)]
enum Faction {
    Elf,
    Goblin,
}

#[derive(Debug)]
struct Combatant {
    id: usize,
    position: (usize, usize),
    hp: isize,
    ap: usize,
    faction: Faction,
}

impl Combatant {
    fn new(id: usize, (x, y): (usize, usize), faction: Faction) -> Combatant {
        Combatant {
            id,
            faction,
            position: (x, y),
            hp: 200,
            ap: 3,
        }
    }

    fn maybe_kill(&self, other: &mut Combatant) -> bool {
        other.hp -= self.ap as isize;
        if other.hp < 0 {
            true
        } else {
            false
        }
    }
}

struct Arena {
    field: Vec<Vec<char>>,
    elves: Vec<Combatant>,
    goblins: Vec<Combatant>,
}

impl Arena {
    fn from_str(input: &str) -> Arena {
        let mut field = vec![];
        let mut elves = vec![];
        let mut goblins = vec![];
        for (y, line) in input.lines().enumerate() {
            field.push(vec![]);
            let current_line = &mut field[y];
            for (x, c) in line.chars().enumerate() {
                match c {
                    'G' => {
                        goblins.push(Combatant::new(goblins.len(), (x, y), Faction::Goblin));
                    }
                    'E' => {
                        elves.push(Combatant::new(elves.len(), (x, y), Faction::Elf));
                    }
                    '#' | '.' => (),
                    c => panic!("unexpected character {} seen in arena", c),
                }
                current_line.push(c);
            }
        }
        Arena {
            field,
            elves,
            goblins,
        }
    }
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for line in self.field.iter() {
            result = result.or(write!(f, "{}\n", line.iter().collect::<String>()));
        }
        result = result.or(write!(f, "E: "));
        for e in &self.elves {
            result = result.or(write!(f, "[{}: {:3>}] ", e.id, e.hp));
        }
        result = result.or(write!(f, "\nG: "));
        for g in &self.goblins {
            result = result.or(write!(f, "[{}: {:3>}] ", g.id, g.hp));
        }
        result = result.or(write!(f, "\n"));
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
