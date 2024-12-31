use std::fmt::Display;

use hashbrown::HashSet;

use crate::{grid::Grid, vec2::Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn next(&self, pos: Vec2<usize>, dimensions: (usize, usize)) -> Option<Vec2<usize>> {
        let (rows, cols) = dimensions;
        let Vec2 { x, y } = pos;

        match self {
            Move::Up => {
                if x == 0 {
                    None
                } else {
                    Some(Vec2::new(x - 1, y))
                }
            }
            Move::Down => {
                if x == rows - 1 {
                    None
                } else {
                    Some(Vec2::new(x + 1, y))
                }
            }

            Move::Left => {
                if y == 0 {
                    None
                } else {
                    Some(Vec2::new(x, y - 1))
                }
            }

            Move::Right => {
                if y == cols - 1 {
                    None
                } else {
                    Some(Vec2::new(x, y + 1))
                }
            }
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("Invalid character {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Obstacle,
    Empty,
    Box,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Obstacle => '#',
            Tile::Empty => '.',
            Tile::Box => 'O',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WideTile {
    LeftBox,
    RightBox,
    Obstacle,
    Empty,
}

impl Display for WideTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            WideTile::LeftBox => '[',
            WideTile::RightBox => ']',
            WideTile::Obstacle => '#',
            WideTile::Empty => '.',
        };

        write!(f, "{}", c)
    }
}

impl From<Grid<Tile>> for Grid<WideTile> {
    fn from(grid: Grid<Tile>) -> Self {
        let mut new_grid = Grid::new();

        for row in 0..grid.rows {
            let mut new_row = Vec::with_capacity(grid.cols * 2);
            for col in 0..grid.cols {
                match grid[(row, col)] {
                    Tile::Obstacle => {
                        new_row.push(WideTile::Obstacle);
                        new_row.push(WideTile::Obstacle);
                    }
                    Tile::Empty => {
                        new_row.push(WideTile::Empty);
                        new_row.push(WideTile::Empty);
                    }
                    Tile::Box => {
                        new_row.push(WideTile::LeftBox);
                        new_row.push(WideTile::RightBox);
                    }
                }
            }

            new_grid.push(&new_row);
        }

        new_grid
    }
}

fn parse_input(input: &str) -> (Grid<Tile>, Vec<Move>, Vec2<usize>) {
    let mut grid = Grid::new();
    let mut moves = Vec::new();
    let mut robot = (0, 0);

    let mut grid_section = true;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            grid_section = grid.is_empty();
            continue;
        }

        if grid_section {
            let mut row = Vec::new();

            for c in line.chars() {
                let tile = match c {
                    '#' => Tile::Obstacle,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => {
                        robot = (grid.rows, row.len());
                        Tile::Empty
                    }
                    _ => panic!("Invalid character {}", c),
                };

                row.push(tile);
            }

            grid.push(&row);
        } else {
            moves.extend(line.chars().map(Move::from));
        }
    }

    (grid, moves, robot.into())
}

pub fn calculate_final_gps_sum(input: &str) -> usize {
    let (grid, moves, start_pos) = parse_input(input);
    let mut grid = grid;

    simulate(&mut grid, &moves, start_pos);

    calculate_gps(&grid)
}

pub fn calculate_final_wide_gps_sum(input: &str) -> usize {
    let (grid, moves, start_pos) = parse_input(input);
    let mut grid = Grid::from(grid);

    let start_pos = (start_pos.x, start_pos.y * 2).into();

    simulate_wide_grid(&mut grid, &moves, start_pos);

    calculate_wide_gps(&grid)
}

fn calculate_gps(grid: &Grid<Tile>) -> usize {
    let mut gps_sum = 0;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if matches!(grid[(row, col)], Tile::Box) {
                gps_sum += row * 100 + col;
            }
        }
    }

    gps_sum
}

