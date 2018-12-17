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
    fn new(position: Point, faction: char, ap: isize) -> Combatant {
        Combatant {
            faction,
            position,
            ap,
            hp: 200,
        }
    }
}

// Pretty printer for the unit.
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
    // Input parsing. We keep units positions twice, once in self.units[i].position, and as E/G
    // character in self.grid. It's a redundancy, but makes obstacles checking shorter.
    fn from_str(input: &str, elf_power: isize) -> Arena {
        let mut grid = vec![];
        let mut units = vec![];
        for (y, line) in input.lines().enumerate() {
            let mut grid_line = vec![];
            for (x, c) in line.trim().chars().enumerate() {
                match c {
                    'E' => units.push(Combatant::new(Point::new(x, y), 'E', elf_power)),
                    'G' => units.push(Combatant::new(Point::new(x, y), 'G', 3)),
                    '#' | '.' => (),
                    c => panic!("unexpected character {} seen in arena", c),
                }
                grid_line.push(c);
            }
            grid.push(grid_line);
        }
        // Sanity checking for the arena.
        // Did we manage to parse sensible amount of lines?
        assert!(grid.len() >= 2);
        // Are all lines of equal length?
        let max_x = grid[0].len();
        assert!(grid.iter().all(|l| l.len() == max_x));
        // Is the arena an enclosed space? If yes, it'll save us on checking for out of bounds
        // positions in neighbours_in_reading_order().
        assert!(grid[0].iter().all(|&c| c == '#'));
        assert!(grid[grid.len() - 1].iter().all(|&c| c == '#'));
        assert!(grid.iter().map(|r| r[0]).all(|c| c == '#'));
        assert!(grid.iter().map(|r| r[grid[0].len() - 1]).all(|c| c == '#'));
        Arena {
            grid,
            units,
            clock: 0,
        }
    }

    // Flood fill from unit's position in a breadth-first manner.
    // For every arena tile reachable from current unit's position, we'll find the distance
    // from the unit and which of the current unit's neighbouring tiles it was reached
    // from.
    fn distances_and_origins(&self, start: Point) -> (Vec<Vec<usize>>, Vec<Vec<Option<usize>>>) {
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
            // By doing this here, the distance reflected in distances will always be the shortest
            // path with its first step being first in "reading order".
            //
            // "The unit then takes a single step toward the chosen square along the shortest path
            // to that square. If multiple steps would put the unit equally closer to its
            // destination, the unit chooses the step which is first in reading order."
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

    // Calculates single round: movement and combat of each unit. Returns false once the battle is
    // over and one of the factions has been eradicated.
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

            // Find all of this unit's enemies.
            let unit_faction = self.units[i].faction;
            let enemies: Vec<&Combatant> = self
                .units
                .iter()
                .filter(|u| u.faction != unit_faction && u.hp > 0)
                .collect();

            // Are there any enemies? Maybe the bloody fight is over?
            if enemies.len() == 0 {
                return false;
            }

            // Find all target tiles near enemies. At this point, it'll include unreachable spots
            // and spots that are occupied by a unit/wall. No out of bounds tiles, thanks to the
            // checks during arena parsing.
            let mut target_tiles = vec![];
            for enemy in &enemies {
                target_tiles.append(&mut enemy.position.neighbours());
            }
            let mut unit_position = self.units[i].position;
            let (distances, origins) = self.distances_and_origins(unit_position);

            // Pick the target: closest, and first in "reading order".
            // By now unreachable targets will have distance == usize::MAX.
            target_tiles.sort_by_key(|p| (distances[p.y][p.x], p.y, p.x));
            let target_tile = target_tiles[0];

            // Move if feasible.
            let target_distance = distances[target_tile.y][target_tile.x];
            if target_distance > 0 && target_distance < std::usize::MAX {
                // eprintln!(
                //     "Moving {} from {:?} to {:?}",
                //     unit_faction, unit_position, target_tile
                // );
                let (x, y) = match origins[target_tile.y][target_tile.x] {
                    Some(0) => (unit_position.x, unit_position.y - 1), // above
                    Some(1) => (unit_position.x - 1, unit_position.y), // left
                    Some(2) => (unit_position.x + 1, unit_position.y), // right
                    Some(3) => (unit_position.x, unit_position.y + 1), // below
                    Some(c) => panic!("Unexpected origin to move towards: {}", c),
                    None => panic!("No origin to move towards."),
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
                eprintln!(
                    "{} damages enemy at {:?}",
                    unit_faction, targets[0].position
                );
                let target = &mut targets[0];
                target.hp -= ap;
                if target.hp <= 0 {
                    self.grid[target.position.y][target.position.x] = '.';
                }
            }
        }
        // Tick the clock!
        self.clock += 1;
        eprintln!("{:?}", self);
        true
    }

    fn outcome(&self) -> usize {
        let hp_sum: usize = self
            .units
            .iter()
            .filter_map(|u| if u.hp > 0 { Some(u.hp as usize) } else { None })
            .sum();
        hp_sum * self.clock
    }
}

