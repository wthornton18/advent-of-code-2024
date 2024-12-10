use aoc_2024::file_utils::read_to_string;
use aoc_2024::q4::count_total_xmas;

fn main() {
    let input = read_to_string("data/q4.txt").expect("Error reading file");
    let result = count_total_xmas(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_total_xmas() {
        let input = read_to_string("data/q4.txt").expect("Error reading file");
        let result = count_total_xmas(&input);
        assert_eq!(result, 2532);
    }
}
