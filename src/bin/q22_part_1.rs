use aoc_2024::q22::sum_nth_secret_number;

fn main() {
    let input = include_str!("../../data/q22.txt");
    let result = sum_nth_secret_number(input, 2000);
    println!("{}", result);
}
