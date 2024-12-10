use aoc_2024::file_utils::read_to_string;
use aoc_2024::q6::get_guard_path_length;

fn main() {
    let input = read_to_string("data/q6.txt").expect("Error reading file");
    let result = get_guard_path_length(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_guard_path_length() {
        let input = read_to_string("data/q6.txt").expect("Error reading file");
        let result = get_guard_path_length(&input);
        assert_eq!(result, 4988);
    }
}
