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

    pub(crate) fn get_choices(&self, dim: usize) -> Vec<Vec<usize>> {
        let successors = (0..dim).map(|i| self.get_successors(i)).collect::<Vec<_>>();
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
