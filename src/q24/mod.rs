use hashbrown::{HashMap, HashSet};
use op::Equation;

mod op;

fn parse_input<'a>(input: &'a str) -> (HashMap<&'a str, bool>, Vec<Equation<'a>>) {
    let mut initial_input = HashMap::new();
    let mut equations = Vec::new();
    let mut initial_input_section = true;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            initial_input_section = initial_input.is_empty();
            continue;
        }

        if initial_input_section {
            let (word, value) = line
                .split_once(": ")
                .expect("Failed to parse initial input");

            initial_input.insert(word.trim(), value == "1");
        } else {
            let (input, output) = line.split_once(" -> ").expect("Failed to parse equation");

            let input_parts = input.split_whitespace().collect::<Vec<_>>();

            let op = match input_parts[1] {
                "AND" => op::Op::And,
                "OR" => op::Op::Or,
                "XOR" => op::Op::Xor,
                _ => panic!("Invalid operator"),
            };

            equations.push(Equation::new(input_parts[0], input_parts[2], op, output));
        }
    }

    (initial_input, equations)
}

pub fn compute_z_number<'a>(input: &'a str) -> usize {
    let (mut initial_wires, equations) = parse_input(input);

    let output_wires = simulate(initial_wires, equations);

    let mut z = 0;

    for (k, v) in output_wires.iter() {
        if k.starts_with("z") && *v {
            let num = k[1..].parse::<usize>().unwrap();
            z |= 1 << num;
        }
    }

    z
}

fn simulate<'a>(
    mut input: HashMap<&'a str, bool>,
    equations: Vec<Equation<'a>>,
) -> HashMap<&'a str, bool> {
    let mut seen = HashSet::new();
    loop {
        let mut updated = false;

        for equation in equations.iter() {
            if input.contains_key(equation.w_1)
                && input.contains_key(equation.w_2)
                && !seen.contains(&equation)
            {
                let w_1 = input[equation.w_1];
                let w_2 = input[equation.w_2];

                let result = equation.evaluate(w_1, w_2);

                updated = true;

                input.insert(equation.out, result);
                seen.insert(equation);
            }
        }

        if !updated {
            break;
        }
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_compute_z_number() {
        assert_eq!(compute_z_number(TEST_INPUT), 2024);
    }
}
