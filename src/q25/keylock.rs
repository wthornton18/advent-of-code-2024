use std::str::FromStr;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyLock {
    Key([u8; 5]),
    Lock([u8; 5]),
}

impl KeyLock {
    pub fn is_key(&self) -> bool {
        matches!(self, KeyLock::Key(_))
    }

    pub fn fits(&self, other: KeyLock) -> bool {
        use KeyLock::*;
        match (*self, other) {
            (Key(a), Lock(b)) | (Lock(b), Key(a)) => {
                for i in 0..5 {
                    let a = a[i];
                    let b = b[i];
                    if a + b > 5 {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl FromStr for KeyLock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(());
        }

        let mut g = Grid::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    _ => {
                        println!("Invalid char: {c}");
                        return Err(());
                    }
                }
            }
            g.push(row.as_slice());
        }

        let is_lock = (0..g.cols).all(|i| g[(0, i)]);

        let mut heights = [0; 5];

        for j in 0..g.cols {
            let mut height = 0;
            for i in 0..g.rows {
                if g[(i, j)] {
                    height += 1;
                }
            }
            heights[j] = height - 1;
        }

        if is_lock {
            Ok(KeyLock::Lock(heights))
        } else {
            Ok(KeyLock::Key(heights))
        }
    }
}
