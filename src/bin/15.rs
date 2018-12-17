use aoc::*;
use std::collections::VecDeque;
use std::fmt;

struct Combatant {
    position: Point,
    faction: char,
    hp: isize,
    ap: isize,
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
    clock: usize,
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
                    '#' | '.' => (),
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
        Arena {
            grid,
            units,
            clock: 0,
        }
    }
    fn distances_and_origins(&self, start: Point) -> (Vec<Vec<usize>>, Vec<Vec<Option<usize>>>) {
        // Flood fill from unit's position in a breadth-first manner.
        // For every arena tile reachable from current unit's position, we'll find the distance
        // from the unit and which of the current unit's neighbouring tiles it was reached
        // from.
        let mut distances = vec![vec![std::usize::MAX; self.grid[0].len()]; self.grid.len()];
        let mut origins = vec![vec![None; self.grid[0].len()]; self.grid.len()];
        // queue will hold all points that we need to consider.
        let mut queue = VecDeque::new();
        // Add starting point to the queue.
        distances[start.y][start.x] = 0;
        origins[start.y][start.x] = None;
        queue.push_back((start, 0));
        while !queue.is_empty() {
            let (current_point, current_distance) = queue.pop_front().unwrap();
            // Get all neighbours of the point being considered, in the "reading order".
            for (origin, &neighbour) in neighbours_in_reading_order(&current_point)
                .iter()
                .enumerate()
            {
                // Is that neighbour a free tile? Have we already seen it and produced
                // a distance for it?
                if self.grid[neighbour.y][neighbour.x] != '.'
                    || distances[neighbour.y][neighbour.x] < std::usize::MAX
                {
                    continue;
                }
                // New free tile! Record its distance, and queue it for further investigation.
                distances[neighbour.y][neighbour.x] = current_distance + 1;
                queue.push_back((neighbour, current_distance + 1));
                // Record where have we arrive from, for this neighbour.
                // Is this a neighbour of unit's starting position?
                if current_point == start {
                    // Yes; assign orgin according to the visiting order.
                    origins[neighbour.y][neighbour.x] = Some(origin);
                } else {
                    origins[neighbour.y][neighbour.x] = origins[current_point.y][current_point.x];
                }
            }
        }
        (distances, origins)
    }

    fn tick(&mut self) -> bool {
        // Order units by "reading order".
        self.units.sort_by_key(|u| (u.position.y, u.position.x));
        // We can't use an iterator here, as borrowchecker would complain about read-only access to
        // self.units later inside this loop.
        for i in 0..self.units.len() {
            // Is this unit still alive?
            if self.units[i].hp < 0 {
                continue;
            }

            let unit_faction = self.units[i].faction;

            // Find all of this unit's enemies.
            let enemies: Vec<&Combatant> = self
                .units
                .iter()
                .filter(|u| u.faction != unit_faction && u.hp > 0)
                .collect();

            // Are there any? Maybe we're done?
            if enemies.len() == 0 {
                return false;
            }

            // Find all target tiles near enemies. At this point, it'll include unreachable spots
            // and spots that are occupied by a unit/wall.
            let mut target_tiles = vec![];
            for enemy in &enemies {
                target_tiles.append(&mut enemy.position.neighbours());
            }
            let mut unit_position = self.units[i].position;
            let (distances, origins) = self.distances_and_origins(unit_position);

            // Pick the target: closest, and first in "reading order".
            // By now unreachable targets will have distance == usize::MAX.
            target_tiles.sort_by_key(|p| (distances[p.y][p.x], origins[p.y][p.x]));
            let target_tile = target_tiles[0];

            // Move if feasible.
            let target_distance = distances[target_tile.y][target_tile.x];
            if target_distance > 0 && target_distance < std::usize::MAX {
                println!(
                    "Moving {} from {:?} to {:?}",
                    unit_faction, unit_position, target_tile
                );
                let (x, y) = match origins[target_tile.y][target_tile.x] {
                    Some(0) => (unit_position.x, unit_position.y - 1), // above
                    Some(1) => (unit_position.x - 1, unit_position.y), // left
                    Some(2) => (unit_position.x + 1, unit_position.y), // right
                    Some(3) => (unit_position.x, unit_position.y + 1), // below
                    Some(_) | None => panic!("Unexpected an origin to move towards."),
                };
                self.grid[unit_position.y][unit_position.x] = '.';
                self.grid[y][x] = unit_faction;
                self.units[i].position = Point::new(x, y);
                unit_position = self.units[i].position;
            }

            // Check for targets around unit after the move.
            let ap = self.units[i].ap;
            let mut targets: Vec<_> = self
                .units
                .iter_mut()
                .filter(|u| {
                    u.faction != unit_faction
                        && u.hp > 0
                        && unit_position.distance_to(u.position) == 1
                })
                .collect();

            // Damage the weakest.
            if targets.len() > 0 {
                targets.sort_by_key(|u| u.hp);
                println!(
                    "{} damages enemy at {:?}",
                    unit_faction, targets[0].position
                );
                targets[0].hp -= ap;
                if targets[0].hp <= 0 {
                    self.grid[targets[0].position.y][targets[0].position.x] = '.';
                }
            }
        }
        // Tick!
        self.clock += 1;
        println!("{:?}", self);
        true
    }
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = Ok(());
        result = result.or(writeln!(f, "Turn {}", self.clock));
        for (y, line) in self.grid.iter().enumerate() {
            result = result.or(write!(f, "{}   ", line.iter().collect::<String>()));
            let mut units_on_this_line: Vec<&Combatant> = self
                .units
                .iter()
                .filter(|u| u.position.y == y && u.hp > 0)
                .collect();
            units_on_this_line.sort_by_key(|u| u.position.x);
            for unit in units_on_this_line {
                result = result.or(write!(f, "{:?} ", unit));
            }
            result = result.or(writeln!(f, ""));
        }
        result
    }
}

