use crate::graph::Graph;

fn parse_input(input: &str) -> Graph<&str> {
    let mut graph = Graph::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (u, v) = line.split_once("-").expect("Failed to split line");
        graph.add_edge(u, v, ());
    }

    graph
}

pub fn count_connected_subgraphs_where<F>(input: &str, predicate: F) -> usize
where
    F: Fn(&str) -> bool,
{
    let graph = parse_input(input);
    graph
        .connected_subgraphs()
        .iter()
        .filter(|g| {
            print!("{:?}", g.vertices);

            g.vertices.iter().any(|v| predicate(v)) && g.vertices.len() == 3
        })
        .count()
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
td-yn
";
}
