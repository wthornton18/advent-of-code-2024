use std::{fmt::format, hash::Hash};

use alias::Alias;
use hashbrown::{HashMap, HashSet};
use op::{Op, SimpleGate};

mod alias;
mod heuristic;
mod op;

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<SimpleGate>) {
    let mut initial_input = HashMap::new();
    let mut gates = Vec::new();
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

            initial_input.insert(word.trim().to_string(), value == "1");
        } else {
            let (input, output) = line.split_once(" -> ").expect("Failed to parse equation");

            let input_parts = input.split_whitespace().collect::<Vec<_>>();

            let op = match input_parts[1] {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!("Invalid operator"),
            };

            gates.push(SimpleGate::new(input_parts[0], input_parts[2], op, output));
        }
    }

    (initial_input, gates)
}

pub fn swap_wires(input: &str) -> Vec<String> {
    let (_, mut gates) = parse_input(input);

    let mut renamed_wires = rename_gate_wires(&mut gates);

    let (outputs, renames) = find_outputs_to_swap(&mut gates);

    renamed_wires.extend(renames);

    let mut inverse_renames = HashMap::new();

    for (k, v) in renamed_wires.iter() {
        inverse_renames.insert(v.clone(), k.clone());
    }

    let mut outputs = outputs
        .iter()
        .map(|output| inverse_renames.get(output).unwrap().to_owned())
        .collect::<Vec<_>>();

    outputs.dedup();
    outputs.sort();

    outputs
}

fn find_outputs_to_swap(gates: &mut [SimpleGate]) -> (Vec<String>, HashMap<String, String>) {
    let mut outputs = Vec::new();
    let mut additional_wire_renames = HashMap::new();

    for i in 1..=46 {
        for heuristic in &[
            and_heuristic,
            xor_heuristic,
            carry_intermediate_heuristic,
            carry_heuristic,
            z_heuristic,
        ] {
            if let Some((expected, non_matching)) = heuristic(gates, i) {
                let renames = swap_outputs(gates, &expected, &non_matching);
                additional_wire_renames.extend(renames);
                outputs.push(expected);
                outputs.push(non_matching);
            }
        }
    }

    (outputs, additional_wire_renames)
}

fn swap_outputs(
    gates: &mut [SimpleGate],
    expected: &str,
    non_matching: &str,
) -> HashMap<String, String> {
    // Find gate with the expected output and swap it with the generated output
    // And find the gate with the generated output and swap it with the expected output

    for gate in gates.iter_mut() {
        if gate.c == expected {
            gate.c = non_matching.to_string();
        } else if gate.c == non_matching {
            gate.c = expected.to_string();
        }
    }

    rename_gate_wires(gates)
}

fn and_heuristic(gates: &[SimpleGate], n: usize) -> Option<(String, String)> {
    let expected_output = format!("AND{:0>2}", n);
    let input_1 = format!("x{:0>2}", n);
    let input_2 = format!("y{:0>2}", n);

    let mut non_matching_output = None;

    for gate in gates.iter().filter(|g| g.op == Op::And) {
        let a = gate.a.as_str();
        let b = gate.b.as_str();

        if (a == input_1 && b == input_2) || (a == input_2 && b == input_1) {
            if gate.c == expected_output {
                return None;
            }
            non_matching_output = Some(gate.c.clone());
            break;
        }
    }

    let non_matching_output = non_matching_output?;

    Some((expected_output, non_matching_output))
}

fn xor_heuristic(gates: &[SimpleGate], n: usize) -> Option<(String, String)> {
    let expected_output = format!("XOR{:0>2}", n);
    let input_1 = format!("x{:0>2}", n);
    let input_2 = format!("y{:0>2}", n);

    let mut non_matching_output = None;

    for gate in gates.iter().filter(|g| g.op == Op::Xor) {
        let a = gate.a.as_str();
        let b = gate.b.as_str();

        if (a == input_1 && b == input_2) || (a == input_2 && b == input_1) {
            if gate.c == expected_output {
                return None;
            }
            non_matching_output = Some(gate.c.clone());
            break;
        }
    }

    let non_matching_output = non_matching_output?;

    Some((expected_output, non_matching_output))
}
fn carry_intermediate_heuristic(gates: &[SimpleGate], n: usize) -> Option<(String, String)> {
    let expected_output = format!("CARRY_INTERMEDIATE{:0>2}", n);
    let input_1 = format!("XOR{:0>2}", n);
    let input_2 = format!("CARRY{:0>2}", n - 1);

    let mut non_matching_output = None;

    for gate in gates.iter().filter(|g| g.op == Op::And) {
        let a = gate.a.as_str();
        let b = gate.b.as_str();

        if (a == input_1 && b == input_2) || (a == input_2 && b == input_1) {
            if gate.c == expected_output {
                return None;
            }
            non_matching_output = Some(gate.c.clone());
            break;
        }
    }

    let non_matching_output = non_matching_output?;

    Some((expected_output, non_matching_output))
}

