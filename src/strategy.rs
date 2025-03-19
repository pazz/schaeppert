use crate::coef::OMEGA;
use crate::ideal::Ideal;
use crate::nfa;
use crate::sheep::Sheep;

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Strategy(HashMap<nfa::Letter, Ideal>);

impl Strategy {
    pub fn get_maximal_strategy(dim: usize, letters: &[nfa::Letter]) -> Self {
        let sheep = Sheep::new(dim, OMEGA);
        let maximal_ideal = Ideal::from_vec([sheep].into());
        Strategy(
            letters
                .iter()
                .map(|l| (*l, maximal_ideal.clone()))
                .collect(),
        )
    }

    pub fn is_defined_on(&self, source: &Sheep) -> bool {
        self.0.values().any(|ideal| ideal.contains(source))
    }

    pub(crate) fn restrict_to_ideal(&self, _winning_ideal: Ideal) -> bool {
        todo!()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&nfa::Letter, &Ideal)> {
        self.0.iter()
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec: Vec<String> = self
            .0
            .iter()
            .map(|x| format!("action {}\\n {}", x.0, x.1))
            .collect();
        write!(f, "{}", vec.join("\n"))
    }
}
