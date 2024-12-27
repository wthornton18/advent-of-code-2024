use aoc_2024::q23::{count_triangle_cliques_where, t_predicate};

fn main() {
    let input = include_str!("../../data/q23.txt");
    let result = count_triangle_cliques_where(input, t_predicate);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_triangle_cliques_where() {
        let input = include_str!("../../data/q23.txt");
        let result = count_triangle_cliques_where(input, t_predicate);
        assert_eq!(result, 1154);
    }
}
