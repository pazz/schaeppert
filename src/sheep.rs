use std::fmt;
use std::vec::Vec;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Sheep(Vec<usize>);

impl Sheep {
    pub const OMEGA: usize = usize::MAX;

    pub fn new(dimension: usize, val: usize) -> Self {
        Sheep(vec![val; dimension])
    }

    pub(crate) fn from_vec(vec: Vec<usize>) -> Sheep {
        Sheep(vec.iter().map(|&x| x as usize).collect())
    }

    pub fn is_below(&self, other: &Self) -> bool {
        self.0.iter().enumerate().all(|(i, &x)| x <= other.0[i])
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
}
