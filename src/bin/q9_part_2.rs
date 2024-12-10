use aoc_2024::file_utils::read_to_string;
use aoc_2024::q9::get_compact_no_frag_checksum;

fn main() {
    let input = read_to_string("data/q9.txt").expect("Error reading file");
    let result = get_compact_no_frag_checksum(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_compact_no_frag_checksum() {
        let input = read_to_string("data/q9.txt").expect("Error reading file");
        let result = get_compact_no_frag_checksum(&input);
        assert_eq!(result, 6304576012713);
    }
}
