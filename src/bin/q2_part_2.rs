use aoc_2024::file_utils::read_to_string;
use aoc_2024::q2::count_safe_reports_with_removal;

fn main() {
    let input = read_to_string("data/q2.txt").expect("Error reading file");
    let result = count_safe_reports_with_removal(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe_reports_with_removal() {
        let input = read_to_string("data/q2.txt").expect("Error reading file");
        let result = count_safe_reports_with_removal(&input);
        assert_eq!(result, 665);
    }
}
