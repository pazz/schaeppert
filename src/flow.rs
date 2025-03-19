use crate::coef::{Coef, C0, OMEGA};
use crate::graph::Graph;
use crate::sheep;
use std::fmt;
use std::{collections::HashSet, vec::Vec};

pub type Domain = Vec<Coef>;
pub type Image = Vec<Coef>;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Flow {
    pub dim: usize,
    //size is dim * dim
    entries: Vec<Coef>,
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
    fn _dom(dim: usize, entries: &[Coef], roundup: bool) -> Domain {
        if entries.len() != dim * dim {
            panic!("Invalid number of entries");
        }
        let mut result = vec![C0; dim];
        if dim == 0 {
            return result;
        }
        for i in 0..dim {
            let line = &entries[i * dim..(i + 1) * dim];
            if line.iter().any(|x| *x == OMEGA) {
                result[i] = OMEGA;
            } else {
                let sum = line.iter().sum();
                result[i] = match sum {
                    Coef::Omega => OMEGA,
                    Coef::Value(x) => {
                        if roundup && x > dim as u16 {
                            OMEGA
                        } else {
                            Coef::Value(x)
                        }
                    }
                }
            }
        }
        result
    }

    fn _im(dim: usize, entries: &[Coef]) -> Image {
        if entries.len() != dim * dim {
            panic!("Invalid number of entries");
        }
        let mut result = vec![C0; dim];
        if dim == 0 {
            return result;
        }
        for j in 0..dim {
            let column: Vec<Coef> = (0..dim).map(|i| entries[i + j * dim]).collect();
            if column.iter().any(|x| *x == OMEGA) {
                result[j] = OMEGA;
            } else {
                result[j] = column.iter().sum();
            }
        }
        result
    }

    fn _product(entries: &[Coef], other_entries: &[Coef], dim: usize) -> Vec<Coef> {
        let mut result = vec![C0; dim * dim];
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

    pub fn _iteration(entries: &[Coef], dim: usize) -> Flow {
        let mut result: Vec<Coef> = entries.into();
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
        let lines = Self::_get_lines_vec(domain, edges);
        Self::_cartesian_product(&lines)
            .iter()
            .map(|x| Flow {
                dim,
                entries: x.iter().flat_map(|x| x.iter()).cloned().collect(),
            })
            .collect()
    }

    //takes a vector of vectors of a generic type and computes its cartesain product
    fn _cartesian_product<T: Clone + Eq + std::hash::Hash>(vectors: &[Vec<T>]) -> HashSet<Vec<T>> {
        match vectors.len() {
            0 => HashSet::new(),
            1 => vectors[0].iter().map(|x| vec![x.clone()]).collect(),
            _ => {
                let mut result = HashSet::new();
                for x in &vectors[0] {
                    for mut y in Self::_cartesian_product(&vectors[1..]) {
                        y.insert(0, x.clone());
                        result.insert(y);
                    }
                }
                result
            }
        }
    }

    fn _get_lines_vec(domain: &sheep::Sheep, edges: &Graph) -> Vec<Vec<Domain>> {
        let dim = domain.len();
        (0..dim)
            .map(|i| {
                let out = edges.get_successors(i);
                let coef = domain.get(i);
                Self::_get_lines(out, coef, dim)
            })
            .collect()
    }

    //todo cache results
    fn _get_lines(out: Vec<usize>, coef: Coef, dim: usize) -> Vec<Domain> {
        match coef {
            C0 => vec![vec![C0; dim]],
            OMEGA => vec![(0..dim)
                .map(|i| if out.contains(&i) { OMEGA } else { C0 })
                .collect()],
            Coef::Value(x) => Self::_get_partitions(x, out.len())
                .iter()
                .map(|p| {
                    let mut result = vec![C0; dim];
                    for (i, j) in out.iter().zip(p.iter()) {
                        result[*i] = Coef::Value(*j);
                    }
                    result
                })
                .collect(),
        }
    }

    /* get all partitions of non-negative integers of length len and sum equal to x.
    E.g. if len = 3 and x = 4 returns [[4,0,0], [3,1,0], [3,0,1], [2,2,0], [2,1,1], ...., [0,0,4]]
     */
    fn _get_partitions(x: u16, len: usize) -> Vec<Vec<u16>> {
        let mut result: Vec<Vec<u16>> = Vec::new();
        if len > 0 {
            let mut current = vec![0; len];
            current[0] = x;
            Self::_get_partitions_rec(0, &mut current, &mut result);
        }
        result
    }

    fn _get_partitions_rec(start_index: usize, current: &mut Vec<u16>, result: &mut Vec<Vec<u16>>) {
        result.push(current.clone());
        if start_index + 1 >= current.len() {
            return;
        }
        while current[start_index] > 0 {
            current[start_index] -= 1;
            current[start_index + 1] = current.iter().skip(start_index + 1).sum::<u16>() + 1;
            (start_index + 2..current.len()).for_each(|i| {
                current[i] = 0;
            });
            Self::_get_partitions_rec(start_index + 1, current, result);
        }
    }
}

impl fmt::Display for Flow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.dim {
            let sheep =
                sheep::Sheep::from_vec(self.entries[i * self.dim..(i + 1) * self.dim].to_vec());
            result.push_str(&sheep.to_string());
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::coef::{C0, C1, C2, C3};

    #[test]
    fn from_domain_and_edges() {
        let domain = sheep::Sheep::from_vec(vec![C1, C2, C3]);
        let edges = Graph::from_vec([(0, 1), (1, 2)].to_vec());
        let flow = Flow::from_domain_and_edges(&domain, &edges);
        assert!(flow.is_empty());
        //todo
    }

    #[test]
    #[should_panic]
    fn from_domain_and_edges_panic_case() {
        let domain = sheep::Sheep::from_vec(vec![C1, C2, C3]);
        let edges = Graph::from_vec(vec![(0, 1), (1, 3)]);
        Flow::from_domain_and_edges(&domain, &edges);
    }

    //test _get_partitions_rec on an example with start_index=0 current= [3,0,0] and result empty
    #[test]
    fn get_partitions_rec_test() {
        let x = 3;
        let expected = vec![
            vec![3, 0, 0],
            vec![2, 1, 0],
            vec![2, 0, 1],
            vec![1, 2, 0],
            vec![1, 1, 1],
            vec![1, 0, 2],
            vec![0, 3, 0],
            vec![0, 2, 1],
            vec![0, 1, 2],
            vec![0, 0, 3],
        ];
        assert_eq!(Flow::_get_partitions(x, 3), expected);
    }

    #[test]
    fn get_lines_test() {
        let out = vec![0, 1];
        let dim = 3;
        assert_eq!(
            Flow::_get_lines(out, C3, dim),
            vec![
                vec![C3, C0, C0],
                vec![C2, C1, C0],
                vec![C1, C2, C0],
                vec![C0, C3, C0],
            ]
        );
    }

    #[test]
    fn get_lines_omega_test() {
        let out = vec![0, 1];
        let coef = Coef::Omega;
        let dim = 3;
        let expected = vec![vec![OMEGA, OMEGA, C0]];
        assert_eq!(Flow::_get_lines(out, coef, dim), expected);
    }

    //test _get_lines_vec on an example with domain=[1,3,omega] and edges=[(0,1),(1,0),(1,1),(2,1),(2,2)]
    #[test]
    fn get_lines_vec_test() {
        let domain = sheep::Sheep::from_vec(vec![C1, C3, OMEGA]);
        let edges = Graph::from_vec(vec![(0, 1), (1, 0), (1, 1), (2, 1), (2, 2)]);
        let expected = vec![
            vec![vec![C0, C1, C0]],
            vec![
                vec![C3, C0, C0],
                vec![C2, C1, C0],
                vec![C1, C2, C0],
                vec![C0, C3, C0],
            ],
            vec![vec![C0, OMEGA, OMEGA]],
        ];
        let computed = Flow::_get_lines_vec(&domain, &edges);
        //check computed and expected are equal, up to order of elements
        assert_eq!(computed.len(), expected.len());
        assert_eq!(computed[0], expected[0]);
        assert_eq!(computed[2], expected[2]);
        assert_eq!(computed[1].len(), expected[1].len());
        for x in &computed[1] {
            assert!(expected[1].contains(&x));
        }
    }

    //tests from_domain_and_edges on an example with domain=[1,3,omega] and edges=[(0,1),(1,0),(1,1),(2,1),(2,2)]
    #[test]
    fn from_domain_and_edges_test() {
        let domain = sheep::Sheep::from_vec(vec![C1, C3, OMEGA]);
        let edges = Graph::from_vec(vec![(0, 1), (1, 0), (1, 1), (2, 1), (2, 2)]);
        let flows = Flow::from_domain_and_edges(&domain, &edges);
        let expected = vec![
            Flow {
                dim: 3,
                entries: vec![C0, C1, C0, C0, C3, C0, C0, OMEGA, OMEGA],
            },
            Flow {
                dim: 3,
                entries: vec![C0, C1, C0, C1, C2, C0, C0, OMEGA, OMEGA],
            },
            Flow {
                dim: 3,
                entries: vec![C0, C1, C0, C2, C1, C0, C0, OMEGA, OMEGA],
            },
            Flow {
                dim: 3,
                entries: vec![C0, C1, C0, C3, C0, C0, C0, OMEGA, OMEGA],
            },
        ];
        assert_eq!(flows, expected.into_iter().collect());
    }
}
