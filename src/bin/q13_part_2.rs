use aoc_2024::file_utils::read_to_string;
use aoc_2024::q13::get_total_number_of_tokens_position_correction;

fn main() {
    let input = read_to_string("data/q13.txt").expect("Failed to read file");
    let total_tokens = get_total_number_of_tokens_position_correction(&input);
    println!("Result: {}", total_tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_number_of_tokens_position_correction() {
        let input = read_to_string("data/q13.txt").expect("Failed to read file");
        let result = get_total_number_of_tokens_position_correction(&input);
        assert_eq!(result, 36758);
    }
}
