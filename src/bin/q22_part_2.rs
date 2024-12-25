use aoc_2024::q22::get_max_bananas;

fn main() {
    let input = include_str!("../../data/q22.txt");
    let result = get_max_bananas(input, 2000);
    println!("{}", result);
}
