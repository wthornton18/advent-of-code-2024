use std::u8;


use crate::grid::Grid;

fn parse_input(input: &str) -> Grid<u8> {
    let mut grid = Grid::new();

    for line in input.lines() {
        let line = line.trim();
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            match c {
                '.' => row.push(u8::MAX),
                '0'..='9' => row.push(c.to_digit(10).unwrap() as u8),
                'a'..='z' => row.push(c as u8 - b'a' + 10),
                'A'..='Z' => row.push(c as u8 - b'A' + 36),
                _ => panic!("Invalid character in input"),
            };
        }

        grid.push(&row);
    }

    grid
}
fn display_signal_grid(signal_grid: &Grid<u8>) {
    for i in 0..signal_grid.rows {
        for j in 0..signal_grid.cols {
            let value = signal_grid[(i, j)];
            match value {
                u8::MAX => print!("."),
                v if v == u8::MAX - 1 => print!("#"), // Antinode
                0..=9 => print!("{}", value),
                10..=35 => print!("{}", (value - 10 + b'a') as char),
                36..=61 => print!("{}", (value - 36 + b'A') as char),
                _ => panic!("Invalid value in signal grid"),
            }
        }
        println!();
    }
}

pub fn count_unique_antinodes(input: &str, keep_iterating: bool) -> usize {
    let signal_grid = parse_input(input);

    let anti_node_grids = generate_antinode_grids(&signal_grid, keep_iterating);

    let overall_antinode_grid = anti_node_grids.iter().fold(
        Grid::with_capacity_and_default(signal_grid.rows, signal_grid.cols, false),
        |acc, grid| acc | grid.clone(),
    );

    overall_antinode_grid
        .into_iter()
        .filter(|(_, v)| *v)
        .count()
}

fn generate_antinode_grids(signal_grid: &Grid<u8>, keep_iterating: bool) -> Vec<Grid<bool>> {
    let unique_values = signal_grid
        .data
        .iter()
        .filter(|&&v| v != u8::MAX)
        .collect::<Vec<_>>();

    let mut antinode_grids = Vec::with_capacity(unique_values.len());

    for unique_value in unique_values {
        let mut valid_positions = Vec::with_capacity(signal_grid.rows * signal_grid.cols);

        for i in 0..signal_grid.rows {
            for j in 0..signal_grid.cols {
                if signal_grid[(i, j)] == *unique_value {
                    valid_positions.push((i as i64, j as i64));
                }
            }
        }

        let mut antinode_grid =
            Grid::with_capacity_and_default(signal_grid.rows, signal_grid.cols, false);

        for (i, (xi, yi)) in valid_positions.iter().enumerate() {
            for (j, (xj, yj)) in valid_positions.iter().enumerate() {
                if i == j {
                    continue;
                }

                if keep_iterating {
                    let mut i = 0;

                    let x_d = xj - xi;
                    let y_d = yj - yi;

                    loop {
                        let a_x = xi - x_d * i;
                        let a_y = yi - y_d * i;

                        if a_x < 0
                            || a_x >= signal_grid.rows as i64
                            || a_y < 0
                            || a_y >= signal_grid.cols as i64
                        {
                            break;
                        }

                        antinode_grid[(a_x as usize, a_y as usize)] = true;

                        i += 1;
                    }
                } else {
                    let a_x = 2 * xi - xj;
                    let a_y = 2 * yi - yj;

                    if a_x < 0
                        || a_x >= signal_grid.rows as i64
                        || a_y < 0
                        || a_y >= signal_grid.cols as i64
                    {
                        continue;
                    }

                    antinode_grid[(a_x as usize, a_y as usize)] = true;
                }
            }
        }

        antinode_grids.push(antinode_grid);
    }

    antinode_grids
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
                                      ........0...
                                      .....0......
                                      .......0....
                                      ....0.......
                                      ......A.....
                                      ............
                                      ............
                                      ........A...
                                      .........A..
                                      ............
                                      ............";

    #[test]
    fn test_count_unique_antinodes() {
        assert_eq!(count_unique_antinodes(TEST_INPUT, false), 14);
    }

    #[test]
    fn test_count_unique_antinodes_iterating() {
        assert_eq!(count_unique_antinodes(TEST_INPUT, true), 34);
    }
}
