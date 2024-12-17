use aoc_2024::file_utils::read_to_string;
use aoc_2024::q16::get_area_covered_by_shortest_paths;

fn main() {
    let input = read_to_string("data/q16.txt").expect("Failed to read input");
    let result = get_area_covered_by_shortest_paths(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area_covered_by_shortest_paths() {
        let input = read_to_string("data/q16.txt").expect("Failed to read input");
        let result = get_area_covered_by_shortest_paths(&input);
        assert_eq!(result, 433);
    }
}
