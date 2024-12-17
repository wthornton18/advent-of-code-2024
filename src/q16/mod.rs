use core::f64;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fmt::Display,
};

use hashbrown::HashMap;

use crate::{
    graph::{self, Graph, Weight},
    grid::Grid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Obstacle => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    East,
    West,
    South,
    North,
}

impl From<(i32, i32)> for Dir {
    fn from(value: (i32, i32)) -> Self {
        match value {
            (0, 1) => Self::East,
            (0, -1) => Self::West,
            (1, 0) => Self::South,
            (-1, 0) => Self::North,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<Dir> for (i32, i32) {
    fn from(value: Dir) -> Self {
        match value {
            Dir::East => (0, 1),
            Dir::West => (0, -1),
            Dir::South => (1, 0),
            Dir::North => (-1, 0),
        }
    }
}

impl Dir {
    fn get_90_rotations(&self) -> [Self; 2] {
        match self {
            Self::East | Self::West => [Self::North, Self::South],
            Self::North | Self::South => [Self::East, Self::West],
        }
    }

    fn get_clockwise_rotation(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }

    fn get_anti_clockwise_rotation(&self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Forward,
    Clockwise,
    AntiClockwise,
}

impl Weight for Move {
    fn weight(&self) -> f64 {
        match self {
            Self::Forward => 1.0,
            Self::Clockwise | Self::AntiClockwise => 1000.0,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Forward => 'F',
            Self::Clockwise => 'C',
            Self::AntiClockwise => 'A',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub position: (usize, usize),
    pub direction: Dir,
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (row, col) = self.position;
        let c = match self.direction {
            Dir::East => 'E',
            Dir::West => 'W',
            Dir::South => 'S',
            Dir::North => 'N',
        };
        write!(f, "({}, {}) {}", row, col, c)
    }
}

pub fn get_shortest_path_cost(input: &str) -> f64 {
    let (grid, start, goal) = parse_input(input);
    let graph = build_graph(&grid, start);
    let (dist, _) = graph.dijkstra(Vertex {
        position: start,
        direction: Dir::East,
    });

    let mut min_cost = f64::INFINITY;
    for (vert, cost) in dist {
        if vert.position == goal {
            min_cost = min_cost.min(cost);
        }
    }
    min_cost
}

pub fn get_area_covered_by_shortest_paths(input: &str) -> usize {
    let (grid, start, goal) = parse_input(input);

    let graph = build_graph(&grid, start);

    let (dist, prev) = graph.dijkstra(Vertex {
        position: start,
        direction: Dir::East,
    });

    let mut min_cost = f64::INFINITY;
    let mut min_cost_vertex = Vertex {
        position: (0, 0),
        direction: Dir::East,
    };
    for (vert, cost) in dist {
        if vert.position == goal && cost < min_cost {
            min_cost = cost;
            min_cost_vertex = vert;
        }
    }
    let graph = prev.shortest_paths_subgraph(
        min_cost_vertex,
        Vertex {
            position: start,
            direction: Dir::East,
        },
    );
    let collapsed_graph = collapse_graph(&graph);

    collapsed_graph.len()
}

fn collapse_graph<W: Weight>(graph: &Graph<Vertex, W>) -> Graph<(usize, usize), f64> {
    let mut new_graph = Graph::new();

    for (from, edges) in &graph.edges {
        for (to, weight) in edges {
            let new_from = from.position;
            let new_to = to.position;
            new_graph.add_edge(new_from, new_to, weight.weight());
        }
    }

    new_graph
}

fn build_graph(grid: &Grid<Tile>, start: (usize, usize)) -> Graph<Vertex, Move> {
    // Worst case capacity is 3 moves per tile, the two rotations and the forward moves
    let mut graph = Graph::with_capacity(grid.rows * grid.cols * 3);

    let mut queue = Vec::from([Vertex {
        position: start,
        direction: Dir::East,
    }]);

    let mut seen: HashMap<Vertex, bool> = HashMap::new();

    while let Some(curr) = queue.pop() {
        if seen.contains_key(&curr) {
            continue;
        }

        seen.insert(curr, true);

        for next_dir in curr.direction.get_90_rotations() {
            let r#move = if next_dir == curr.direction.get_clockwise_rotation() {
                Move::Clockwise
            } else if next_dir == curr.direction.get_anti_clockwise_rotation() {
                Move::AntiClockwise
            } else {
                panic!("Invalid rotation");
            };
            let new_vertex = Vertex {
                position: curr.position,
                direction: next_dir,
            };

            if !seen.contains_key(&new_vertex) {
                queue.push(new_vertex);
            }
            graph.add_edge(curr, new_vertex, r#move);
        }

        let (dx, dy) = curr.direction.into();

        let next_coord = (curr.position.0 as i32 + dx, curr.position.1 as i32 + dy);

        if next_coord.0 < 0 || next_coord.1 < 0 {
            continue;
        }

        if next_coord.0 >= grid.rows as i32 || next_coord.1 >= grid.cols as i32 {
            continue;
        }

        let next_coord = (next_coord.0 as usize, next_coord.1 as usize);

        if grid[next_coord] == Tile::Obstacle {
            continue;
        }

        let r#move = Move::Forward;
        let next_vertex = Vertex {
            position: next_coord,
            direction: curr.direction,
        };

        if !seen.contains_key(&next_vertex) {
            queue.push(next_vertex);
        }
        graph.add_edge(curr, next_vertex, r#move);
    }

    graph
}

fn parse_input(input: &str) -> (Grid<Tile>, (usize, usize), (usize, usize)) {
    let mut grid = Grid::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);

    let mut offset = 0;
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            offset += 1;
            continue;
        }

        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Obstacle,
                'S' => {
                    start = (i - offset, j);
                    Tile::Empty
                }
                'E' => {
                    goal = (i - offset, j);
                    Tile::Empty
                }
                _ => panic!("Invalid character"),
            };
            row.push(tile);
        }
        grid.push(&row);
    }

    (grid, start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const OTHER_TEST_INPUT: &str = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################  
";

    #[rstest]
    #[case(TEST_INPUT, 7036.0)]
    #[case(OTHER_TEST_INPUT, 11048.0)]
    fn test_get_shortest_path_cost(#[case] input: &str, #[case] expected: f64) {
        assert_eq!(get_shortest_path_cost(input), expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 45)]
    #[case(OTHER_TEST_INPUT, 64)]
    fn test_get_area_covered_by_shortest_paths(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(get_area_covered_by_shortest_paths(input), expected);
    }
}
