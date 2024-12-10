use std::fmt::Display;

use crate::grid::Grid;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
    Traversed,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Obstacle => '#',
            Self::Traversed => 'X',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn next_position(&self, position: (usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = position;
        if row == 0 && matches!(self, Self::Up) {
            return None;
        }

        if col == 0 && matches!(self, Self::Left) {
            return None;
        }

        Some(match self {
            Self::Up => (row - 1, col),
            Self::Down => (row + 1, col),
            Self::Left => (row, col - 1),
            Self::Right => (row, col + 1),
        })
    }

    fn rotate_90(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl From<Orientation> for u8 {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Up => 1,
            Orientation::Down => 2,
            Orientation::Left => 4,
            Orientation::Right => 8,
        }
    }
}

pub fn get_guard_path_length(input: &str) -> usize {
    let (mut grid, initial_guard_position) = parse_input(input);
    compute_guard_path(&mut grid, initial_guard_position);

    grid.into_iter()
        .filter(|(_, t)| *t == Tile::Traversed)
        .count()
}

pub fn get_total_number_of_cycles(input: &str) -> usize {
    let (grid, initial_guard_position) = parse_input(input);

    let mut total_number_of_cycles = 0;

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == Tile::Empty {
                let mut new_grid = grid.clone();
                new_grid[(i, j)] = Tile::Obstacle;
                if contains_cycle(&new_grid, initial_guard_position) {
                    total_number_of_cycles += 1;
                }
            }
        }
    }
    total_number_of_cycles
}

pub fn get_total_number_of_cycles_parallel(input: &str) -> usize {
    let (grid, initial_guard_position) = parse_input(input);

    let mut total_number_of_cycles = 0;

    let mut new_grids: Vec<Grid<Tile>> = Vec::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == Tile::Empty {
                let mut new_grid = grid.clone();
                new_grid[(i, j)] = Tile::Obstacle;
                new_grids.push(new_grid);
            }
        }
    }

    total_number_of_cycles = new_grids
        .par_iter()
        .map(|new_grid| contains_cycle(new_grid, initial_guard_position) as usize)
        .sum();

    total_number_of_cycles
}

pub fn get_total_number_of_cycles_parallel_chunked(input: &str) -> usize {
    let (grid, initial_guard_position) = parse_input(input);

    let mut total_number_of_cycles = 0;

    let mut new_grids: Vec<Grid<Tile>> = Vec::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == Tile::Empty {
                let mut new_grid = grid.clone();
                new_grid[(i, j)] = Tile::Obstacle;
                new_grids.push(new_grid);
            }
        }
    }

    let num_cpus = num_cpus::get();

    total_number_of_cycles = new_grids
        .par_chunks(num_cpus)
        .map(|chunk| {
            chunk
                .iter()
                .map(|new_grid| contains_cycle(new_grid, initial_guard_position) as usize)
                .sum::<usize>()
        })
        .sum();

    total_number_of_cycles
}

fn compute_guard_path(grid: &mut Grid<Tile>, start_position: (usize, usize)) {
    let mut position = start_position;
    let mut direction = Orientation::Up;

    loop {
        let (row, col) = position;
        let tile = grid[(row, col)];

        if tile == Tile::Empty {
            grid[(row, col)] = Tile::Traversed;
        }

        let next_position = direction.next_position(position);

        if next_position.is_none() {
            break;
        }

        let next_position = next_position.unwrap();

        match grid.get(next_position) {
            Some(Tile::Empty) | Some(Tile::Traversed) => {
                position = next_position;
            }
            Some(Tile::Obstacle) => {
                direction = direction.rotate_90();
            }
            None => {
                break;
            }
        }
    }
}

fn contains_cycle(grid: &Grid<Tile>, start_position: (usize, usize)) -> bool {
    let mut position = start_position;
    let mut direction = Orientation::Up;

    let mut direction_grid: Grid<u8> = Grid::with_capacity_and_default(grid.rows, grid.cols, 0);
    direction_grid[position] = Orientation::Up.into();

    let mut working_grid = grid.clone();

    loop {
        let (row, col) = position;
        let tile = working_grid[(row, col)];

        if tile == Tile::Empty {
            working_grid[(row, col)] = Tile::Traversed;
        }

        let next_position = direction.next_position(position);

        if next_position.is_none() {
            break;
        }

        let next_position = next_position.unwrap();

        match working_grid.get(next_position) {
            Some(Tile::Empty) | Some(Tile::Traversed) => {
                position = next_position;
            }
            Some(Tile::Obstacle) => {
                direction = direction.rotate_90();
            }
            None => {
                break;
            }
        }

        let d: u8 = direction.into();
        let previous_d = direction_grid[next_position];

        if d & previous_d != 0 {
            return true;
        }

        direction_grid[next_position] |= d;
    }

    false
}

fn parse_input(input: &str) -> (Grid<Tile>, (usize, usize)) {
    let mut grid = Grid::new();

    for line in input.lines() {
        let line = line.trim();
        let mut row = Vec::new();

        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Obstacle,
                '^' => Tile::Traversed,
                _ => panic!("Invalid character in input"),
            };
            row.push(tile);
        }

        grid.push(&row);
    }

    let initial_guard_position = grid
        .clone()
        .into_iter()
        .find(|(_, t)| *t == Tile::Traversed)
        .unwrap()
        .0;

    (grid, initial_guard_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::redundant_static_lifetimes)]
    const TEST_INPUT: &'static str = "....#.....
                                      .........#
                                      ..........
                                      ..#.......
                                      .......#..
                                      ..........
                                      .#..^.....
                                      ........#.
                                      #.........
                                      ......#...";

    #[test]
    fn test_get_guard_path_length() {
        let result = get_guard_path_length(TEST_INPUT);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_guard_path_has_cycle() {
        let test_has_cycle = "....#.....
                                    .........#
                                    ..........
                                    ..#.......
                                    .......#..
                                    ..........
                                    .#.#^.....
                                    ........#.
                                    #.........
                                    ......#...";
        let (grid, start_position) = parse_input(test_has_cycle);
        assert!(contains_cycle(&grid, start_position))
    }

    #[test]
    fn test_guard_path_has_no_cycle() {
        let (grid, start_position) = parse_input(TEST_INPUT);
        assert!(!contains_cycle(&grid, start_position))
    }

    #[test]
    fn test_get_total_number_of_cycles() {
        let result = get_total_number_of_cycles(TEST_INPUT);
        assert_eq!(result, 6);
    }
}
