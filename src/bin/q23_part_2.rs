use aoc_2024::q23::get_largest_clique;

fn main() {
    let input = include_str!("../../data/q23.txt");
    let result = get_largest_clique(input);
    let mut result = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    result.sort();
    println!("{}", result.join::<&str>(","))
}

#[cfg(test)]
mod tests {
    use hashbrown::HashSet;

    use super::*;

    #[test]
    fn test_get_largest_clique() {
        let input = include_str!("../../data/q23.txt");
        let result = get_largest_clique(input);
        let expected_clique: HashSet<&str, _> = HashSet::from([
            "aj", "ds", "gg", "id", "im", "jx", "kq", "nj", "ql", "qr", "ua", "yh", "zn",
        ]);

        assert_eq!(result, expected_clique);
    }
}