fn carry_heuristic(gates: &[SimpleGate], n: usize) -> Option<(String, String)> {
    let expected_output = format!("CARRY{:0>2}", n);
    let input_1 = format!("CARRY_INTERMEDIATE{:0>2}", n);
    let input_2 = format!("AND{:0>2}", n);

    let mut non_matching_output = None;

    for gate in gates.iter().filter(|g| g.op == Op::Or) {
        let a = gate.a.as_str();
        let b = gate.b.as_str();

        if (a == input_1 && b == input_2) || (a == input_2 && b == input_1) {
            if gate.c == expected_output {
                return None;
            }
            non_matching_output = Some(gate.c.clone());
            break;
        }
    }

    let non_matching_output = non_matching_output?;

    Some((expected_output, non_matching_output))
}

fn z_heuristic(gates: &[SimpleGate], n: usize) -> Option<(String, String)> {
    let expected_output = format!("z{:0>2}", n);
    let input_1 = format!("CARRY{:0>2}", n);
    let input_2 = format!("XOR{:0>2}", n);

    let mut non_matching_output = None;

    for gate in gates.iter().filter(|g| g.op == Op::Xor) {
        let a = gate.a.as_str();
        let b = gate.b.as_str();

        if (a == input_1 && b == input_2) || (a == input_2 && b == input_1) {
            if gate.c == expected_output {
                return None;
            }
            non_matching_output = Some(gate.c.clone());
            break;
        }
    }

    let non_matching_output = non_matching_output?;

    Some((expected_output, non_matching_output))
}

fn rename_gate_wires(gates: &mut [SimpleGate]) -> HashMap<String, String> {
    let mut renamed_wires = HashMap::new();

    Alias::new("x(N)", Op::And, "y(N)", "AND(N)").alias(gates, &mut renamed_wires);
    Alias::new("x(N)", Op::Xor, "y(N)", "XOR(N)").alias(gates, &mut renamed_wires);

    let and_00_original_wire = renamed_wires
        .iter()
        .find(|(_, v)| *v == "AND00")
        .map(|(k, _)| k.clone())
        .expect("Failed to find original wire for AND00");

    // Rename all occurences of AND00 and the original wire to CARRY00

    renamed_wires.insert(and_00_original_wire.clone(), "CARRY00".to_string());

    for gate in gates.iter_mut() {
        if gate.a == "AND00" {
            gate.a = "CARRY00".to_string();
        }
        if gate.b == "AND00" {
            gate.b = "CARRY00".to_string();
        }

        if gate.c == "AND00" {
            gate.c = "CARRY00".to_string();
        }
    }

    Alias::new("XOR(N)", Op::And, "CARRY(N-1)", "CARRY_INTERMEDIATE(N)")
        .alias(gates, &mut renamed_wires);

    Alias::new("AND(N)", Op::Or, "CARRY_INTERMEDIATE(N)", "CARRY(N)")
        .alias(gates, &mut renamed_wires);

    for g in gates {
        println!("{}", g);
    }

    renamed_wires
}

pub fn compute_z_number(input: &str) -> usize {
    let (initial_wires, gates) = parse_input(input);

    let output_wires = simulate(initial_wires, gates);

    let mut z = 0;

    for (k, v) in output_wires.iter() {
        if k.starts_with("z") && *v {
            let num = k[1..].parse::<usize>().unwrap();
            z |= 1 << num;
        }
    }

    z
}

fn simulate<'a>(mut input: HashMap<String, bool>, gates: Vec<SimpleGate>) -> HashMap<String, bool> {
    let mut seen = HashSet::new();
    loop {
        let mut updated = false;

        for gate in gates.iter() {
            let g_a = gate.a.as_str();
            let g_b = gate.b.as_str();
            if input.contains_key(g_a) && input.contains_key(g_b) && !seen.contains(&gate) {
                let a = input[g_a];
                let b = input[g_b];

                let result = gate.evaluate(a, b);

                updated = true;

                input.insert(gate.c.clone(), result);
                seen.insert(gate);
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
