use aoc_2024::q25::count_fitting_key_locks;

fn main() {
    let input = include_str!("../../data/q25.txt");
    let result = count_fitting_key_locks(input);
    println!("{}", result);
}