fn calculate_wide_gps(grid: &Grid<WideTile>) -> usize {
    let mut gps_sum = 0;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if matches!(grid[(row, col)], WideTile::LeftBox) {
                gps_sum += row * 100 + col;
            }
        }
    }

    gps_sum
}

fn simulate(grid: &mut Grid<Tile>, moves: &[Move], start_pos: Vec2<usize>) {
    let mut pos = start_pos;
    for m in moves {
        let next_pos = m.next(pos, (grid.rows, grid.cols));

        if next_pos.is_none() {
            continue;
        }

        let next_pos = next_pos.unwrap();

        if matches!(grid[next_pos], Tile::Obstacle) {
            continue;
        }

        if matches!(grid[next_pos], Tile::Empty) {
            pos = next_pos;
            continue;
        }

        // Project a ray from the position in the direction of the move until we hit an obstacle

        let mut ray_pos = next_pos;
        let mut ray = Vec::with_capacity(grid.rows.max(grid.cols)); // worst case size for grid

        while !matches!(grid[ray_pos], Tile::Obstacle) && matches!(grid[ray_pos], Tile::Box) {
            ray.push(ray_pos);
            ray_pos = m.next(ray_pos, (grid.rows, grid.cols)).unwrap();
        }

        // We check if the position after the final box is empty

        let next_box_pos = m.next(ray[ray.len() - 1], (grid.rows, grid.cols));

        if next_box_pos.is_none() || !matches!(grid[next_box_pos.unwrap()], Tile::Empty) {
            continue;
        }

        // Then we shift all the boxes in the ray by one position in the direction of the move

        let mut initial_swap = next_box_pos.unwrap();

        for pos in ray.iter().rev() {
            let pos = *pos;
            grid.swap(pos.into(), initial_swap.into());

            initial_swap = pos;
        }

        // Then finally we move the robot to the next position

        pos = next_pos;
    }
}

#[allow(dead_code)]
fn display_grid_with_robot<K>(grid: &Grid<K>, robot: Vec2<usize>, r#move: Move)
where
    K: Display + Clone,
{
    println!(
        "{}",
        std::iter::repeat_n('=', grid.cols).collect::<String>()
    );
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if (row, col) == (robot.x, robot.y) {
                print!("@");
            } else {
                print!("{}", grid[(row, col)]);
            }
        }
        println!();
    }
    println!("Move: {:?}", r#move);
    println!(
        "{}",
        std::iter::repeat_n('=', grid.cols).collect::<String>()
    );
}

