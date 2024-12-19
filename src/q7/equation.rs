use std::str::FromStr;

use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    pub fn evaluate(&self, current: usize, next: usize) -> usize {
        match self {
            Self::Add => current + next,
            Self::Multiply => current * next,
            Self::Concatenate => {
                let digit_count = next.ilog10() + 1;
                current * (10_usize.pow(digit_count)) + next
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub target: usize,
    pub numbers: Vec<usize>,
}

fn is_satisfiable(
    numbers: &[usize],
    current: usize,
    target: usize,
    valid_ops: &[Operation],
) -> bool {
    if numbers.is_empty() {
        return current == target;
    }

    if current > target {
        return false;
    }

    let next = numbers[0];
    let slice = &numbers[1..];

    valid_ops
        .iter()
        .any(|&op| is_satisfiable(slice, op.evaluate(current, next), target, valid_ops))
}

impl Equation {
    pub fn is_satisfiable(&self, valid_ops: &[Operation]) -> bool {
        if self.numbers.is_empty() {
            return false;
        }

        is_satisfiable(&self.numbers[1..], self.numbers[0], self.target, valid_ops)
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, numbers) = s.split_once(":").ok_or(())?;
        let target = target.trim().parse().map_err(|_| ())?;
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { target, numbers })
    }
}
