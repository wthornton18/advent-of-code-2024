use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::ops::Index;
use std::{fmt::Display, hash::Hash};

use hashbrown::{HashMap, HashSet};

pub trait Weight {
    fn weight(&self) -> f64;
}

#[derive(Debug, Clone)]
pub struct Graph<K, W> {
    pub vertices: HashSet<K>,
    pub edges: HashMap<K, Vec<(K, W)>>,
}

impl<K, W> Graph<K, W> {
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
        }
    }
}

impl<K, W> Graph<K, W> {
    pub fn with_capacity(vertices: usize) -> Self {
        Self {
            vertices: HashSet::with_capacity(vertices),
            edges: HashMap::with_capacity(vertices),
        }
    }
}

impl<K, W> Graph<K, W>
where
    K: Eq + Hash + Clone,
{
    pub fn add_edge(&mut self, from: K, to: K, weight: W) {
        self.vertices.insert(from.clone());
        self.vertices.insert(to.clone());
        self.edges.entry(from).or_default().push((to, weight));
    }

    #[allow(dead_code)]
    pub fn contains(&self, vertex: &K) -> bool {
        self.vertices.contains(vertex)
    }

    pub fn remove_vertex(&mut self, vertex: K) {
        self.vertices.remove(&vertex);
        self.edges.remove(&vertex);
        for (_, edges) in self.edges.iter_mut() {
            edges.retain(|(v, _)| v != &vertex);
        }
    }
}

impl<K, W> Index<K> for Graph<K, W>
where
    K: Eq + Hash + Clone,
{
    type Output = Vec<(K, W)>;

    fn index(&self, index: K) -> &Self::Output {
        &self.edges[&index]
    }
}

impl<K, W> Graph<K, W> {
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl<K, W> std::fmt::Display for Graph<K, W>
where
    K: std::fmt::Display,
    W: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (vertex, edges) in &self.edges {
            write!(f, "{} -> ", vertex)?;
            for (to, weight) in edges {
                write!(f, "({} -> {}), ", to, weight)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Weight for f64 {
    fn weight(&self) -> f64 {
        *self
    }
}

impl Weight for i32 {
    fn weight(&self) -> f64 {
        *self as f64
    }
}
impl Weight for u32 {
    fn weight(&self) -> f64 {
        *self as f64
    }
}

impl Weight for u64 {
    fn weight(&self) -> f64 {
        *self as f64
    }
}

impl Weight for usize {
    fn weight(&self) -> f64 {
        *self as f64
    }
}

impl Weight for f32 {
    fn weight(&self) -> f64 {
        *self as f64
    }
}

impl Weight for i64 {
    fn weight(&self) -> f64 {
        *self as f64
    }
}

impl<K, W> Graph<K, W>
where
    K: Eq + Hash + Clone,
    W: Weight,
{
    pub fn dijkstra(&self, source: K) -> (HashMap<K, f64>, Graph<K, f64>) {
        let mut dist = HashMap::with_capacity(self.len());
        let mut prev = Graph::with_capacity(self.len()); // The prev graph will always be shorter than the original graph, so we can use the same capacity

        let mut queue = BinaryHeap::new();

        pub struct Elem<K> {
            vertex: K,
            priority: f64,
        }

        impl<K> PartialEq for Elem<K> {
            fn eq(&self, other: &Self) -> bool {
                self.priority == other.priority
            }
        }

        impl<K> Eq for Elem<K> {}

        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl<K> PartialOrd for Elem<K> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.priority.partial_cmp(&other.priority)
            }
        }

        impl<K> Ord for Elem<K> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.priority.partial_cmp(&other.priority).unwrap()
            }
        }

        for vertex in &self.vertices {
            dist.insert(vertex.clone(), f64::INFINITY);
            let priority = f64::INFINITY;
            queue.push(Reverse(Elem {
                vertex: vertex.clone(),
                priority,
            }));
        }

