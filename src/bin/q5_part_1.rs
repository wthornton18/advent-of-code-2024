use aoc_2024::file_utils::read_to_string;
use aoc_2024::q5::get_total_valid_middle_page_numbers;

fn main() {
    let input = read_to_string("data/q5.txt").expect("Error reading file");
    let result = get_total_valid_middle_page_numbers(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_valid_middle_page_numbers() {
        let input = read_to_string("data/q5.txt").expect("Error reading file");
        let result = get_total_valid_middle_page_numbers(&input);
        assert_eq!(result, 4609);
    }
}
