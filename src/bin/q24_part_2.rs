use aoc_2024::q24::swap_wires;

fn main() {
    let input = include_str!("../../data/q24.txt");

    let wires = swap_wires(input);

    for w in wires {
        println!("{}", w);
    }
}
