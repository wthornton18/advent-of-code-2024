use aoc_2024::file_utils::read_to_string;
use aoc_2024::q17::get_total_output_from_computer;

fn main() {
    let input = read_to_string("data/q17.txt").expect("Failed to read input");
    let result = get_total_output_from_computer(&input);
    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}
