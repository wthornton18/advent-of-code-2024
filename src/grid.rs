use std::{
    fmt::Display,
    hash::Hash,
    ops::{BitOr, BitOrAssign, Index, IndexMut},
};

use crate::{a_star_search::AStarSearch, vec2::Vec2};

#[derive(Default, Clone)]
pub struct Grid<K: Clone> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<K>,
}

impl<K: Display + Clone> Display for Grid<K> {
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

impl<K: Clone> Grid<K> {
    pub fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }
}

impl<K: Clone> Grid<K> {
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

impl<K: Clone> Grid<K> {
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
                    Some(self[(i, j)].clone())
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
                    result[(i, j)] = self[(i, j)].clone();
                }
            }
        }

        result
    }

    pub fn get(&self, (row, col): (usize, usize)) -> Option<K> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self[(row, col)].clone())
        }
    }

    pub fn deltas(&self, (row, col): (usize, usize), diagonals: bool) -> Vec<(i32, i32)> {
        // Gives only valid deltas from the given position

        let mut deltas = Vec::with_capacity(8);

        if row > 0 {
            deltas.push((-1, 0));
        }

        if col > 0 {
            deltas.push((0, -1));
        }

        if row + 1 < self.rows {
            deltas.push((1, 0));
        }

        if col + 1 < self.cols {
            deltas.push((0, 1));
        }

        if diagonals {
            if row > 0 && col > 0 {
                deltas.push((-1, -1));
            }

            if row > 0 && col + 1 < self.cols {
                deltas.push((-1, 1));
            }

            if row + 1 < self.rows && col > 0 {
                deltas.push((1, -1));
            }

            if row + 1 < self.rows && col + 1 < self.cols {
                deltas.push((1, 1));
            }
        }

        deltas
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

    pub fn is_empty(&self) -> bool {
        self.rows == 0 || self.cols == 0
    }

    pub fn swap(&mut self, pos: (usize, usize), other: (usize, usize)) {
        self.data
            .swap(pos.0 * self.cols + pos.1, other.0 * self.cols + other.1);
    }

    pub fn rotate_90(&self) -> Self {
        let mut result = Grid::with_capacity(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(j, self.rows - i - 1)] = self[(i, j)].clone();
            }
        }

        result
    }
}

impl<K: Clone> Index<(usize, usize)> for Grid<K> {
    type Output = K;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<K: Clone> Index<Vec2<usize>> for Grid<K> {
    type Output = K;

    fn index(&self, Vec2 { x, y }: Vec2<usize>) -> &Self::Output {
        &self.data[x * self.cols + y]
    }
}

impl<K: Clone> Index<std::ops::RangeFrom<usize>> for Grid<K> {
    type Output = [K];

    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.data[range.start..]
    }
}

impl<K: Clone> IndexMut<(usize, usize)> for Grid<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

impl<K: Clone> IndexMut<Vec2<usize>> for Grid<K> {
    fn index_mut(&mut self, Vec2 { x, y }: Vec2<usize>) -> &mut Self::Output {
        &mut self.data[x * self.cols + y]
    }
}

impl<K: Clone> IntoIterator for Grid<K> {
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
pub struct GridIterator<K: Clone> {
    grid: Grid<K>,
    current_row: usize,
    current_col: usize,
}

impl<K: Clone> Iterator for GridIterator<K> {
    type Item = ((usize, usize), K);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row == self.grid.rows {
            return None;
        }

        let result = (
            (self.current_row, self.current_col),
            self.grid[(self.current_row, self.current_col)].clone(),
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

impl BitOrAssign for Grid<bool> {
    fn bitor_assign(&mut self, rhs: Self) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self[(i, j)] |= rhs[(i, j)];
            }
        }
    }
}

impl Grid<bool> {
    pub fn shannon_entropy(&self) -> f64 {
        let mut count = 0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self[(i, j)] {
                    count += 1;
                }
            }
        }

        let total = self.rows * self.cols;
        let p = count as f64 / total as f64;
        let q = 1.0 - p;

        if p == 0.0 || q == 0.0 {
            return 0.0;
        }

        -p * p.log2() - q * q.log2()
    }
}

pub trait Tile {
    fn traversable(&self) -> bool;
    fn cost_from<V>(&self, other: V) -> f64
    where
        V: Tile;
}

impl<K> AStarSearch for Grid<K>
where
    K: Clone + Tile + Eq + Hash,
{
    type Node = (usize, usize);

    fn weighted_neighbours(&self, node: &Self::Node) -> Option<Vec<(Self::Node, f64)>> {
        let mut neighbours = Vec::new();
        let tile = self.get(*node)?;
        for delta in self.deltas(*node, false) {
            let new_node = (
                (node.0 as i32 + delta.0) as usize,
                (node.1 as i32 + delta.1) as usize,
            );

            if let Some(new_tile) = self.get(new_node) {
                if new_tile.traversable() {
                    neighbours.push((new_node, new_tile.cost_from(tile.clone())));
                }
            }
        }

        Some(neighbours)
    }
}
