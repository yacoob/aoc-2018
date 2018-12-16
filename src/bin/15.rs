use aoc::*;
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

    fn enemy(&self) -> Faction {
        match self.faction {
            Faction::Elf => Faction::Goblin,
            Faction::Goblin => Faction::Elf,
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
    units: BTreeMap<Point, Combatant>,
    max_x: usize,
    max_y: usize,
}

impl Arena {
    fn from_str(input: &str) -> Arena {
        let mut field = vec![];
        let mut units = BTreeMap::new();
        for (y, line) in input.lines().enumerate() {
            field.push(vec![]);
            let current_line = &mut field[y];
            for (x, c) in line.trim().chars().enumerate() {
                match c {
                    c @ 'E' | c @ 'G' => {
                        current_line.push('.');
                        units.insert(Point::new(x, y), Combatant::new(c));
                    }
                    c @ '#' | c @ '.' => current_line.push(c),
                    c => panic!("unexpected character {} seen in arena", c),
                }
            }
        }
        // Does arena exist?
        assert!(field.len() > 0);
        // Are all lines of equal length?
        let max_x = field[0].len() - 1;
        let max_y = field.len() - 1;
        assert!(field.iter().all(|l| l.len() == max_x + 1));
        Arena {
            field,
            units,
            max_y,
            max_x,
        }
    }

    fn is_free(&self, point: &Point) -> bool {
        if self.units.contains_key(point) {
            return false;
        };
        if self.field[point.y][point.x] != '.' {
            return false;
        };
        true
    }

    fn free_spots_around(&self, point: &Point) -> Vec<Point> {
        let mut targets = vec![];
        for p in point.neighbours(self.max_x, self.max_y) {
            if self.is_free(&p) {
                targets.push(p);
            }
        }
        // eprintln!("Free spots for {:?}: {:?}", point, targets);
        targets
    }

    fn all_enemies(&self, point: &Point) -> Vec<Point> {
        eprintln!("{:?}", point);
        let unit = self.units.get(point);
        assert!(unit.is_some());
        let other_faction = unit.unwrap().enemy();
        self.units
            .iter()
            .filter_map(|(p, u)| {
                if u.faction == other_faction {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn maybe_move_unit_at(&self, point: &Point) -> Vec<Point> {
        // Check nearest surroundings for enemies.
        //
        // Need to move.
        // Determine all target squares.
        let mut targets = vec![];
        for enemy in self.all_enemies(point) {
            eprintln!("Considering enemy at {:?}", enemy);
            targets.append(&mut self.free_spots_around(&enemy))
        }
        // Determine all paths towards target squares.
        let mut paths = vec![];
        for target in &targets {
            let path = bfs(point, |p| self.free_spots_around(&p), |p| p == target);
            paths.push(path);
            // if path.is_some() {
            //     let mut path = path.unwrap();
            //     path.remove(0);
            //     paths.push(path);
            // }
        }
        eprintln!("paths: {:?}", paths);
        vec![Point::new(0, 0)]
    }
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for (y, line) in self.field.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                // let output = *c;
                let output = if self.units.contains_key(&Point::new(x, y)) {
                    self.units.get(&Point::new(x, y)).unwrap().faction.as_char()
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

fn part1(arena: &mut Arena) -> i32 {
    loop {
        for position in arena.units.keys() {
            eprintln!(
                "{:?} from {:?} ",
                arena.units.get(position).unwrap(),
                position
            );
            // identify all enemy units
            // identify all target squares
            arena.maybe_move_unit_at(&position);

            // work out all paths towards those squares
            // pick the shortest one
            // move
            // check for targets around
            // deal damage
        }
        break;
    }
    42
}

fn main() {
    let mut arena = Arena::from_str(&read_file("inputs/15"));
    println!("{:?}", arena);
    let answer1 = part1(&mut arena);
    // assert_eq!(answer1, 3671);
    // println!("Part 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let INPUT = r#"#######
                       #.....#
                       #...E.#
                       #...G.#
                       #######"#;
        let mut arena = Arena::from_str(INPUT);
        assert_eq!(arena.maybe_move_unit_at(&Point::new(4, 2)), vec![]);
    }
}
