use itertools::repeat_n;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
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

    fn delta(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Obstacle,
    Robot,
    Box,
}

impl Grid<Option<Tile>> {
    fn gps(&self) -> usize {
        let mut gps = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if let Some(Tile::Box) = self[(i, j)] {
                    gps += j + i * 100
                }
            }
        }

        gps
    }

    fn move_robot(&mut self, dir: Move) {
        let robot_pos = self.get_robot_pos();
        let delta = dir.delta();
        let next_pos = dir.next_position(robot_pos);

        if next_pos.is_none() {
            panic!("Cannot make a valid move in this position");
        }

        let next_pos = next_pos.unwrap();

        match self[next_pos] {
            None => self.swap(next_pos, robot_pos),
            Some(Tile::Obstacle) => {}
            Some(Tile::Robot) => panic!("Cannot be multiple robots"),
            Some(Tile::Box) => {
                let mut can_shift_boxes = false;
                let mut n_pos = next_pos;
                let mut boxes = 0;

                while let Some(pos) = dir.next_position(n_pos) {
                    println!("{:?}", self[pos]);
                    match self[pos] {
                        None => {
                            can_shift_boxes = true;
                            break;
                        }
                        Some(Tile::Box) => {
                            boxes += 1;
                        }
                        Some(Tile::Obstacle) | Some(Tile::Robot) => break,
                    }
                    n_pos = pos;
                }

                if can_shift_boxes {
                    for i in (0..boxes).rev() {
                        let swap_pos = ((delta.0 * (i + 1)) as usize, (delta.1 * (i + 1)) as usize);
                        let pos = ((delta.0 * i) as usize, (delta.1 * i) as usize);
                        self.swap(pos, swap_pos);
                    }
                }
            }
        }
    }

    fn get_robot_pos(&self) -> (usize, usize) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                if let Some(Tile::Robot) = self[(i, j)] {
                    return (i, j);
                }
            }
        }
        panic!("Could not find a robot")
    }
}

pub fn get_final_gps(input: &str) -> usize {
    let (mut grid, moves) = parse_input(input);
    simulate(&mut grid, &moves);
    grid.gps()
}

fn parse_input(input: &str) -> (Grid<Option<Tile>>, Vec<Move>) {
    let mut grid = Grid::new();
    let mut robot_moves = Vec::new();
    let mut grid_section = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            grid_section = grid.is_empty();
            continue;
        }

        let mut row = Vec::with_capacity(line.len());

        for c in line.chars() {
            match (c, grid_section) {
                ('#', true) => row.push(Some(Tile::Obstacle)),
                ('O', true) => row.push(Some(Tile::Box)),
                ('@', true) => row.push(Some(Tile::Robot)),
                ('.', true) => row.push(None),
                ('^', false) => robot_moves.push(Move::Up),
                ('>', false) => robot_moves.push(Move::Right),
                ('<', false) => robot_moves.push(Move::Left),
                ('v', false) => robot_moves.push(Move::Down),
                (c, _) => panic!("Unknown character in input {c}"),
            }
        }

        if !row.is_empty() {
            grid.push(row.as_slice());
        }
    }

    (grid, robot_moves)
}

#[allow(dead_code)]
fn display_grid(grid: &Grid<Option<Tile>>) {
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            match grid[(i, j)] {
                Some(Tile::Box) => print!("O"),
                Some(Tile::Robot) => print!("@"),
                Some(Tile::Obstacle) => print!("#"),
                None => print!("."),
            }
        }
        println!();
    }
}

fn simulate(grid: &mut Grid<Option<Tile>>, moves: &[Move]) {
    for (i, r#move) in moves.iter().enumerate() {
        grid.move_robot(*r#move);
        // display_grid(grid);
        println!("Iteration {i}");
        println!("Move {:?}", r#move);
        println!("{}", repeat_n('=', grid.cols).collect::<String>())
    }
}
