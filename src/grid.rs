use std::{
    fmt::Display,
    ops::{BitOr, Index, IndexMut},
};

#[derive(Default, Clone)]
pub struct Grid<K: Copy> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<K>,
}

impl<K: Display + Copy> Display for Grid<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self[(i, j)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<K: Copy> Grid<K> {
    pub fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }
}

impl<K: Copy> Grid<K> {
    pub fn with_capacity(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: Vec::with_capacity(rows * cols),
        }
    }

    pub fn with_capacity_and_default(rows: usize, cols: usize, default: K) -> Self {
        Self {
            rows,
            cols,
            data: vec![default; rows * cols],
        }
    }
}

impl<K: Copy> Grid<K> {
    pub fn push(&mut self, row: &[K]) {
        if self.cols == 0 {
            self.cols = row.len();
        }
        assert_eq!(self.cols, row.len());
        self.data.extend_from_slice(row);
        self.rows += 1;
    }

    pub fn mask(&self, mask: Grid<bool>) -> Grid<Option<K>> {
        assert_eq!(self.rows, mask.rows);
        assert_eq!(self.cols, mask.cols);

        let mut result = Grid::with_capacity(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] = if mask[(i, j)] {
                    Some(self[(i, j)])
                } else {
                    None
                };
            }
        }

        result
    }

    pub fn mask_or_default(&self, mask: Grid<bool>, default: K) -> Grid<K> {
        assert_eq!(self.rows, mask.rows);
        assert_eq!(self.cols, mask.cols);

        let mut result = Grid::with_capacity_and_default(self.rows, self.cols, default);

        for i in 0..self.rows {
            for j in 0..self.cols {
                if mask[(i, j)] {
                    result[(i, j)] = self[(i, j)];
                }
            }
        }

        result
    }

    pub fn get(&self, (row, col): (usize, usize)) -> Option<K> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self[(row, col)])
        }
    }

    pub fn adjacent_indices(
        &self,
        (row, col): (usize, usize),
        diagonals: bool,
    ) -> Vec<(usize, usize)> {
        let mut indices = Vec::with_capacity(8);

        if row > 0 {
            indices.push((row - 1, col));
        }

        if col > 0 {
            indices.push((row, col - 1));
        }

        if row + 1 < self.rows {
            indices.push((row + 1, col));
        }

        if col + 1 < self.cols {
            indices.push((row, col + 1));
        }

        if diagonals {
            if row > 0 && col > 0 {
                indices.push((row - 1, col - 1));
            }

            if row > 0 && col + 1 < self.cols {
                indices.push((row - 1, col + 1));
            }

            if row + 1 < self.rows && col > 0 {
                indices.push((row + 1, col - 1));
            }

            if row + 1 < self.rows && col + 1 < self.cols {
                indices.push((row + 1, col + 1));
            }
        }

        indices
    }
}

impl<K: Copy> Index<(usize, usize)> for Grid<K> {
    type Output = K;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<K: Copy> Index<std::ops::RangeFrom<usize>> for Grid<K> {
    type Output = [K];

    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.data[range.start..]
    }
}

impl<K: Copy> IndexMut<(usize, usize)> for Grid<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl<K: Copy> IntoIterator for Grid<K> {
    type Item = ((usize, usize), K);
    type IntoIter = GridIterator<K>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            current_row: 0,
            current_col: 0,
        }
    }
}
pub struct GridIterator<K: Copy> {
    grid: Grid<K>,
    current_row: usize,
    current_col: usize,
}

impl<K: Copy> Iterator for GridIterator<K> {
    type Item = ((usize, usize), K);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row == self.grid.rows {
            return None;
        }

        let result = (
            (self.current_row, self.current_col),
            self.grid[(self.current_row, self.current_col)],
        );

        self.current_col += 1;
        if self.current_col == self.grid.cols {
            self.current_col = 0;
            self.current_row += 1;
        }

        Some(result)
    }
}

impl BitOr for Grid<bool> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        let mut result = Grid::with_capacity_and_default(self.rows, self.cols, false);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] = self[(i, j)] | rhs[(i, j)];
            }
        }

        result
    }
}