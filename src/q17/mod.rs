use computer::Computer;
use rayon::prelude::*;
mod computer;
mod op_code;

fn parse_input(input: &str) -> (Computer, Vec<usize>) {
    let mut computer = Computer::new();
    let mut program = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(register_a) = line.strip_prefix("Register A: ") {
            computer.set_register_a(register_a.parse().unwrap());
        } else if let Some(register_b) = line.strip_prefix("Register B: ") {
            computer.set_register_b(register_b.parse().unwrap());
        } else if let Some(register_c) = line.strip_prefix("Register C: ") {
            computer.set_register_c(register_c.parse().unwrap());
        } else if let Some(program_line) = line.strip_prefix("Program: ") {
            program = program_line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
        }
    }

    (computer, program)
}

pub fn get_total_output_from_computer(input: &str) -> Vec<usize> {
    let (mut computer, program) = parse_input(input);
    computer.run(&program)
}

pub fn find_a_for_quine_sequence(input: &str) -> usize {
    let (_, program) = parse_input(input);

    let slice = program.as_slice();

    let mut state = Vec::from([(0, 0)]);
    let mut result = usize::MAX;

    while let Some((a, i)) = state.pop() {
        if i == slice.len() {
            result = result.min(a);
            continue;
        }

        for b in 0..8 {
            let new_a = (a * 8) + b;
            let mut computer = Computer::new_with_registers(new_a, 0, 0);
            let output = computer.run(slice);

            if output[0] == slice[slice.len() - 1 - i] {
                state.push((new_a, i + 1));
            }
        }
    }

    result
}
#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const QUINE_TEST_INPUT: &str = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_get_total_output_from_computer() {
        let result = get_total_output_from_computer(TEST_INPUT);
        assert_eq!(result, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_find_a_for_quine_sequence() {
        let result = find_a_for_quine_sequence(QUINE_TEST_INPUT);
        assert_eq!(result, 117440);
    }
}
