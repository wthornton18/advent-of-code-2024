use aoc_2024::q15::calculate_final_gps_sum;

fn main() {
    let input = include_str!("../../data/q15.txt");
    let result = calculate_final_gps_sum(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_final_gps_sum() {
        let input = include_str!("../../data/q15.txt");
        let result = calculate_final_gps_sum(input);
        assert_eq!(result, 1568399);
    }
}
