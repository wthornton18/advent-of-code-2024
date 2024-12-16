#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Equation {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

pub fn get_total_number_of_tokens(input: &str) -> usize {
    let equations = parse_input(input);

    equations
        .iter()
        .filter_map(solve)
        .map(|(x, y)| (3 * x) + y)
        .sum()
}

pub fn get_total_number_of_tokens_position_correction(input: &str) -> usize {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter_map(|(mut eq_1, mut eq_2)| {
            eq_1.c += 10000000000000;
            eq_2.c += 10000000000000;

            solve(&(eq_1, eq_2))
        })
        .map(|(x, y)| (3 * x) + y)
        .sum()
}

/// Uses Cramer's rule (or just in general the solution to a 2x2 Matrix sum expressed by)
/// Ax = c
/// E_1: a_1A + b_1B = c_1
/// E_2: a_2A + b_2B = c_2
/// D   = |a_1 b_1|
///       |a_2 b_2|
/// D_A = |c_1 b_1|
///       |c_2 b_2|
/// D_B = |a_1 c_1|
///       |a_2 c_2|
///
/// A = det(D_A)/det(D)
/// B = det(D_B)/det(D)
fn solve((eq_1, eq_2): &(Equation, Equation)) -> Option<(usize, usize)> {
    let determinant = (eq_1.a * eq_2.b) as i128 - (eq_1.b * eq_2.a) as i128;
    let determinant_a = (eq_1.c * eq_2.b) as i128 - (eq_1.b * eq_2.c) as i128;
    let determinant_b = (eq_1.a * eq_2.c) as i128 - (eq_1.c * eq_2.a) as i128;

    if determinant == 0 || (determinant_a % determinant != 0) || (determinant_b % determinant != 0)
    {
        return None;
    }
    let a = (determinant_a / determinant) as usize;
    let b = (determinant_b / determinant) as usize;

    Some((a, b))
}

fn parse_input(input: &str) -> Vec<(Equation, Equation)> {
    let mut equations = Vec::new();

    let mut eq_x = Equation::default();
    let mut eq_y = Equation::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(suffix) = line.strip_prefix("Button A:") {
            let suffix = suffix.trim();
            let (x, y) = suffix.split_once(", ").unwrap();
            eq_x.a = x.strip_prefix("X+").unwrap().parse().unwrap();
            eq_y.a = y.strip_prefix("Y+").unwrap().parse().unwrap();
        } else if let Some(suffix) = line.strip_prefix("Button B:") {
            let suffix = suffix.trim();
            let (x, y) = suffix.split_once(", ").unwrap();
            eq_x.b = x.strip_prefix("X+").unwrap().parse().unwrap();
            eq_y.b = y.strip_prefix("Y+").unwrap().parse().unwrap();
        } else if let Some(suffix) = line.strip_prefix("Prize:") {
            let suffix = suffix.trim();
            let (x, y) = suffix.split_once(", ").unwrap();
            eq_x.c = x.strip_prefix("X=").unwrap().parse().unwrap();
            eq_y.c = y.strip_prefix("Y=").unwrap().parse().unwrap();
        }

        if eq_x.c != 0 && eq_y.c != 0 {
            equations.push((eq_x, eq_y));
            eq_x = Equation::default();
            eq_y = Equation::default();
        }
    }

    equations
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_get_total_number_of_tokens() {
        assert_eq!(get_total_number_of_tokens(TEST_INPUT), 480);
    }
}