fn neighbours_in_reading_order(p: &Point) -> Vec<Point> {
    // This is a replica of Point::neighbours, with the ordering of the points reflecting this
    // puzzle's desired ordering ("reading order"). Didn't want to make a cross-file assumption
    // without a way to ensure it.
    vec![
        Point::new(p.x, p.y - 1),
        Point::new(p.x - 1, p.y),
        Point::new(p.x + 1, p.y),
        Point::new(p.x, p.y + 1),
    ]
}

fn part1(arena: &mut Arena) -> usize {
    while arena.tick() {}
    let hp_sum: usize = arena
        .units
        .iter()
        .filter_map(|u| if u.hp > 0 { Some(u.hp as usize) } else { None })
        .sum();
    hp_sum * arena.clock
}

fn main() {
    let INPUT = r#"#######
                   #.G...#
                   #...EG#
                   #.#.#G#
                   #..G#E#
                   #.....#
                   #######"#;
    // let mut arena = Arena::from_str(&read_file("inputs/15"));
    let mut arena = Arena::from_str(INPUT);
    let answer1 = part1(&mut arena);
    // assert_eq!(answer1, 3671);
    println!("Part 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"#######
                       #.G...#
                       #...EG#
                       #.#.#G#
                       #..G#E#
                       #.....#
                       #######"#;
        assert_eq!(part1(&mut Arena::from_str(input)), 27730);
        let input = r#"#######
                       #G..#E#
                       #E#E.E#
                       #G.##.#
                       #...#E#
                       #...E.#
                       #######"#;
        assert_eq!(part1(&mut Arena::from_str(input)), 36334);
        let input = r#"#######
                       #E..EG#
                       #.#G.E#
                       #E.##E#
                       #G..#.#
                       #..E#.#
                       #######"#;
        assert_eq!(part1(&mut Arena::from_str(input)), 39514);

        let input = r#"#######
                       #E.G#.#
                       #.#G..#
                       #G.#.G#
                       #G..#.#
                       #...E.#
                       #######"#;
        assert_eq!(part1(&mut Arena::from_str(input)), 27755);

        let input = r#"#######
                       #.E...#
                       #.#..G#
                       #.###.#
                       #E#G#G#
                       #...#G#
                       #######"#;
        assert_eq!(part1(&mut Arena::from_str(input)), 28944);

        let input = r#"#########
                       #G......#
                       #.E.#...#
                       #..##..G#
                       #...##..#
                       #...#...#
                       #.G...G.#
                       #.....G.#
                       #########"#;
        assert_eq!(part1(&mut Arena::from_str(input)), 18740);
    }
}
