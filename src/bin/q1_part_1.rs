use aoc_2024::file_utils::read_to_string;
use aoc_2024::q1::compute_total_distance;

fn main() {
    let input = read_to_string("data/q1.txt").expect("Error reading file");
    let result = compute_total_distance(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_total_distance() {
        let input = read_to_string("data/q1.txt").expect("Error reading file");
        let result = compute_total_distance(&input);
        assert_eq!(result, 2113135);
    }
}
