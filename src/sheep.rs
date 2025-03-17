use log::debug;
use std::fmt;
use std::{collections::HashSet, vec::Vec};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Sheep(pub Vec<usize>);

#[derive(Clone, Eq, PartialEq)]
pub struct Ideal(pub HashSet<Sheep>);

impl Sheep {
    pub const OMEGA: usize = usize::MAX;

    pub fn new(dimension: usize, val: usize) -> Self {
        Sheep(vec![val; dimension])
    }

    pub fn is_below(&self, other: &Self) -> bool {
        self.0.iter().enumerate().all(|(i, &x)| x <= other.0[i])
    }

    pub fn is_in_ideal(&self, ideal: &Ideal) -> bool {
        debug!("Checking whether {} belongs to {}", self, ideal);
        ideal.0.iter().any(|bound| self.is_below(bound))
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, i: usize) -> usize {
        self.0[i]
    }

    pub(crate) fn set(&mut self, state: usize, val: usize) {
        self.0[state] = val;
    }
}

impl fmt::Display for Sheep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = self
            .0
            .iter()
            .map(|&x| match x {
                0 => ".",
                1 => "1",
                _ => "w",
            })
            .collect::<Vec<_>>()
            .join(" , ");
        write!(f, "| {} |", content)
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

    #[test]
    fn is_below() {
        let master_sheep = Sheep(vec![Sheep::OMEGA, Sheep::OMEGA]);
        let medium_sheep = Sheep(vec![Sheep::OMEGA / 2, Sheep::OMEGA / 2]);
        let ini_sheep = Sheep(vec![Sheep::OMEGA, 0]);
        let final_sheep = Sheep(vec![0, Sheep::OMEGA]);

        assert!(master_sheep.is_below(&master_sheep));
        assert!(medium_sheep.is_below(&master_sheep));
        assert!(ini_sheep.is_below(&master_sheep));
        assert!(final_sheep.is_below(&master_sheep));

        assert!(!master_sheep.is_below(&medium_sheep));
        assert!(medium_sheep.is_below(&medium_sheep));
        assert!(!ini_sheep.is_below(&medium_sheep));
        assert!(!final_sheep.is_below(&medium_sheep));

        assert!(!master_sheep.is_below(&ini_sheep));
        assert!(!medium_sheep.is_below(&ini_sheep));
        assert!(ini_sheep.is_below(&ini_sheep));
        assert!(!final_sheep.is_below(&ini_sheep));

        assert!(!master_sheep.is_below(&final_sheep));
        assert!(!medium_sheep.is_below(&final_sheep));
        assert!(!ini_sheep.is_below(&final_sheep));
        assert!(final_sheep.is_below(&final_sheep));
    }

    #[test]
    fn is_in_ideal() {
        let omega = Sheep::OMEGA;
        let master_sheep = Sheep(vec![omega, omega]);
        let medium_sheep = Sheep(vec![1, 1]);
        let ini_sheep = Sheep(vec![1, 0]);
        let final_sheep = Sheep(vec![0, 2]);

        let ideal = Ideal([ini_sheep.clone(), final_sheep.clone()].into());
        assert!(ini_sheep.is_in_ideal(&ideal));
        assert!(final_sheep.is_in_ideal(&ideal));
        assert!(!master_sheep.is_in_ideal(&ideal));
        assert!(!medium_sheep.is_in_ideal(&ideal));

        let ideal2 = Ideal([medium_sheep.clone()].into());
        assert!(ini_sheep.is_in_ideal(&ideal2));
        assert!(!final_sheep.is_in_ideal(&ideal2));
        assert!(!master_sheep.is_in_ideal(&ideal2));
        assert!(medium_sheep.is_in_ideal(&ideal2));
    }
}
