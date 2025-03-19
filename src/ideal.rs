use crate::sheep::Sheep;
use std::fmt;
use std::{collections::HashSet, vec::Vec};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Ideal(HashSet<Sheep>);

impl Ideal {
    pub(crate) fn sheeps(&self) -> impl Iterator<Item = &Sheep> {
        self.0.iter()
    }

    pub(crate) fn from_vec(into: Vec<Sheep>) -> Self {
        Ideal(into.into_iter().collect())
    }

    pub(crate) fn contains(&self, source: &Sheep) -> bool {
        self.0.iter().any(|x| source.is_below(x))
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
    use crate::coef::{C0, C1, OMEGA};

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
}
