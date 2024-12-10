use aoc_2024::file_utils::read_to_string;
use aoc_2024::q9::get_maximally_compact_checksum;

fn main() {
    let input = read_to_string("data/q9.txt").expect("Error reading file");
    let result = get_maximally_compact_checksum(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximally_compact_checksum() {
        let input = read_to_string("data/q9.txt").expect("Error reading file");
        let result = get_maximally_compact_checksum(&input);
        assert_eq!(result, 6283404590840);
    }
}
