use std::{collections::BinaryHeap, hash::Hash};

use hashbrown::HashMap;

pub trait AStarSearch {
    type Node: Clone + Eq + Hash;

    fn weighted_neighbours(&self, node: &Self::Node) -> Option<Vec<(Self::Node, f64)>>;

    fn reconstruct_path(
        &self,
        came_from: HashMap<Self::Node, Self::Node>,
        goal: Self::Node,
    ) -> Vec<Self::Node> {
        let mut current = goal;
        let mut path = vec![current.clone()];

        while let Some(next) = came_from.get(&current) {
            current = next.clone();
            path.push(current.clone());
        }

        path.reverse();
        path
    }

    fn a_star_search(
        &self,
        start: Self::Node,
        goal: Self::Node,
        heuristic: fn(Self::Node, Self::Node) -> f64,
    ) -> Option<Vec<Self::Node>> {
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
        g_score.insert(start.clone(), 0.0);
        f_score.insert(start.clone(), heuristic(start.clone(), goal.clone()));

        let mut open_set: BinaryHeap<Elem<Self::Node>> = BinaryHeap::new();
        open_set.push(Elem(start.clone(), f_score[&start]));

        while let Some(Elem(v, _)) = open_set.pop() {
            if v == goal {
                return Some(self.reconstruct_path(came_from, goal));
            }

            if let Some(weighted_neighbours) = self.weighted_neighbours(&v) {
                for (u, w) in weighted_neighbours {
                    let tenative_g_score = g_score[&v] + w;
                    if tenative_g_score < *g_score.get(&u).unwrap_or(&f64::INFINITY) {
                        came_from.insert(u.clone(), v.clone());
                        g_score.insert(u.clone(), tenative_g_score);
                        f_score.insert(
                            u.clone(),
                            tenative_g_score + heuristic(u.clone(), goal.clone()),
                        );
                        open_set.push(Elem(u.clone(), f_score[&u]));
                    }
                }
            }
        }

        None
    }
}
