use aoc_2024::file_utils::read_to_string;
use aoc_2024::q19::count_possible_constructable_towels;

fn main() {
    let input = read_to_string("data/q19.txt").expect("Error reading file");
    let result = count_possible_constructable_towels(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_possible_constructable_towels() {
        let input = read_to_string("data/q19.txt").expect("Error reading file");
        let result = count_possible_constructable_towels(&input);
        assert_eq!(result, 240);
    }
}
