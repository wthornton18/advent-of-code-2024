use aoc_2024::file_utils::read_to_string;
use aoc_2024::q17::find_a_for_quine_sequence;

fn main() {
    let input = read_to_string("data/q17.txt").expect("Failed to read input");
    let result = find_a_for_quine_sequence(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_a_for_quine_sequence() {
        let input = read_to_string("data/q17.txt").expect("Failed to read input");
        let result = find_a_for_quine_sequence(&input);
        assert_eq!(result, 164278496489149);
    }
}