        queue.push(Reverse(Elem {
            vertex: source.clone(),
            priority: 0.0,
        }));

        dist.insert(source.clone(), 0.0);

        while let Some(Reverse(Elem { vertex: u, .. })) = queue.pop() {
            if let Some(edges) = self.edges.get(&u) {
                for (v, w) in edges {
                    let alt = dist[&u] + w.weight();
                    if alt <= dist[v] {
                        dist.insert(v.clone(), alt);
                        prev.add_edge(v.clone(), u.clone(), alt);
                        queue.retain(|Reverse(Elem { vertex, .. })| vertex != v);
                        queue.push(Reverse(Elem {
                            vertex: v.clone(),
                            priority: alt,
                        }));
                    }
                }
            }
        }

        (dist, prev)
    }
}

impl<K> Graph<K, f64>
where
    K: Clone + Eq + Hash + Display + Debug,
{
    // This assumes that the graph is a DAG and that the source and target vertices are in the graph
    // And that the graph is strongly connected, such as the prev graph returned by dijkstra
    pub fn shortest_paths_subgraph(self, source: K, target: K) -> Graph<K, f64> {
        let mut graph = Graph::new();

        let mut queue = vec![source.clone()];

        while let Some(e) = queue.pop() {
            if e == target {
                continue;
            }

            let mut minimum_weight = f64::INFINITY;
            let mut minium_weight_verticies = HashSet::new();

            if let Some(edges) = self.edges.get(&e) {
                for (v, w) in edges.iter() {
                    if w < &minimum_weight {
                        minimum_weight = *w;
                        minium_weight_verticies = HashSet::new();
                    } else if w == &minimum_weight {
                        minium_weight_verticies.insert(v.clone());
                    }
                }
            }

            for v in minium_weight_verticies {
                graph.add_edge(e.clone(), v.clone(), minimum_weight);
                queue.push(v);
            }
        }

        graph
    }
}

impl<K, W> Graph<K, W>
where
    K: Eq + Hash + Clone + Debug,
    W: Weight + Debug,
{
    fn reconstruct_path(&self, came_from: HashMap<K, K>, goal: K) -> Vec<K> {
        let mut current = goal;
        let mut path = vec![current.clone()];

        while let Some(next) = came_from.get(&current) {
            current = next.clone();
            path.push(current.clone());
        }

        path.reverse();
        path
    }

    pub fn a_star_search<H>(&self, source: K, target: K, heuristic: H) -> Vec<K>
    where
        H: Fn(K, K) -> f64,
    {
        struct Elem<K>(K, f64);
        impl<K> PartialEq for Elem<K> {
            fn eq(&self, other: &Self) -> bool {
                self.1 == other.1
            }
        }
        impl<K> Eq for Elem<K> {}
        impl<K> Ord for Elem<K> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.1.partial_cmp(&other.1).unwrap().reverse()
            }
        }

        impl<K> PartialOrd for Elem<K> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();
        g_score.insert(source.clone(), 0.0);
        f_score.insert(source.clone(), heuristic(source.clone(), target.clone()));

        let mut open_set: BinaryHeap<Elem<K>> = BinaryHeap::new();
        open_set.push(Elem(source.clone(), f_score[&source]));

        while let Some(Elem(v, _)) = open_set.pop() {
            if v == target {
                return self.reconstruct_path(came_from, target);
            }

            if let Some(edges) = self.edges.get(&v) {
                for (u, w) in edges {
                    let tenative_g_score = g_score[&v] + w.weight();
                    if tenative_g_score < *g_score.get(u).unwrap_or(&f64::INFINITY) {
                        came_from.insert(u.clone(), v.clone());
                        g_score.insert(u.clone(), tenative_g_score);
                        f_score.insert(
                            u.clone(),
                            tenative_g_score + heuristic(u.clone(), target.clone()),
                        );
                        open_set.push(Elem(u.clone(), f_score[u]));
                    }
                }
            }
        }

        Vec::new()
    }
}
