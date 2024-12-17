use std::usize;

use itertools::repeat_n;
use robot::{Quadrant, Robot};

use crate::grid::Grid;

mod robot;

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        robots.push(line.parse().unwrap())
    }

    robots
}

pub fn get_lowest_safety_factor_idx(input: &str, constraints: (i128, i128)) -> usize {
    let mut robots = parse_input(input);

    let mut lowest_safety_score = usize::MAX;
    let mut idx = 0;

    for i in 0..((constraints.0 * constraints.1) as usize) {
        for robot in robots.iter_mut() {
            robot.move_robot(constraints, 1);
        }

        let safety_factor = get_safety_factor_for_iteration(&robots, constraints);

        if safety_factor < lowest_safety_score {
            lowest_safety_score = safety_factor;
            idx = i;
        }
    }

    idx
}

pub fn get_safety_factor(input: &str, n: i128, constraints: (i128, i128)) -> usize {
    let mut robots = parse_input(input);

    for robot in robots.iter_mut() {
        robot.move_robot(constraints, n);
    }

    get_safety_factor_for_iteration(&robots, constraints)
}

fn get_safety_factor_for_iteration(robots: &[Robot], constraints: (i128, i128)) -> usize {
    use Quadrant::*;
    let mut top_left = 0;
    let mut bottom_left = 0;
    let mut top_right = 0;
    let mut bottom_right = 0;

    for robot in robots {
        match robot.quadrant(constraints) {
            Some(TopLeft) => top_left += 1,
            Some(TopRight) => top_right += 1,
            Some(BottomLeft) => bottom_left += 1,
            Some(BottomRight) => bottom_right += 1,
            _ => continue,
        }
    }

    top_left * bottom_left * top_right * bottom_right
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_get_safety_factor() {
        let result = get_safety_factor(TEST_INPUT, 100, (11, 7));
        assert_eq!(result, 12);
    }
}
