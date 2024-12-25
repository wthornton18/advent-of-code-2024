use aoc_2024::q24::compute_z_number;

fn main() {
    let input = include_str!("../../data/q24.txt");
    let result = compute_z_number(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_z_number() {
        let input = include_str!("../../data/q24.txt");
        let result = compute_z_number(input);
        assert_eq!(result, 507);
    }
}
