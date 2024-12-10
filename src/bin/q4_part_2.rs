use aoc_2024::file_utils::read_to_string;
use aoc_2024::q4::count_total_x_mas;

fn main() {
    let input = read_to_string("data/q4.txt").expect("Error reading file");
    let result = count_total_x_mas(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_total_x_mas() {
        let input = read_to_string("data/q4.txt").expect("Error reading file");
        let result = count_total_x_mas(&input);
        assert_eq!(result, 1941);
    }
}
