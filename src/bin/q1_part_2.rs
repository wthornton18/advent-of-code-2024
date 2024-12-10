use aoc_2024::file_utils::read_to_string;

fn main() {
    let input = read_to_string("data/q1.txt").expect("Error reading file");
    let result = aoc_2024::q1::compute_total_similarity_score(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_total_similarity_score() {
        let input = read_to_string("data/q1.txt").expect("Error reading file");
        let result = aoc_2024::q1::compute_total_similarity_score(&input);
        assert_eq!(result, 19097157);
    }
}
