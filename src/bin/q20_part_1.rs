use aoc_2024::q20::get_total_number_of_cheats;

fn main() {
    let input = include_str!("../../data/q20.txt");
    let result = get_total_number_of_cheats(input, 100, 2);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_number_of_cheats() {
        let input = include_str!("../../data/q20.txt");
        let result = get_total_number_of_cheats(input, 100, 2);
        assert_eq!(result, 1426);
    }
}