// Pretty printer for the arena.
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

// This is a replica of Point::neighbours, with the ordering of the points reflecting this
// puzzle's desired ordering ("reading order"). Didn't want to make a cross-file assumption
// without a way to ensure it.
fn neighbours_in_reading_order(p: &Point) -> Vec<Point> {
    vec![
        Point::new(p.x, p.y - 1),
        Point::new(p.x - 1, p.y),
        Point::new(p.x + 1, p.y),
        Point::new(p.x, p.y + 1),
    ]
}

fn part1(input: &str) -> usize {
    let mut arena = Arena::from_str(input, 3);
    while arena.tick() {}
    arena.outcome()
}

fn part2(input: &str) -> usize {
    let mut elf_ap = 4;
    let mut arena = Arena::from_str(&input, elf_ap);
    let elven_army_size = arena
        .units
        .iter()
        .filter(|u| u.faction == 'E')
        .fold(0, |acc, _| acc + 1);
    eprintln!("Starting elvish army size: {}", elven_army_size);
    loop {
        eprintln!("Trying out elf_ap={}", elf_ap);
        while arena.tick() {}

        let surviving_elves: Vec<_> = arena
            .units
            .iter()
            .filter(|u| u.hp > 0 && u.faction == 'E')
            .collect();
        eprintln!(
            "{} surviving elves: {:?}",
            surviving_elves.len(),
            surviving_elves
        );
        if surviving_elves.len() == elven_army_size {
            break;
        } else {
            elf_ap += 1;
            arena = Arena::from_str(&input, elf_ap);
        }
    }
    arena.outcome()
}

fn main() {
    let input = read_file("inputs/15");
    let answer1 = part1(&input);
    assert_eq!(answer1, 195774);
    println!("Outcome of battle for part 1: {}", answer1);
    let answer2 = part2(&input);
    assert_eq!(answer2, 37272);
    println!("Outcome of battle for part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // let input = r#"#######
        //                #.G...#
        //                #...EG#
        //                #.#.#G#
        //                #..G#E#
        //                #.....#
        //                #######"#;
        // assert_eq!(part1(&input), 27730);
        // assert_eq!(part2(&input), 4988);
        // let input = r#"#######
        //                #G..#E#
        //                #E#E.E#
        //                #G.##.#
        //                #...#E#
        //                #...E.#
        //                #######"#;
        // assert_eq!(part1(&input), 36334);
        let input = r#"#######
                       #E..EG#
                       #.#G.E#
                       #E.##E#
                       #G..#.#
                       #..E#.#
                       #######"#;
        //assert_eq!(part1(&input), 39514);
        assert_eq!(part2(&input), 31284);
        // let input = r#"#######
        //                #E.G#.#
        //                #.#G..#
        //                #G.#.G#
        //                #G..#.#
        //                #...E.#
        //                #######"#;
        // assert_eq!(part1(&input), 27755);
        // assert_eq!(part2(&input), 3478);

        // let input = r#"#######
        //                #.E...#
        //                #.#..G#
        //                #.###.#
        //                #E#G#G#
        //                #...#G#
        //                #######"#;
        // assert_eq!(part1(&input), 28944);
        // assert_eq!(part2(&input), 6474);

        // let input = r#"#########
        //                #G......#
        //                #.E.#...#
        //                #..##..G#
        //                #...##..#
        //                #...#...#
        //                #.G...G.#
        //                #.....G.#
        //                #########"#;
        // assert_eq!(part1(&input), 18740);
        // assert_eq!(part2(&input), 1140);
    }
}
