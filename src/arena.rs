use crate::nfa;
use crate::sheep;
use crate::sheep::SheepTrait;
use std::collections::HashSet;

pub struct Arena {
    dimension: usize,
    configurations: HashSet<sheep::Sheep>,
    source: HashSet<sheep::Sheep>,
    target: HashSet<sheep::Sheep>,
}

impl Arena {
    pub fn from_nfa(nfa: nfa::Nfa) -> Self {
        let mut arena = Arena::new(nfa.nb_states());
        /* create a sheep of size arena.dimension and set every coordinate to 1 or 0 depending whether the stte is initial or not  */

        let all_configurations: Vec<nfa::State> =
            nfa.states().iter().map(|&x| sheep::OMEGA).collect();
        arena.add_configuration(all_configurations);

        let initial_configuration: Vec<nfa::State> = nfa
            .states()
            .iter()
            .map(|&x| if nfa.is_initial(&x) { sheep::OMEGA } else { 0 }) // Convert to 0 or 1
            .collect();
        arena.add_source(initial_configuration);

        let final_configuration: Vec<nfa::State> = nfa
            .states()
            .iter()
            .map(|&x| if nfa.is_final(&x) { sheep::OMEGA } else { 0 }) // Convert to 0 or 1
            .collect();
        arena.add_target(final_configuration);

        return arena;
    }

    pub fn contains(&self, configuration: &sheep::Sheep) -> bool {
        self.configurations
            .iter()
            .any(|c| configuration.is_below(c))
    }

    pub fn is_final(&self, configuration: &sheep::Sheep) -> bool {
        self.target.iter().any(|c| configuration.is_below(c))
    }

    pub fn is_initial(&self, configuration: &sheep::Sheep) -> bool {
        self.source.iter().any(|c| configuration.is_below(c))
    }

    pub fn new(dimension: usize) -> Self {
        return Arena {
            dimension: dimension,
            configurations: HashSet::new(),
            source: HashSet::new(),
            target: HashSet::new(),
        };
    }

    fn _check_configuration(&self, configuration: &sheep::Sheep) {
        if configuration.len() != self.dimension {
            panic!("Configuration is not of the dimension of the arena");
        }
    }

    fn add_configuration(&mut self, configuration: sheep::Sheep) {
        self._check_configuration(&configuration);
        self.configurations.insert(configuration);
    }

    fn add_source(&mut self, configuration: sheep::Sheep) {
        self.source.insert(configuration.clone());
        self.add_configuration(configuration);
    }

    fn add_target(&mut self, configuration: sheep::Sheep) {
        self.target.insert(configuration.clone());
        self.add_configuration(configuration);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arena() {
        let mut arena = Arena::new(3);
        let configuration = sheep::Sheep::from(vec![1, 2, 3]);
        arena.add_configuration(configuration.clone());
        assert!(arena.configurations.contains(&configuration));
    }

    #[test]
    fn nfa() {
        let mut nfa = nfa::Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'a');
        nfa.add_transition(0, 0, 'b');
        nfa.add_transition(1, 1, 'b');
        nfa.add_initial(0);
        nfa.add_final(1);

        let arena = Arena::from_nfa(nfa);
        assert!(arena.dimension == 2);

        let master_sheep = sheep::Sheep::from(vec![sheep::OMEGA, sheep::OMEGA]);
        let medium_sheep = sheep::Sheep::from(vec![sheep::OMEGA / 2, sheep::OMEGA / 2]);
        let ini_sheep = sheep::Sheep::from(vec![sheep::OMEGA, 0]);
        let final_sheep = sheep::Sheep::from(vec![0, sheep::OMEGA]);
        assert!(arena.contains(&master_sheep));
        assert!(arena.contains(&medium_sheep));
        assert!(arena.contains(&ini_sheep));
        assert!(arena.contains(&final_sheep));

        assert!(!arena.is_initial(&master_sheep));
        assert!(!arena.is_initial(&medium_sheep));
        assert!(arena.is_initial(&ini_sheep));
        assert!(!arena.is_initial(&final_sheep));

        assert!(!arena.is_final(&master_sheep));
        assert!(!arena.is_final(&medium_sheep));
        assert!(!arena.is_final(&ini_sheep));
        assert!(arena.is_final(&final_sheep));
    }
}