fn simulate_wide_grid(grid: &mut Grid<WideTile>, moves: &[Move], start_pos: Vec2<usize>) {
    use WideTile::*;
    let mut pos = start_pos;

    let cols = grid.cols;
    let rows = grid.rows;
    let max_v = rows.max(cols);

    for m in moves {
        let next_pos = m.next(pos, (grid.rows, grid.cols));

        if next_pos.is_none() {
            continue;
        }

        let next_pos = next_pos.unwrap();

        if grid[next_pos] == Obstacle {
            continue;
        }

        if grid[next_pos] == Empty {
            pos = next_pos;
            continue;
        }

        // Our ray propagation behaves differently if we are moving up/down or left/right
        // In the case of left/right, we check for contiguous boxes to the left/right of the box ie. [LeftBox, RightBox]
        // And depending on if we are moving left or right
        // For up/down, we flood fill all connected boxes in the direction of the move
        // And then check if the position after the connected boxes in the direction of the move is empty (noting that the boxes can be bizarrely shaped)

        if matches!(m, Move::Left | Move::Right) {
            let mut ray_pos = next_pos;
            let mut ray = Vec::with_capacity(max_v); // worst case size for grid

            while !matches!(grid[ray_pos], Obstacle | Empty) {
                let next_ray_pos = m.next(ray_pos, (rows, cols)).unwrap();
                match (grid[ray_pos], grid[next_ray_pos]) {
                    (LeftBox, RightBox) | (RightBox, LeftBox) => {
                        ray.push(ray_pos);
                        ray.push(next_ray_pos);

                        ray_pos = m.next(next_ray_pos, (rows, cols)).unwrap();
                    }
                    _ => break,
                }
            }

            let next_box_pos = m.next(ray[ray.len() - 1], (rows, cols));

            if next_box_pos.is_none() || !matches!(grid[next_box_pos.unwrap()], Empty) {
                continue;
            }

            let mut initial_swap = next_box_pos.unwrap();

            for pos in ray.iter().rev() {
                let pos = *pos;
                grid.swap(pos.into(), initial_swap.into());

                initial_swap = pos;
            }
        } else {
            // In this case, we flood fill all connected boxes in the direction of the move, making sure to connected left/right boxes

            let mut connected_boxes = HashSet::new();

            let get_connected_box = |pos| {
                if grid[pos] == LeftBox {
                    let right_pos = Move::Right.next(pos, (rows, cols)).unwrap();
                    return (pos, right_pos);
                } else if grid[pos] == RightBox {
                    let left_pos = Move::Left.next(pos, (rows, cols)).unwrap();
                    return (left_pos, pos);
                };
                unreachable!()
            };

            let r#box = get_connected_box(next_pos);
            connected_boxes.insert(r#box);
            let mut queue = Vec::from([r#box.0, r#box.1]);

            while let Some(pos) = queue.pop() {
                let next_pos = m.next(pos, (rows, cols));
                if let Some(next_pos) = next_pos {
                    if matches!(grid[next_pos], LeftBox | RightBox) {
                        let r#box = get_connected_box(next_pos);
                        if connected_boxes.insert(r#box) {
                            queue.push(r#box.0);
                            queue.push(r#box.1);
                        }
                    }
                }
            }

            // We get the edge boxes of the connected boxes in the direction of the move, this is all boxes that are not connected to another box in the direction of the move in either/both the left and right directions

            let mut edge_boxes = HashSet::new();

            for (l, r) in connected_boxes.iter() {
                let left_pos = m.next(*l, (rows, cols));
                let right_pos = m.next(*r, (rows, cols));

                if left_pos.is_none() || right_pos.is_none() {
                    continue;
                }

                let left_pos = left_pos.unwrap();
                let right_pos = right_pos.unwrap();

                if !matches!(grid[left_pos], LeftBox | RightBox) {
                    edge_boxes.insert(*l);
                }

                if !matches!(grid[right_pos], LeftBox | RightBox) {
                    edge_boxes.insert(*r);
                }
            }

            // We then check if the position after the edge boxes in the direction of the move is empty

            let mut invalid = false;

            for pos in edge_boxes.iter() {
                let next_pos = m.next(*pos, (cols, rows)).unwrap();
                if !matches!(grid[next_pos], Empty) {
                    invalid = true;
                    break;
                }
            }

            if invalid || edge_boxes.is_empty() {
                continue;
            }

            for (l, r) in connected_boxes.iter() {
                grid[*l] = Empty;
                grid[*r] = Empty;
            }

            for (l, r) in connected_boxes.iter() {
                grid[m.next(*l, (cols, rows)).unwrap()] = LeftBox;
                grid[m.next(*r, (cols, rows)).unwrap()] = RightBox;
            }
        }

        pos = next_pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    const SMALL_TEST_INPUT: &str = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_LARGE_INPUT: &str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SMALL_WIDE_BOX_TEST_INPUT: &str = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[rstest]
    #[case(SMALL_TEST_INPUT, 2028)]
    #[case(TEST_LARGE_INPUT, 10092)]
    fn test_calculate_final_gps_sum(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(calculate_final_gps_sum(input), expected);
    }

    #[rstest]
    #[case(SMALL_WIDE_BOX_TEST_INPUT, 618)]
    #[case(TEST_LARGE_INPUT, 9021)]
    fn test_calculate_final_wide_gps_sum(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(calculate_final_wide_gps_sum(input), expected);
    }
}
