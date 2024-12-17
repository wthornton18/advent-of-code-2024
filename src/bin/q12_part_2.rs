use aoc_2024::file_utils::read_to_string;
use aoc_2024::q12::get_total_garden_discounted_price;

fn main() {
    let input = read_to_string("data/q12.txt").unwrap();
    let total_garden_price = get_total_garden_discounted_price(&input);
    println!("Result: {}", total_garden_price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_garden_discounted_price() {
        let input = read_to_string("data/q12.txt").unwrap();
        let result = get_total_garden_discounted_price(&input);
        assert_eq!(result, 910066);
    }
}
