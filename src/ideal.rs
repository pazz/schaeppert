use crate::coef::{Coef, OMEGA};
use crate::sheep::Sheep;
use crate::{coef, partitions};
use std::fmt;
use std::{collections::HashSet, vec::Vec};
#[derive(Clone, Eq, Debug)]
pub struct Ideal(HashSet<Sheep>);

impl PartialEq for Ideal {
    fn eq(&self, other: &Self) -> bool {
        //print!("{}\n{}\n", self, other);
        self.is_contained_in(other) && other.is_contained_in(self)
    }
}

impl Ideal {
    pub(crate) fn sheeps(&self) -> impl Iterator<Item = &Sheep> {
        self.0.iter()
    }

    pub(crate) fn from_vec(w: &[Sheep]) -> Self {
        Ideal(w.iter().cloned().collect())
    }

    pub(crate) fn from_vecs(w: &[&[Coef]]) -> Self {
        Ideal(w.iter().map(|&v| Sheep::from_vec(v.to_vec())).collect())
    }

    pub(crate) fn contains(&self, source: &Sheep) -> bool {
        self.0.iter().any(|x| source <= x)
    }

    pub(crate) fn is_contained_in(&self, other: &Ideal) -> bool {
        self.0.iter().all(|x| other.contains(x))
    }

    pub(crate) fn restrict_to(&mut self, other: &Ideal) -> bool {
        let mut changed = false;
        let mut new_sheeps = Ideal::new();
        for sheep in self.0.iter() {
            if other.contains(sheep) {
                new_sheeps.insert(sheep);
            } else {
                for other_sheep in &other.0 {
                    changed |= new_sheeps.insert(&Sheep::intersection(sheep, other_sheep));
                }
            }
        }
        new_sheeps.minimize();
        self.0 = new_sheeps.0;
        changed
    }

    pub(crate) fn pre_image(&self, edges: &crate::graph::Graph) -> Ideal {
        if self.0.is_empty() {
            return Ideal::new();
        }
        let dim = self.dim();

        //compute for every j the maximal finite coef appearing at index j, if exists
        let max_finite_coords = (0..dim)
            .map(|j| {
                self.0
                    .iter()
                    .filter_map(|sheep| match sheep.get(j) {
                        Coef::Omega => None,
                        Coef::Value(c) => Some(c),
                    })
                    .max()
            })
            .collect::<Vec<_>>();

        //compute for every i whether omega is possible at i, which happens iff there exists a sheep in the ideal such that all successors lead to omega
        let is_omega_possible = (0..dim)
            .map(|i| {
                let succ = edges.get_successors(i);
                return self.0.iter().any(|sheep| sheep.all_omega(&succ));
            })
            .collect::<Vec<_>>();

        let possible_coefs = (0..dim)
            .map(|i| {
                match (
                    max_finite_coords.get(i).unwrap(),
                    is_omega_possible.get(i).unwrap(),
                ) {
                    (Some(c), false) => (0..(std::cmp::max(*c, 1) + 1))
                        .map(coef::Coef::Value)
                        .rev()
                        .collect::<Vec<_>>(),
                    (Some(c), true) => (0..(std::cmp::max(*c, 1) + 1))
                        .map(coef::Coef::Value)
                        .chain(std::iter::once(OMEGA))
                        .rev()
                        .collect::<Vec<_>>(),
                    (None, true) => vec![OMEGA],
                    (None, false) => panic!("logically inconsistent case"),
                }
            })
            .collect::<Vec<_>>();
        print!("max_finite_coords: {:?}\n", max_finite_coords);
        print!("is_omega_possible: {:?}\n", is_omega_possible);
        print!("possible_coefs: {:?}\n", possible_coefs);

        let mut result = Ideal::new();
        for candidate in partitions::cartesian_product(&possible_coefs) {
            print!("candidate: {:?}\n", candidate);
            if self.is_safe(&candidate, edges) {
                print!("\t...ok\n");
                result.insert(&Sheep::from_vec(candidate));
            }
        }
        result.minimize();
        print!("result {}\n", result);
        result
    }

    fn new() -> Self {
        Ideal(HashSet::new())
    }

