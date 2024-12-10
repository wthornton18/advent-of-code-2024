use aoc_2024::file_utils::read_to_string;
use aoc_2024::q7::get_satisfiable_equation_target_all;

fn main() {
    let input = read_to_string("data/q7.txt").expect("Error reading file");
    let result = get_satisfiable_equation_target_all(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_satisfiable_equation_target_all() {
        let input = read_to_string("data/q7.txt").expect("Error reading file");
        let result = get_satisfiable_equation_target_all(&input);
        assert_eq!(result, 177531);
    }
}
