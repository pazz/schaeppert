use crate::partitions;
use std::{collections::HashSet, fmt};

/// A directed graph is a set of edges.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Graph(HashSet<(usize, usize)>);

impl Graph {
    /// Create a new graph from a list of edges.
    pub fn new(edges: &[(usize, usize)]) -> Self {
        Graph(edges.iter().cloned().collect())
    }

    /// Create a new graph from a list of edges.
    #[allow(dead_code)]
    pub(crate) fn from_vec(vec: Vec<(usize, usize)>) -> Graph {
        Graph(vec.into_iter().collect())
    }

    /// Return an iterator over the edges of the graph.
    pub(crate) fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.0.iter()
    }

    /// Return the successors of a node.
    pub(crate) fn get_successors(&self, i: usize) -> Vec<usize> {
        self.0
            .iter()
            .filter_map(|&(i0, j0)| (i == i0).then_some(j0))
            .collect()
    }

    /// Returns the maximal deterministic subgraphs of the graph.
    /// This is a selection of edges such that every vertex has a single outgoing edge,
    /// or no outgoing edge at all if this is already the case in the original graph.
    pub(crate) fn get_maximal_deterministic_subgraphs(
        &self,
        dim: usize,
    ) -> Vec<Vec<Option<usize>>> {
        let successors = (0..dim)
            .map(|i| {
                let succ = self.get_successors(i);
                if succ.is_empty() {
                    vec![None]
                } else {
                    succ.iter().map(|&x| Some(x)).collect()
                }
            })
            .collect::<Vec<_>>();
        partitions::cartesian_product(&successors)
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec: Vec<String> = self.0.iter().map(|x| format!("{:?}", x)).collect();
        vec.sort();
        write!(f, "\n\t{}", vec.join("\n\t"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_choices() {
        let edges = vec![(0, 1), (0, 2)];
        let graph = Graph::new(&edges);
        let choices = graph.get_maximal_deterministic_subgraphs(3);
        assert_eq!(choices.len(), 2);
        assert!(choices.contains(&vec![Some(1), None, None]));
        assert!(choices.contains(&vec![Some(2), None, None]));
    }
}
