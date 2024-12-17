use aoc_2024::file_utils::read_to_string;
use aoc_2024::q16::get_shortest_path_cost;

fn main() {
    let input = read_to_string("data/q16.txt").expect("Failed to read input");
    let result = get_shortest_path_cost(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shortest_path_cost() {
        let input = read_to_string("data/q16.txt").expect("Failed to read input");
        let result = get_shortest_path_cost(&input);
        assert_eq!(result, 66404.0);
    }
}
