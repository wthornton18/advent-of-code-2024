use hashbrown::HashMap;

use crate::a_star_search::AStarSearch;
use crate::grid::{Grid, Tile};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaceTrack {
    Empty,
    Wall,
}

impl Tile for RaceTrack {
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

fn parse_input(input: &str) -> (Grid<RaceTrack>, (usize, usize), (usize, usize)) {
    let mut grid = Grid::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut row = Vec::with_capacity(line.len());
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(RaceTrack::Empty),
                '#' => row.push(RaceTrack::Wall),
                'S' => {
                    row.push(RaceTrack::Empty);
                    start = (grid.rows, j);
                }
                'E' => {
                    row.push(RaceTrack::Empty);
                    end = (grid.rows, j);
                }
                _ => panic!("Invalid character in input: {}", c),
            }
        }
        grid.push(row.as_slice());
    }

    (grid, start, end)
}

#[inline(always)]
fn manhattan_distance<K>(a: (usize, usize), b: (usize, usize)) -> K
where
    K: From<u32>,
{
    let (x1, y1) = a;
    let (x2, y2) = b;
    let d = ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as u32;
    K::from(d)
}

fn get_optimum_path(
    grid: &Grid<RaceTrack>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    grid.a_star_search(start, end, manhattan_distance::<f64>)
        .expect("No path found")
}

#[inline(always)]
fn manhattan_deltas(r: usize) -> Vec<((i32, i32), usize)> {
    let mut deltas = Vec::with_capacity(r * r * 4);

    for i in 0..=r {
        for j in 0..=r {
            if i == 0 && j == 0 {
                continue;
            }

            let d = manhattan_distance::<u32>((i, j), (0, 0)) as usize;
            let i = i as i32;
            let j = j as i32;
            if d <= r {
                if !deltas.contains(&((i, j), d)) {
                    deltas.push(((i, j), d));
                }
                if !deltas.contains(&((-i, j), d)) {
                    deltas.push(((-i, j), d));
                }

                if !deltas.contains(&((i, -j), d)) {
                    deltas.push(((i, -j), d));
                }

                if !deltas.contains(&((-i, -j), d)) {
                    deltas.push(((-i, -j), d));
                }
            }
        }
    }
    deltas
}

pub fn get_total_number_of_cheats(
    input: &str,
    saves_at_least: usize,
    cheat_distance: usize,
) -> usize {
    let (grid, start, end) = parse_input(input);
    let path = get_optimum_path(&grid, start, end);

    let mut path_to_distances = Grid::with_capacity_and_default(grid.rows, grid.cols, None);
    for (i, p_a) in path.iter().enumerate() {
        path_to_distances[*p_a] = Some(i);
    }

    let deltas = manhattan_deltas(cheat_distance);
    path.par_iter()
        .enumerate()
        .map(|(i, a)| {
            let mut total_cheats = 0;
            for (delta, distance) in &deltas {
                let (dy, dx) = delta;
                let b = (a.0 as i32 + dx, a.1 as i32 + dy);
                let b = (b.0 as usize, b.1 as usize);
                if let Some(j) = path_to_distances.get(b).flatten() {
                    if let Some(distance) = j.checked_sub(i + distance) {
                        if distance >= saves_at_least {
                            total_cheats += 1;
                        }
                    }
                }
            }
            total_cheats
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[rstest]
    #[case(65, 0, 2)]
    #[case(64, 1, 2)]
    #[case(40, 2, 2)]
    #[case(38, 3, 2)]
    #[case(36, 4, 2)]
    #[case(20, 5, 2)]
    #[case(12, 8, 2)]
    #[case(10, 10, 2)]
    #[case(8, 14, 2)]
    #[case(6, 16, 2)]
    #[case(4, 30, 2)]
    #[case(2, 44, 2)]
    #[case(76, 3, 20)]
    #[case(74, 7, 20)]
    fn test_get_total_number_of_cheats(
        #[case] saves_at_least: usize,
        #[case] expected: usize,
        #[case] cheat_distance: usize,
    ) {
        let cheats = get_total_number_of_cheats(TEST_INPUT, saves_at_least, cheat_distance);
        assert_eq!(cheats, expected);
    }
}
