use crate::coef::OMEGA;
use crate::sheep::Sheep;
use std::fmt;
use std::{collections::HashSet, vec::Vec};

#[derive(Clone, Eq, Debug)]
pub struct Ideal(HashSet<Sheep>);

impl PartialEq for Ideal {
    fn eq(&self, other: &Self) -> bool {
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
        let dim = self.0.iter().next().unwrap().len().clone();
        let mut result = Ideal::from_vec(
            &self
                .0
                .iter()
                .map(|upper_bound| {
                    Sheep::from_vec(
                        (0..dim)
                            .map(|i| {
                                edges
                                    .get_successors(i)
                                    .iter()
                                    .map(|j| upper_bound.get(*j))
                                    .min()
                                    .unwrap_or(OMEGA)
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        );
        result.minimize();
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
}

impl fmt::Display for Ideal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        vec.sort();
        write!(f, "{}", vec.join("\r\n\t"))
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

    #[test]
    fn pre_image() {
        let edges =
            crate::graph::Graph::from_vec(vec![(0, 0), (1, 1), (1, 2), (2, 2), (2, 3), (3, 3)]);

        let sheep0 = Sheep::from_vec(vec![C0, C1, C2, OMEGA]);
        let ideal0 = Ideal::from_vec(&[sheep0]);

        let pre_image0 = ideal0.pre_image(&edges);
        assert_eq!(
            pre_image0,
            Ideal::from_vec(&[Sheep::from_vec(vec![C0, C1, C2, OMEGA]),])
        );

        let sheep2 = Sheep::from_vec(vec![OMEGA, C1, C2, OMEGA]);
        let sheep3 = Sheep::from_vec(vec![OMEGA, C2, C1, OMEGA]);
        let ideal1 = Ideal::from_vec(&[sheep2, sheep3]);
        let pre_image1 = ideal1.pre_image(&edges);
        assert_eq!(
            pre_image1,
            Ideal::from_vec(&[Sheep::from_vec(vec![OMEGA, C1, C2, OMEGA]),])
        );
    }
}
