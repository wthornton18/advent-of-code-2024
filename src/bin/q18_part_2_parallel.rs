use aoc_2024::file_utils::read_to_string;
use aoc_2024::q18::find_minimum_bytes_to_fall_parallel;

fn main() {
    let input = read_to_string("data/q18.txt").expect("Failed to read input");
    let result = find_minimum_bytes_to_fall_parallel(&input, (71, 71), (0, 0), (70, 70));
    println!("{:?}", result.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_minimum_bytes_to_fall_parallel() {
        let input = read_to_string("data/q18.txt").expect("Failed to read input");
        let result = find_minimum_bytes_to_fall_parallel(&input, (71, 71), (0, 0), (70, 70));
        assert_eq!(result, Some((22, 33)));
    }
}
