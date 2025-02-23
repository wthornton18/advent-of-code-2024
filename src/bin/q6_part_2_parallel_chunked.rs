use aoc_2024::file_utils::read_to_string;
use aoc_2024::q6::get_total_number_of_cycles_parallel_chunked;

fn main() {
    let input = read_to_string("data/q6.txt").expect("Error reading file");
    let result = get_total_number_of_cycles_parallel_chunked(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_number_of_cycles() {
        let input = read_to_string("data/q6.txt").expect("Error reading file");
        let result = get_total_number_of_cycles_parallel_chunked(&input);
        assert_eq!(result, 1697);
    }
}
