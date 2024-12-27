use hashbrown::HashSet;

use crate::graph::Graph;

fn parse_input(input: &str) -> Graph<&str> {
    let mut graph = Graph::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();

        graph.add_edge(from, to, ());
        graph.add_edge(to, from, ());
    }

    graph
}

pub fn t_predicate(clique: &[&str]) -> bool {
    clique.iter().any(|&node| node.starts_with("t"))
}

pub fn count_triangle_cliques_where<P>(input: &str, predicate: P) -> usize
where
    P: Fn(&[&str]) -> bool,
{
    let graph = parse_input(input);

    graph
        .k_cliques(3)
        .iter()
        .filter(|clique| predicate(clique.as_slice()))
        .count()
}

pub fn get_largest_clique(input: &str) -> HashSet<&str> {
    let graph = parse_input(input);

    graph.bron_kerbosh()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_count_triangle_cliques_where() {
        let result = count_triangle_cliques_where(TEST_INPUT, t_predicate);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_get_largest_clique() {
        let result = get_largest_clique(TEST_INPUT);
        assert_eq!(result.len(), 4);
    }
}
