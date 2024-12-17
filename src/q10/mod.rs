use crate::grid::Grid;
use rayon::prelude::*;

fn parse_input(input: &str) -> Grid<u8> {
    let mut grid = Grid::new();

    let input = input.trim();

    for line in input.lines() {
        let line = line.trim();
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            if c == '.' {
                row.push(u8::MAX);
            } else {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }
        grid.push(&row);
    }
    grid
}

pub fn get_total_trailhead_score(input: &str) -> usize {
    let grid = parse_input(input);
    get_all_trailhead_scores(&grid).iter().sum()
}

pub fn get_total_trailhead_score_parallel(input: &str) -> usize {
    let grid = parse_input(input);
    get_all_trailhead_scores_parallel(&grid).iter().sum()
}

pub fn get_total_trailhead_rating(input: &str) -> usize {
    let grid = parse_input(input);
    get_all_trailhead_ratings(&grid).iter().sum()
}

pub fn get_total_trailhead_rating_parallel(input: &str) -> usize {
    let grid = parse_input(input);
    get_all_trailhead_ratings_parallel(&grid).iter().sum()
}

fn get_all_trailhead_scores(grid: &Grid<u8>) -> Vec<usize> {
    let mut trailhead_scores = Vec::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == 0 {
                trailhead_scores.push(get_trailhead_score(grid, (i, j)));
            }
        }
    }
    trailhead_scores
}

fn get_all_trailhead_scores_parallel(grid: &Grid<u8>) -> Vec<usize> {
    let trailheads = (0..grid.rows)
        .flat_map(|i| (0..grid.cols).map(move |j| (i, j)))
        .filter(|&v| grid[v] == 0)
        .collect::<Vec<_>>();

    trailheads
        .par_iter()
        .map(|&trailhead| get_trailhead_score(grid, trailhead))
        .collect()
}

fn get_all_trailhead_ratings_parallel(grid: &Grid<u8>) -> Vec<usize> {
    let trailheads = (0..grid.rows)
        .flat_map(|i| (0..grid.cols).map(move |j| (i, j)))
        .filter(|&v| grid[v] == 0)
        .collect::<Vec<_>>();

    trailheads
        .par_iter()
        .map(|&trailhead| get_trailhead_rating(grid, trailhead))
        .collect()
}

fn get_all_trailhead_ratings(grid: &Grid<u8>) -> Vec<usize> {
    let mut trailhead_ratings = Vec::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == 0 {
                trailhead_ratings.push(get_trailhead_rating(grid, (i, j)));
            }
        }
    }
    trailhead_ratings
}

#[inline(always)]
fn get_trailhead_score(grid: &Grid<u8>, trailhead: (usize, usize)) -> usize {
    let mut trailhead_score = 0;

    let mut s = Vec::with_capacity(grid.rows * grid.cols);
    s.push(trailhead);
    let mut discovered = Grid::with_capacity_and_default(grid.rows, grid.cols, false);

    while let Some(v) = s.pop() {
        if discovered[v] {
            continue;
        }

        discovered[v] = true;

        if grid[v] == 9 {
            trailhead_score += 1;
            continue;
        }

        for adj in grid.adjacent_indices(v, false) {
            if grid[adj].checked_sub(grid[v]) == Some(1) {
                s.push(adj);
            }
        }
    }

    trailhead_score
}

#[inline(always)]
fn get_trailhead_rating(grid: &Grid<u8>, trailhead: (usize, usize)) -> usize {
    let mut trailhead_rating = 0;

    let mut s = Vec::with_capacity(grid.rows * grid.cols);
    s.push(trailhead);

    while let Some(v) = s.pop() {
        if grid[v] == 9 {
            trailhead_rating += 1;
            continue;
        }

        for adj in grid.adjacent_indices(v, false) {
            if grid[adj].checked_sub(grid[v]) == Some(1) {
                s.push(adj);
            }
        }
    }

    trailhead_rating
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = "89010123
                                                      78121874
                                                      87430965
                                                      96549874
                                                      45678903
                                                      32019012
                                                      01329801
                                                      10456732";

    const TEST_TRAILHEAD_RATING_INPUT: &str = ".....0.
                                                       ..4321.
                                                       ..5..2.
                                                       ..6543.
                                                       ..7..4.
                                                       ..8765.
                                                       ..9....";

    const TEST_TRAILHEAD_RATING_INPUT_SINGLE: &str = "..90..9
                                                              ...1.98
                                                              ...2..7
                                                              6543456
                                                              765.987
                                                              876....
                                                              987....";
    const TEST_TRAILHEAD_RATING_HIGH_SCORE: &str = "012345
                                                            123456
                                                            234567
                                                            345678
                                                            4.6789
                                                            56789.";

    #[test]
    fn test_get_total_trailhead_score() {
        assert_eq!(get_total_trailhead_score(TEST_INPUT), 36);
    }

    #[rstest]
    #[case(TEST_INPUT, 81)]
    #[case(TEST_TRAILHEAD_RATING_INPUT, 3)]
    #[case(TEST_TRAILHEAD_RATING_INPUT_SINGLE, 13)]
    #[case(TEST_TRAILHEAD_RATING_HIGH_SCORE, 227)]

    fn test_get_total_trailhead_rating(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(get_total_trailhead_rating(input), expected);
    }
}
