use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub struct Constraint {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Constraint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (x, y) = s.split_once('|').ok_or(())?;
        let x = x.trim().parse().map_err(|_| ())?;
        let y = y.trim().parse().map_err(|_| ())?;
        Ok(Self { x, y })
    }
}

impl Constraint {
    pub fn is_valid(&self, values: &[usize]) -> bool {
        let x_i = values.iter().position(|&v| v == self.x);
        if x_i.is_none() {
            return true;
        }

        let y_i = values.iter().position(|&v| v == self.y);
        if y_i.is_none() {
            return true;
        }
        x_i.unwrap() < y_i.unwrap()
    }

    pub fn is_relevant(&self, values: &[usize]) -> bool {
        values.contains(&self.x) || values.contains(&self.y)
    }
}
