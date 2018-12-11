const GRID_SIZE: usize = 300;

struct PowerGrid {
    serial_number: i32,
    cells: Vec<Vec<i32>>,
}

// power level calculation for a single cell.
fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    ((((x + 10) * y + serial) * (x + 10)) % 1000) / 100 - 5
}

fn parse_input(input: i32) -> PowerGrid {
    let mut power_grid = PowerGrid {
        serial_number: input,
        cells: vec![vec![0; GRID_SIZE]; GRID_SIZE],
    };
    // Establish power level for each cell.
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            power_grid.cells[i][j] = power_level(i as i32, j as i32, power_grid.serial_number);
        }
    }
    power_grid
}

// Where's the most powerful 3x3 square of cells?
fn part1(power_grid: &PowerGrid) -> (usize, usize) {
    // Brute force: sweep all locations that are suitable for top left corner of a 3x3 square on
    // the grid and calculate square's total power.
    let mut power = vec![vec![0; GRID_SIZE - 2]; GRID_SIZE - 2];
    for i in 0..GRID_SIZE - 2 {
        for j in 0..GRID_SIZE - 2 {
            for m in 0..=2 {
                for n in 0..=2 {
                    power[i][j] += power_grid.cells[i + m][j + n];
                }
            }
        }
    }
    // Sweep all squares, find the most powerful one.
    let mut max = i32::min_value();
    let mut coords = (0, 0);
    for i in 0..GRID_SIZE - 2 {
        for j in 0..GRID_SIZE - 2 {
            if power[i][j] > max {
                max = power[i][j];
                coords = (i, j);
            }
        }
    }
    coords
}

// Where's the most powerful square of cells, sized [1..300]?
fn part2(power_grid: &PowerGrid) -> (usize, usize, usize) {
    // Calculate summed-area table.
    // https://en.wikipedia.org/wiki/Summed-area_table
    let mut summed_table = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let mut tmp = power_grid.cells[i][j];
            tmp += if i > 0 { summed_table[i - 1][j] } else { 0 };
            tmp += if j > 0 { summed_table[i][j - 1] } else { 0 };
            tmp -= if i > 0 && j > 0 {
                summed_table[i - 1][j - 1]
            } else {
                0
            };
            summed_table[i][j] = tmp;
        }
    }
    // Sweep all grid locations, for every location calculate power of all possible squares
    // anchored at this point.
    let mut power = vec![vec![vec![i32::min_value(); GRID_SIZE]; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            for n in 0..GRID_SIZE {
                // Does the square fit in the grid?
                if i + n >= GRID_SIZE || j + n >= GRID_SIZE {
                    continue;
                }
                let mut tmp = summed_table[i + n][j + n];
                tmp += if i > 0 && j > 0 {
                    summed_table[i - 1][j - 1]
                } else {
                    0
                };
                tmp -= if j > 0 { summed_table[i + n][j - 1] } else { 0 };
                tmp -= if i > 0 { summed_table[i - 1][j + n] } else { 0 };
                power[i][j][n] = tmp;
            }
        }
    }
    // Sweep all possible powers, find the largest one.
    let mut max = i32::min_value();
    let mut coords = (0, 0, 0);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            for n in 0..GRID_SIZE {
                if power[i][j][n] > max {
                    max = power[i][j][n];
                    coords = (i, j, n + 1);
                }
            }
        }
    }
    coords
}

fn main() {
    let power_grid = parse_input(5719);
    let best_3x3_square = part1(&power_grid);
    assert_eq!(best_3x3_square, (21, 34));
    println!(
        "Coordinates of the most powerful 3x3 square: {},{}",
        best_3x3_square.0, best_3x3_square.1
    );

    let best_cell = part2(&power_grid);
    assert_eq!(best_cell, (90, 244, 16));
    println!("The most powerful cell is {:?}.", best_cell);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
        assert_eq!(part1(&parse_input(18)), (33, 45));
        assert_eq!(part1(&parse_input(42)), (21, 61));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(18)), (90, 269, 16));
        assert_eq!(part2(&parse_input(42)), (232, 251, 12));
    }
}
