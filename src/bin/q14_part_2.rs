use aoc_2024::file_utils::read_to_string;
use aoc_2024::q14::get_lowest_safety_factor;

fn main() {
    let input = read_to_string("data/q14.txt").expect("Failed to read file");
    // This one is non-deterministic and requires visual inspect
    let result = get_lowest_safety_factor(&input, (101, 103));
    println!("Result: {}", result);
}
