use aoc_2024::file_utils::read_to_string;
use aoc_2024::q18::get_bytes_shortest_path_length;

fn main() {
    let input = read_to_string("data/q18.txt").expect("Failed to read input");
    let result = get_bytes_shortest_path_length(&input, 1024, (71, 71), (0, 0), (70, 70));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bytes_shortest_path_length() {
        let input = read_to_string("data/q18.txt").expect("Failed to read input");
        let result = get_bytes_shortest_path_length(&input, 1024, (71, 71), (0, 0), (70, 70));
        assert_eq!(result, 356);
    }
}
