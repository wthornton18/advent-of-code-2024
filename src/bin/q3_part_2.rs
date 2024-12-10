use aoc_2024::file_utils::read_to_string;
use aoc_2024::q3::compute_multiplication_sum_op_aware;

fn main() {
    let input = read_to_string("data/q3.txt").expect("Error reading file");
    let result = compute_multiplication_sum_op_aware(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_multiplication_sum_op_aware() {
        let input = read_to_string("data/q3.txt").expect("Error reading file");
        let result = compute_multiplication_sum_op_aware(&input);
        assert_eq!(result, 93729253);
    }
}
