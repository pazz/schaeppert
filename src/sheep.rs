use crate::coef::Coef;
use std::fmt;
use std::vec::Vec;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Sheep(Vec<Coef>);

impl Sheep {
    pub fn new(dimension: usize, val: Coef) -> Self {
        Sheep(vec![val; dimension])
    }

    pub(crate) fn from_vec(vec: Vec<Coef>) -> Sheep {
        Sheep(vec)
    }

    pub fn is_below(&self, other: &Self) -> bool {
        self.0.iter().enumerate().all(|(i, &x)| x <= other.0[i])
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, i: usize) -> Coef {
        self.0[i]
    }

    pub(crate) fn set(&mut self, state: usize, val: Coef) {
        self.0[state] = val;
    }
}

impl fmt::Display for Sheep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = self
            .0
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" , ");
        write!(f, "| {} |", content)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::coef::OMEGA;
    use crate::coef::C0;

    #[test]
    fn is_below() {
        let master_sheep = Sheep(vec![OMEGA, OMEGA]);
        let medium_sheep = Sheep(vec![Coef::Value(7), Coef::Value(7)]);
        let ini_sheep = Sheep(vec![OMEGA, C0]);
        let final_sheep = Sheep(vec![C0, OMEGA]);

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