    fn insert(&mut self, sheep: &Sheep) -> bool {
        if self.0.contains(sheep) {
            false
        } else {
            self.0.insert(sheep.clone());
            true
        }
    }

    fn minimize(&mut self) -> bool {
        //remove from self.0 any element strictly smaller than another
        let mut changed = false;
        for sheep in self
            .0
            .iter()
            .filter(|&x| self.0.iter().any(|y| x < y))
            .cloned()
            .collect::<Vec<_>>()
        {
            changed |= self.0.remove(&sheep);
        }
        changed
    }

    fn dim(&self) -> usize {
        match self.0.iter().next() {
            Some(sheep) => sheep.len(),
            None => panic!("Empty ideal has no dimension"),
        }
    }

    fn is_safe(&self, candidate: &[Coef], edges: &crate::graph::Graph) -> bool {
        let dim = candidate.len();
        edges.get_choices(dim).iter().all(|choice| {
            let image = (0..dim)
                .map(|j: usize| {
                    choice
                        .iter()
                        .enumerate()
                        .filter_map(|(i0, &j0)| (j == j0).then(|| candidate.get(i0).unwrap()))
                        .sum()
                })
                .collect();
            self.contains(&Sheep::from_vec(image))
        })
    }
}

impl fmt::Display for Ideal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        vec.sort();
        write!(f, "{{\n\n{}\n\n}}\n", vec.join(" ,\n"))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::coef::{C0, C1, C2, OMEGA};

    #[test]
    fn is_in_ideal() {
        let master_sheep = Sheep::from_vec(vec![OMEGA, OMEGA]);
        let medium_sheep = Sheep::from_vec(vec![C1, C1]);
        let ini_sheep = Sheep::from_vec(vec![C1, C0]);
        let final_sheep = Sheep::from_vec(vec![C0, C1 + C1]);

        let ideal = Ideal([ini_sheep.clone(), final_sheep.clone()].into());
        assert!(ideal.contains(&ini_sheep));
        assert!(ideal.contains(&final_sheep));
        assert!(!ideal.contains(&master_sheep));
        assert!(!ideal.contains(&medium_sheep));

        let ideal2 = Ideal([medium_sheep.clone()].into());
        assert!(ideal2.contains(&ini_sheep));
        assert!(!ideal2.contains(&final_sheep));
        assert!(!ideal2.contains(&master_sheep));
        assert!(ideal2.contains(&medium_sheep));
    }

    //test equality
    #[test]
    fn order() {
        let sheep0 = Sheep::from_vec(vec![C0, C1, C2, OMEGA]);
        let sheep1 = Sheep::from_vec(vec![OMEGA, C2, C1, C0]);
        let ideal0 = Ideal::from_vec(&[sheep0, sheep1]);

        let sheep2 = Sheep::from_vec(vec![OMEGA, C1, C2, OMEGA]);
        let sheep3 = Sheep::from_vec(vec![OMEGA, C2, C1, OMEGA]);
        let ideal1 = Ideal::from_vec(&[sheep2, sheep3]);

        let sheep4 = Sheep::from_vec(vec![OMEGA, C2, C2, OMEGA]);
        let ideal2 = Ideal::from_vec(&[sheep4]);

        assert!(ideal0.is_contained_in(&ideal1));
        assert!(ideal1.is_contained_in(&ideal2));
        assert!(ideal0.is_contained_in(&ideal2));
    }

    #[test]
    fn restrict_to() {
        let sheep0 = Sheep::from_vec(vec![C0, C1, C2, OMEGA]);
        let sheep1 = Sheep::from_vec(vec![OMEGA, C2, C1, C0]);
        let mut ideal0 = Ideal::from_vec(&[sheep0, sheep1]);

        let sheep2 = Sheep::from_vec(vec![OMEGA, C1, C2, OMEGA]);
        let sheep3 = Sheep::from_vec(vec![OMEGA, C2, C1, OMEGA]);
        let mut ideal1 = Ideal::from_vec(&[sheep2, sheep3]);

        let sheep4 = Sheep::from_vec(vec![C1, OMEGA, C1, C2]);
        let sheep5 = Sheep::from_vec(vec![C2, OMEGA, C1, C1]);
        let ideal2 = Ideal::from_vec(&[sheep4, sheep5]);

        let ideal0original = ideal0.clone();
        let changed0 = ideal0.restrict_to(&ideal1);
        assert!(!changed0);
        assert_eq!(ideal0, ideal0original);

        let ideal1original = ideal1.clone();
        let changed1 = ideal1.restrict_to(&ideal2);
        assert!(changed1);
        assert!(ideal1 != ideal1original);
        assert_eq!(
            ideal1,
            Ideal::from_vec(&[
                Sheep::from_vec(vec![C2, C2, C1, C1]),
                Sheep::from_vec(vec![C1, C2, C1, C2])
            ])
        );
        assert_eq!(
            ideal1,
            Ideal::from_vec(&[
                Sheep::from_vec(vec![C2, C2, C1, C1]),
                Sheep::from_vec(vec![C1, C1, C1, C2]),
                Sheep::from_vec(vec![C1, C2, C1, C2])
            ])
        );
        assert_eq!(
            ideal1,
            Ideal::from_vec(&[
                Sheep::from_vec(vec![C1, C2, C1, C2]),
                Sheep::from_vec(vec![C2, C2, C1, C1]),
                Sheep::from_vec(vec![C1, C1, C1, C2]),
                Sheep::from_vec(vec![C2, C1, C1, C1]),
            ])
        );
    }

    //test issafe
    #[test]
    fn is_safe() {
        /*Self::is_safe(&candidate, edges, sheep) */
        let edges = crate::graph::Graph::from_vec(vec![(0, 1), (0, 2)]);
        let sheep0 = Sheep::from_vec(vec![C0, C1, C0]);
        let sheep1 = Sheep::from_vec(vec![C0, C0, C1]);
        let ideal = Ideal::from_vec(&[sheep0, sheep1]);

        let candidate = vec![C1, C0, C0];
        assert!(ideal.is_safe(&candidate, &edges));
    }

    #[test]
    fn is_safe2() {
        let c4 = Coef::Value(4);
        /*Self::is_safe(&candidate, edges, sheep) */
        let edges = crate::graph::Graph::from_vec(vec![(0, 1), (0, 2)]);
        let ideal = Ideal::from_vecs(&[&[C0, c4, C0], &[C0, C0, c4]]);

        let candidate = vec![c4, C0, C0];
        assert!(ideal.is_safe(&candidate, &edges));
    }

    #[test]
    fn pre_image1() {
        let edges =
            crate::graph::Graph::from_vec(vec![(0, 0), (1, 1), (1, 2), (2, 2), (2, 3), (3, 3)]);
        let ideal0 = Ideal::from_vecs(&[&[C0, C1, C2, OMEGA]]);

        let pre_image0 = ideal0.pre_image(&edges);
        assert_eq!(
            pre_image0,
            Ideal::from_vec(&[
                Sheep::from_vec(vec![C0, C1, C1, OMEGA]),
                Sheep::from_vec(vec![C0, C0, C2, OMEGA]),
            ])
        );
    }

    #[test]
    fn pre_image1bis() {
        let edges =
            crate::graph::Graph::from_vec(vec![(0, 0), (1, 1), (1, 2), (2, 2), (2, 3), (3, 3)]);
        let ideal1 = Ideal::from_vecs(&[&[OMEGA, C1, C2, OMEGA], &[OMEGA, C2, C1, OMEGA]]);
        let pre_image1 = ideal1.pre_image(&edges);
        assert_eq!(
            pre_image1,
            Ideal::from_vecs(&[
                &[OMEGA, C2, C0, OMEGA],
                &[OMEGA, C0, C2, OMEGA],
                &[OMEGA, C1, C1, OMEGA]
            ])
        );
    }

    #[test]
    fn pre_image2() {
        let edges = crate::graph::Graph::from_vec(vec![(0, 1), (0, 2)]);

        let sheep0 = Sheep::from_vec(vec![C0, C0, OMEGA]);
        let sheep1 = Sheep::from_vec(vec![C0, OMEGA, C0]);
        let ideal0 = Ideal::from_vec(&[sheep0, sheep1]);

        let pre_image0 = ideal0.pre_image(&edges);
        assert_eq!(
            pre_image0,
            Ideal::from_vec(&[Sheep::from_vec(vec![C1, OMEGA, OMEGA]),])
        );
    }
}
