use aoc_2024::file_utils::read_to_string;
use aoc_2024::q10::get_total_trailhead_score;

fn main() {
    let input = read_to_string("data/q10.txt").expect("Error reading file");
    let result = get_total_trailhead_score(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_trailhead_score() {
        let input = read_to_string("data/q10.txt").expect("Error reading file");
        let result = get_total_trailhead_score(&input);
        assert_eq!(result, 822);
    }
}
