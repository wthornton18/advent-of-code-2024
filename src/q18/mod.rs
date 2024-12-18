use crate::{
    a_star_search::AStarSearch,
    graph::Graph,
    grid::{Grid, Tile},
};
use hashbrown::HashSet;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Q18Tile {
    Empty,
    Obstacle,
}

impl Tile for Q18Tile {
    fn traversable(&self) -> bool {
        matches!(self, Self::Empty)
    }

    fn cost_from<V>(&self, _other: V) -> f64
    where
        V: Tile,
    {
        1.0
    }
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let mut parts = line.split(',');
            let y = parts.next().unwrap().parse().unwrap();
            let x = parts.next().unwrap().parse().unwrap();
            Some((x, y))
        })
        .collect()
}

fn construct_graph(
    falling_bytes: &[(usize, usize)],
    bytes_to_fall: usize,
    (rows, cols): (usize, usize),
) -> Graph<(usize, usize), usize> {
    let mut graph = Graph::new();
    let bytes_to_fall = falling_bytes.len().min(bytes_to_fall);
    let bytes = &falling_bytes[..bytes_to_fall];
    let mut byte_positions = HashSet::with_capacity(bytes.len());
    for (i, j) in bytes {
        byte_positions.insert((*i, *j));
    }
    println!("{:?}", byte_positions);

    for i in 0..rows {
        for j in 0..cols {
            if byte_positions.contains(&(i, j)) {
                continue;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_i = i as isize + dx;
                let new_j = j as isize + dy;

                if new_i < 0 || new_j < 0 {
                    continue;
                }

                let new_i = new_i as usize;
                let new_j = new_j as usize;

                if new_i >= rows || new_j >= cols {
                    continue;
                }

                if byte_positions.contains(&(new_i, new_j)) {
                    continue;
                }

                graph.add_edge((i, j), (new_i, new_j), 1);
            }
        }
    }

    graph
}

fn heuristic(source: (usize, usize), target: (usize, usize)) -> f64 {
    let dx = (source.0 as isize - target.0 as isize).abs() as f64;
    let dy = (source.1 as isize - target.1 as isize).abs() as f64;
    dy + dx
}

pub fn get_bytes_shortest_path_length(
    input: &str,
    bytes_to_fall: usize,
    dim: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let falling_bytes = parse_input(input);
    let graph = construct_graph(&falling_bytes, bytes_to_fall, dim);

    let path = graph.a_star_search(start, end, heuristic);
    path.unwrap().len() - 1
}

pub fn get_bytes_shortest_path_length_grid(
    input: &str,
    bytes_to_fall: usize,
    dim: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
) -> usize {
    let falling_bytes = parse_input(input);

    let mut grid = Grid::with_capacity_and_default(dim.0, dim.1, Q18Tile::Empty);
    for (i, j) in falling_bytes.iter().take(bytes_to_fall) {
        grid[(*i, *j)] = Q18Tile::Obstacle;
    }

    let path = grid.a_star_search(start, end, heuristic);
    path.unwrap().len() - 1
}

pub fn find_minimum_bytes_to_fall(
    input: &str,
    (rows, cols): (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, usize)> {
    let falling_bytes = parse_input(input);

    let mut graph = Graph::new();

    for i in 0..rows {
        for j in 0..cols {
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_i = i as isize + dx;
                let new_j = j as isize + dy;

                if new_i < 0 || new_j < 0 {
                    continue;
                }

                let new_i = new_i as usize;
                let new_j = new_j as usize;

                if new_i >= rows || new_j >= cols {
                    continue;
                }

                graph.add_edge((i, j), (new_i, new_j), 1);
            }
        }
    }

    for byte_position in falling_bytes {
        graph.remove_vertex(byte_position);

        let path = graph.a_star_search(start, end, heuristic);
        if path.is_none() {
            return Some((byte_position.1, byte_position.0)); // invert the coordinates
        }
    }

    None
}

pub fn find_minimum_bytes_to_fall_parallel(
    input: &str,
    (rows, cols): (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, usize)> {
    let falling_bytes = parse_input(input);

    (0..falling_bytes.len())
        .into_par_iter()
        .find_first(|idx| {
            let mut grid = Grid::with_capacity_and_default(rows, cols, Q18Tile::Empty);
            for (i, j) in falling_bytes.iter().take(*idx) {
                grid[(*i, *j)] = Q18Tile::Obstacle;
            }

            let path = grid.a_star_search(start, end, heuristic);
            path.is_none()
        })
        .map(|byte_position| {
            let byte_position = falling_bytes[byte_position - 1];
            (byte_position.1, byte_position.0)
        })
}

#[allow(dead_code)]
fn display_path_on_grid(
    graph: &Graph<(usize, usize), usize>,
    (rows, cols): (usize, usize),
    path: &[(usize, usize)],
) {
    // Grid will also mark unreachable nodes with '#' - this is not a bug
    let mut grid = Grid::with_capacity_and_default(rows, cols, '#');

    for i in 0..rows {
        for j in 0..cols {
            if graph.contains(&(i, j)) {
                grid[(i, j)] = '.';
            }
        }
    }

    println!("{}", grid);

    for (i, j) in path {
        grid[(*i, *j)] = 'O';
    }

    println!("{}", grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_get_bytes_shortest_path_length() {
        let result = get_bytes_shortest_path_length(TEST_INPUT, 12, (7, 7), (0, 0), (6, 6));
        assert_eq!(result, 22);
    }

    #[test]
    fn test_get_bytes_shortest_path_length_grid() {
        let result = get_bytes_shortest_path_length_grid(TEST_INPUT, 12, (7, 7), (0, 0), (6, 6));
        assert_eq!(result, 22);
    }

    #[test]
    fn test_find_minimum_bytes_to_fall() {
        let result = find_minimum_bytes_to_fall(TEST_INPUT, (7, 7), (0, 0), (6, 6));
        assert_eq!(result, Some((6, 1)));
    }

    #[test]
    fn test_find_minimum_bytes_to_fall_parallel() {
        let result = find_minimum_bytes_to_fall_parallel(TEST_INPUT, (7, 7), (0, 0), (6, 6));
        assert_eq!(result, Some((6, 1)));
    }
}
