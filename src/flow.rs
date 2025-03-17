use crate::graph::Graph;
use crate::sheep;
use crate::sheep::OMEGA;
use std::fmt;
use std::{collections::HashSet, vec::Vec};

pub type Domain = Vec<usize>;
pub type Image = Vec<usize>;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Flow {
    pub dim: usize,
    //size is dim * dim
    entries: Vec<usize>,
}

pub trait FlowTrait {
    fn dom(&self, roundup: bool) -> Domain;
    fn im(&self) -> Image;
    fn product(&self, other: &Flow) -> Flow;
    fn iteration(&self) -> HashSet<Flow>;
}

impl FlowTrait for Flow {
    fn dom(&self, roundup: bool) -> Domain {
        Self::_dom(self.dim, &self.entries, roundup)
    }

    fn im(&self) -> Image {
        Self::_im(self.dim, &self.entries)
    }

    //product of two flows
    fn product(&self, other: &Flow) -> Flow {
        let dim = self.dim;
        let entries = Self::_product(&self.entries, &other.entries, dim);
        Flow { dim, entries }
    }

    fn iteration(&self) -> HashSet<Flow> {
        //!todo!("generates all possible sharp results");
        HashSet::from([Self::_iteration(&self.entries, self.dim)])
    }
}

impl Flow {
    fn _dom(dim: usize, entries: &Vec<usize>, roundup: bool) -> Domain {
        if entries.len() != dim * dim {
            panic!("Invalid number of entries");
        }
        let mut result = vec![0; dim];
        if dim == 0 {
            return result;
        }
        for i in 0..dim {
            let line = &entries[i * dim..(i + 1) * dim];
            if line.iter().any(|x| *x == OMEGA) {
                result[i] = OMEGA;
            } else {
                let sum: usize = line.iter().sum();
                if roundup && sum > dim {
                    result[i] = OMEGA;
                } else {
                    result[i] = sum;
                }
            }
        }
        result
    }

    fn _im(dim: usize, entries: &Vec<usize>) -> Image {
        if entries.len() != dim * dim {
            panic!("Invalid number of entries");
        }
        let mut result = vec![0; dim];
        if dim == 0 {
            return result;
        }
        for j in 0..dim {
            let column: Vec<usize> = (0..dim).map(|i| entries[i + j * dim]).collect();
            if column.iter().any(|x| *x == OMEGA) {
                result[j] = OMEGA;
            } else {
                result[j] = column.iter().sum();
            }
        }
        result
    }

    fn _product(entries: &Vec<usize>, other_entries: &Vec<usize>, dim: usize) -> Vec<usize> {
        let mut result = vec![0; dim * dim];
        for i in 0..dim {
            for j in 0..dim {
                result[i * dim + j] = (0..dim)
                    .map(|k| std::cmp::min(entries[i * dim + k], other_entries[k * dim + j]))
                    .max()
                    .unwrap();
            }
        }
        result
    }

    //iteration of a flow

    pub fn _iteration(entries: &Vec<usize>, dim: usize) -> Flow {
        let mut result = entries.clone();
        loop {
            let result_squared = Self::_product(&result, &result, dim);
            if result == result_squared {
                return Flow {
                    dim,
                    entries: result,
                };
            }
            result = result_squared;
        }
    }

    //compute all possible flows compatible with this domain and edges
    //there might be choice from small constants: a 5 distributed on 3 edges might lead to (5 + 0 + 0) or (1 + 1+ 3)
    //this is annoyingly exponential and should be stored in the future as a compact representation,
    //compatible with product and start in the monoid
    //but for now we will just compute it exhasutively
    /*
    WIP
    */
    pub(crate) fn from_domain_and_edges(domain: &sheep::Sheep, edges: &Graph) -> HashSet<Flow> {
        println!("Creating flow from domain and edges");
        println!("domain {}", domain);
        println!("edges {}", edges);

        let dim = domain.len();
        if edges.iter().any(|f| f.0 >= dim || f.1 >= dim) {
            panic!("Edge out of domain");
        }
        let mut entries = vec![0; dim * dim];
        for i in 0..dim {
            let out = edges.get_successors(i);
            let val = domain.get(i);
            match val {
                0 => {}
                OMEGA => {
                    for j in out {
                        //todo take 1 out of omega in some direction
                        entries[i * dim + j] = OMEGA;
                    }
                }
                x => {
                    //compute all possible distribution of x over out.len() edges
                    //this is a bit annoyingly exponential
                    for j in out {
                        entries[i * dim + j] = x; //wrong todo
                    }
                }
            }
        }
        for (i, j) in edges.iter() {
            entries[i * dim + j] = domain.get(*i);
        }
        let result = Flow { dim, entries };
        println!("flow\n{}", result);
        HashSet::new() //todo
    }
}

impl fmt::Display for Flow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.dim {
            let sheep =
                sheep::Sheep::from_vec(self.entries[i * self.dim..(i + 1) * self.dim].to_vec());
            result.push_str(sheep.to_string().as_str());
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_domain_and_edges() {
        let domain = sheep::Sheep::from_vec(vec![1, 2, 3]);
        let edges = Graph::from_vec([(0, 1), (1, 2)].to_vec());
        let flow = Flow::from_domain_and_edges(&domain, &edges);
        assert!(flow.len() == 0);
        //todo
    }

    #[test]
    #[should_panic]
    fn from_domain_and_edges_panic_case() {
        let domain = sheep::Sheep::from_vec(vec![1, 2, 3]);
        let edges = Graph::from_vec(vec![(0, 1), (1, 3)]);

        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));

        Flow::from_domain_and_edges(&domain, &edges);

        std::panic::set_hook(default_hook);
    }
}
