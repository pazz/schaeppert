use crate::partitions;
use std::{collections::HashSet, fmt};

//Eq and Partial Eq
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Graph(HashSet<(usize, usize)>);

impl Graph {
    pub fn new(edges: &[(usize, usize)]) -> Self {
        Graph(edges.iter().cloned().collect())
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.0.iter()
    }

    #[allow(dead_code)]
    pub(crate) fn from_vec(vec: Vec<(usize, usize)>) -> Graph {
        Graph(vec.into_iter().collect())
    }

    pub(crate) fn get_successors(&self, i: usize) -> Vec<usize> {
        self.0
            .iter()
            .filter_map(|&(i0, j0)| (i == i0).then_some(j0))
            .collect()
    }

    pub(crate) fn get_choices(&self, dim: usize) -> Vec<Vec<Option<usize>>> {
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
        let choices = graph.get_choices(3);
        assert_eq!(choices.len(), 2);
        assert!(choices.contains(&vec![Some(1), Some(2)]));
        assert!(choices.contains(&vec![Some(2), Some(1)]));
    }
}
