use aoc_2024::file_utils::read_to_string;
use aoc_2024::q11::count_total_stones;

fn main() {
    let input = read_to_string("data/q11.txt").expect("Failed to read file");
    let result = count_total_stones(&input, 75);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_total_stones() {
        let input = read_to_string("data/q11.txt").expect("Failed to read file");
        assert_eq!(count_total_stones(&input, 75), 224577979481346)
    }
}
