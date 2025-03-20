use crate::coef::OMEGA;
use crate::graph::Graph;
use crate::ideal::Ideal;
use crate::nfa;
use crate::sheep::Sheep;

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Strategy(HashMap<nfa::Letter, Ideal>);

impl Strategy {
    pub fn get_maximal_strategy(dim: usize, letters: &[nfa::Letter]) -> Self {
        let maximal_ideal = Ideal::from_vecs(&[&vec![OMEGA; dim]]);
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

    pub(crate) fn restrict_to(
        &mut self,
        safe: Ideal,
        edges_per_letter: &HashMap<nfa::Letter, Graph>,
    ) -> bool {
        let mut result = false;
        for (a, ideal) in self.0.iter_mut() {
            let edges = edges_per_letter.get(a).unwrap();
            //compute the subset of safe which gurantees to stay in safe when playing the letter
            let very_safe = safe.pre_image(edges);
            result |= ideal.restrict_to(&very_safe);
        }
        result
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
            .map(|x| format!("action {} played in\n{}\n\n", x.0, x.1))
            .collect();
        write!(f, "{}", vec.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sheep::Sheep;

    #[test]
    fn test_strategy() {
        let dim = 2;
        let letters = ['a', 'b'];
        let strategy = Strategy::get_maximal_strategy(dim, &letters);
        let source = Sheep::new(dim, OMEGA);
        assert!(strategy.is_defined_on(&source));
        assert_eq!(
            strategy.0,
            HashMap::from([
                ('a', Ideal::from_vecs(&[&[OMEGA, OMEGA]])),
                ('b', Ideal::from_vecs(&[&[OMEGA, OMEGA]]))
            ])
        );
    }

    //test restrict to_ideal
}
