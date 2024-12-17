
use crate::grid::Grid;

const XMAS_DELTAS: [[(i32, i32); 3]; 8] = [
    [(0, 1), (0, 2), (0, 3)],
    [(1, 0), (2, 0), (3, 0)],
    [(0, -1), (0, -2), (0, -3)],
    [(-1, 0), (-2, 0), (-3, 0)],
    [(1, 1), (2, 2), (3, 3)],
    [(1, -1), (2, -2), (3, -3)],
    [(-1, 1), (-2, 2), (-3, 3)],
    [(-1, -1), (-2, -2), (-3, -3)],
];

const X_MAS_DELTAS: [[(i32, i32); 2]; 2] = [[(-1, 1), (1, -1)], [(1, 1), (-1, -1)]];

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn count_total_xmas(input: &str) -> i32 {
    let grid = parse_input(input);

    let mut count = 0;

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == 'X' {
                count += count_xmas(&grid, i, j);
            }
        }
    }

    count
}

pub fn count_total_x_mas(input: &str) -> i32 {
    let grid = parse_input(input);

    let mut count = 0;

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid[(i, j)] == 'A' && count_x_mas(&grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

pub fn parse_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new();

    for line in input.lines() {
        let line = line.trim();
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        grid.push(&row);
    }

    grid
}

fn count_xmas(grid: &Grid<char>, i: usize, j: usize) -> i32 {
    let mut count = 0;

    let rows = grid.rows as i32;
    let cols = grid.cols as i32;

    let i = i as i32;
    let j = j as i32;

    for delta in XMAS_DELTAS.iter() {
        let mut found = true;
        for (k, (di, dj)) in delta.iter().enumerate() {
            let k = k + 1;
            let ni = i + *di;
            let nj = j + *dj;

            if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
                found = false;
                break;
            }

            if grid[(ni as usize, nj as usize)] != XMAS[k] {
                found = false;
                break;
            }
        }

        if found {
            count += 1;
        }
    }

    count
}

fn count_x_mas(grid: &Grid<char>, i: usize, j: usize) -> bool {
    let rows = grid.rows as i32;
    let cols = grid.cols as i32;

    let i = i as i32;
    let j = j as i32;

    let mut found = true;

    for deltas in X_MAS_DELTAS.iter() {
        let mut characters = ['_', '_'];
        for (k, (di, dj)) in deltas.iter().enumerate() {
            let ni = i + *di;
            let nj = j + *dj;

            if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
                break;
            }

            characters[k] = grid[(ni as usize, nj as usize)];
        }

        if !(characters.contains(&'M') && characters.contains(&'S')) {
            found = false;
            break;
        }
    }

    found
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_xmas() {
        let input = "MMMSXXMASM
                            MSAMXMSMSA
                            AMXSXMAAMM
                            MSAMASMSMX
                            XMASAMXAMM
                            XXAMMXXAMA
                            SMSMSASXSS
                            SAXAMASAAA
                            MAMMMXMMMM
                            MXMXAXMASX";

        assert_eq!(count_total_xmas(input), 18);
    }

    #[test]
    fn test_count_x_mas() {
        let input = "MMMSXXMASM
                            MSAMXMSMSA
                            AMXSXMAAMM
                            MSAMASMSMX
                            XMASAMXAMM
                            XXAMMXXAMA
                            SMSMSASXSS
                            SAXAMASAAA
                            MAMMMXMMMM
                            MXMXAXMASX";

        assert_eq!(count_total_x_mas(input), 9);
    }
}
