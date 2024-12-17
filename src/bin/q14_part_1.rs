use aoc_2024::file_utils::read_to_string;
use aoc_2024::q14::get_safety_factor;

fn main() {
    let input = read_to_string("data/q14.txt").expect("Failed to read file");
    let safety_factor = get_safety_factor(&input, 100, (101, 103));
    println!("Result: {}", safety_factor);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_safety_factor() {
        let input = read_to_string("data/q14.txt").expect("Failed to read file");
        let result = get_safety_factor(&input, 100, (101, 103));
        assert_eq!(result, 214109808);
    }
}
