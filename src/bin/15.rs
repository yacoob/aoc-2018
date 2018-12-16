use aoc::*;
use std::fmt;

#[derive(PartialEq)]
struct Combatant {
    position: Point,
    faction: char,
    hp: isize,
    ap: usize,
}

impl Combatant {
    fn new(position: Point, faction: char) -> Combatant {
        Combatant {
            faction,
            position,
            hp: 200,
            ap: 3,
        }
    }
}

impl fmt::Debug for Combatant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}: ({:3>})]", self.faction, self.hp)
    }
}

struct Arena {
    grid: Vec<Vec<char>>,
    units: Vec<Combatant>,
}

impl Arena {
    fn from_str(input: &str) -> Arena {
        let mut grid = vec![];
        let mut units = vec![];
        for (y, line) in input.lines().enumerate() {
            let mut grid_line = vec![];
            for (x, c) in line.trim().chars().enumerate() {
                match c {
                    c @ 'E' | c @ 'G' => {
                        units.push(Combatant::new(Point::new(x, y), c));
                    }
                    c @ '#' | c @ '.' => (),
                    c => panic!("unexpected character {} seen in arena", c),
                }
                grid_line.push(c);
            }
            grid.push(grid_line);
        }
        // Does arena exist?
        assert!(grid.len() > 0);
        // Are all lines of equal length?
        let max_x = grid[0].len();
        assert!(grid.iter().all(|l| l.len() == max_x));
        Arena { grid, units }
    }
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        for (y, line) in self.grid.iter().enumerate() {
            result = result.or(writeln!(f, "{} ", line.iter().collect::<String>()));
            // for unit in units.filter(|u| u.position.y == y) {}
        }
        result
    }
}

fn part1(arena: &mut Arena) -> i32 {
    // identify all enemy units
    // identify all target squares
    // work out all paths towards those squares
    // pick the shortest one
    // move
    // check for targets around
    // deal damage
    42
}

fn main() {
    let mut arena = Arena::from_str(&read_file("inputs/15"));
    println!("{:?}", arena);
    // let answer1 = part1(&mut arena);
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
