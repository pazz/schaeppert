use std::{collections::HashSet, vec::Vec};

//the length of the vector coincides with the number of states
pub type Sheep = Vec<usize>;

pub type Ideal = HashSet<Sheep>;

pub const OMEGA: usize = usize::MAX;

pub trait SheepTrait {
    fn new(dimension: usize, val: usize) -> Self;
    fn is_below(&self, other: &Self) -> bool;
    fn is_in_ideal(&self, ideal: &Ideal) -> bool;
}

impl SheepTrait for Sheep {
    fn new(dimension: usize, val: usize) -> Self {
        Sheep::from(vec![val; dimension])
    }

    fn is_below(&self, other: &Self) -> bool {
        self.iter().enumerate().all(|(i, &x)| x <= other[i])
    }

    fn is_in_ideal(&self, ideal: &Ideal) -> bool {
        ideal.iter().any(|bound| self.is_below(bound))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn herd() {
        let master_sheep = Sheep::from(vec![OMEGA, OMEGA]);
        let medium_sheep = Sheep::from(vec![OMEGA / 2, OMEGA / 2]);
        let ini_sheep = Sheep::from(vec![OMEGA, 0]);
        let final_sheep = Sheep::from(vec![0, OMEGA]);

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
}
