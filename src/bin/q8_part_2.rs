use aoc_2024::file_utils::read_to_string;
use aoc_2024::q8::count_unique_antinodes;

fn main() {
    let input = read_to_string("data/q8.txt").expect("Error reading file");
    let result = count_unique_antinodes(&input, true);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_antinodes() {
        let input = read_to_string("data/q8.txt").expect("Error reading file");
        let result = count_unique_antinodes(&input, true);
        assert_eq!(result, 861);
    }
}
