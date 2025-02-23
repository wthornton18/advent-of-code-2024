use aoc_2024::file_utils::read_to_string;
use aoc_2024::q2::count_safe_reports;

fn main() {
    let input = read_to_string("data/q2.txt").expect("Error reading file");
    let result = count_safe_reports(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe_reports() {
        let input = read_to_string("data/q2.txt").expect("Error reading file");
        let result = count_safe_reports(&input);
        assert_eq!(result, 631);
    }
}
