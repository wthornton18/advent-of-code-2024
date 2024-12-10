use equation::{Equation, ValidOperation};
use rayon::prelude::*;

mod equation;

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            line.parse().unwrap()
        })
        .collect()
}

pub fn get_satisfiable_equation_target_sum_add_mul(input: &str) -> usize {
    let equations = parse_input(input);
    let equations =
        get_satisfiable_equations(&equations, &[ValidOperation::Add, ValidOperation::Multiply]);
    equations.iter().map(|eq| eq.target).sum()
}

pub fn get_satisfiable_equation_target_all(input: &str) -> usize {
    let equations = parse_input(input);
    let equations = get_satisfiable_equations(
        &equations,
        &[
            ValidOperation::Add,
            ValidOperation::Multiply,
            ValidOperation::Concatenate,
        ],
    );
    equations.iter().map(|eq| eq.target).sum()
}

pub fn get_satisfiable_equation_target_all_parallel(input: &str) -> usize {
    let equations = parse_input(input);
    let equations = get_satisfiable_equations(
        &equations,
        &[
            ValidOperation::Add,
            ValidOperation::Multiply,
            ValidOperation::Concatenate,
        ],
    );
    equations.par_iter().map(|eq| eq.target).sum()
}

fn get_satisfiable_equations(
    equations: &[Equation],
    operations: &[ValidOperation],
) -> Vec<Equation> {
    equations
        .iter()
        .filter(|eq| eq.satisfiable_configurations(operations) > 0)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
                                      3267: 81 40 27
                                      83: 17 5
                                      156: 15 6
                                      7290: 6 8 6 15
                                      161011: 16 10 13
                                      192: 17 8 14
                                      21037: 9 7 18 13
                                      292: 11 6 16 20";

    #[test]
    fn test_get_satisfiable_equation_target_sum_add_mul() {
        let result = get_satisfiable_equation_target_sum_add_mul(TEST_INPUT);
        assert_eq!(result, 190 + 3267 + 292);
    }

    #[test]
    fn test_get_satisfiable_equation_target_all() {
        let result = get_satisfiable_equation_target_all(TEST_INPUT);
        assert_eq!(result, 190 + 3267 + 156 + 7290 + 192 + 292);
    }
}
