use std::str::FromStr;

use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy)]
pub enum ValidOperation {
    Add,
    Multiply,
    Concatenate,
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub target: usize,
    pub numbers: Vec<usize>,
}

impl Equation {
    pub fn satisfiable_configurations(&self, valid_ops: &[ValidOperation]) -> usize {
        let total_ops = self.numbers.len() - 1;

        let ops = repeat_n(valid_ops, total_ops)
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        let mut count = 0;

        for configuration in ops {
            let mut result = self.numbers[0];
            for (i, &op) in configuration.iter().enumerate() {
                match op {
                    ValidOperation::Add => result += self.numbers[i + 1],
                    ValidOperation::Multiply => result *= self.numbers[i + 1],
                    ValidOperation::Concatenate => {
                        let str_result = format!("{}{}", result, self.numbers[i + 1]);
                        result = str_result.parse().unwrap();
                    }
                };
            }

            if result == self.target {
                count += 1;
            }
        }

        count
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
