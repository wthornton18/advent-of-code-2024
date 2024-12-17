use aoc_2024::file_utils::read_to_string;
use aoc_2024::q14::get_lowest_safety_factor_idx;

fn main() {
    let input = read_to_string("data/q14.txt").expect("Failed to read file");
    let result = get_lowest_safety_factor_idx(&input, (101, 103));
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lowest_safety_factor_idx() {
        let input = read_to_string("data/q14.txt").expect("Failed to read file");
        let result = get_lowest_safety_factor_idx(&input, (101, 103));
        assert_eq!(result, 7686);
    }
}
