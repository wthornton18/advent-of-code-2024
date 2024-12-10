use aoc_2024::file_utils::read_to_string;
use aoc_2024::q3::compute_multiplication_sum;

fn main() {
    let input = read_to_string("data/q3.txt").expect("Error reading file");
    let result = compute_multiplication_sum(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_multiplication_sum() {
        let input = read_to_string("data/q3.txt").expect("Error reading file");
        let result = compute_multiplication_sum(&input);
        assert_eq!(result, 173731097);
    }
}
