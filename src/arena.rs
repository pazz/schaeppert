use crate::flow;
use crate::nfa;
use crate::semigroup;
use crate::sheep;
use crate::sheep::SheepTrait;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Commit {
    pub sheep: sheep::Sheep,
    pub letter: char,
}

#[derive(Clone)]
pub struct Arena {
    dimension: usize,
    configurations: HashSet<sheep::Sheep>,
    commits: HashSet<Commit>,
    source: sheep::Sheep,
    target: sheep::Sheep,
    transitions: HashSet<nfa::Transition>,
}

impl Arena {
    pub fn from_nfa(nfa: nfa::Nfa) -> Self {
        let mut arena = Arena::new(nfa.nb_states());
        /* create a sheep of size arena.dimension and set every coordinate to 1 or 0 depending whether the stte is initial or not  */

        let mega_sheep: Vec<nfa::State> = sheep::SheepTrait::new(nfa.nb_states(), sheep::OMEGA);

        nfa.get_letters().iter().for_each(|&letter| {
            arena.add_commit(letter, &mega_sheep);
        });

        let initial_configuration: Vec<nfa::State> = nfa
            .states()
            .iter()
            .map(|&x| if nfa.is_initial(&x) { sheep::OMEGA } else { 0 }) // Convert to 0 or 1
            .collect();
        arena.set_source(initial_configuration);

        let final_configuration: Vec<nfa::State> = nfa
            .states()
            .iter()
            .map(|&x| if nfa.is_final(&x) { sheep::OMEGA } else { 0 }) // Convert to 0 or 1
            .collect();
        arena.set_target(final_configuration);

        return arena;
    }

    pub fn shrink_to_largest_subarena_without_deadend_nor_sink(&mut self) {
        loop {
            let nb_dead_ends_removed = self.remove_dead_ends();
            println!("Removed {} dead ends", nb_dead_ends_removed);
            let nb_sinks_removed = self.remove_sinks();
            println!("Removed {} sinks", nb_sinks_removed);
            if nb_sinks_removed == 0 {
                break;
            }
        }
    }

    // Remove dead ends and returns true off something changed
    pub fn remove_dead_ends(&mut self) -> usize {
        let non_deadend: HashSet<sheep::Sheep> = self
            .commits
            .iter()
            .map(|commit| commit.sheep.clone())
            .collect();
        let before = self.configurations.len();
        self.configurations.retain(|c| non_deadend.contains(c));
        let after = self.configurations.len();
        return before - after;
    }

    pub fn compute_flow_semigroup(&self) -> semigroup::FlowSemigroup {
        let mut action_flows = HashSet::new();
        for commit in self.commits.iter() {
            let action = commit.letter;
            let edges: HashSet<(usize, usize)> = self.get_edges(action);
            let domain = &commit.sheep;
            let flow = flow::Flow::from_domain_and_edges(domain, &edges);
            action_flows.insert(flow);
        }
        let semigroup = semigroup::FlowSemigroup::compute(action_flows);
        return semigroup;
    }

    pub fn remove_sinks(&mut self) -> usize {
        let monoid = self.compute_flow_semigroup();
        let sinks = monoid.compute_sinks(&self.configurations, &self.target);
        let nb_sinks = sinks.len();
        for sink in sinks {
            self.configurations.remove(&sink);
        }
        return nb_sinks;
    }

    pub fn initial_configuration_belong_to_the_arena(&self) -> bool {
        self.contains(&self.source)
    }

    pub fn contains(&self, sheep: &sheep::Sheep) -> bool {
        self.configurations.iter().any(|c| sheep.is_below(c))
    }

    pub fn is_final(&self, configuration: &sheep::Sheep) -> bool {
        configuration.is_below(&self.target)
    }

    pub fn is_initial(&self, configuration: &sheep::Sheep) -> bool {
        configuration.is_below(&self.source)
    }

    pub fn new(dimension: usize) -> Self {
        return Arena {
            dimension: dimension,
            configurations: HashSet::new(),
            commits: HashSet::new(),
            source: sheep::Sheep::new(),
            target: sheep::Sheep::new(),
            transitions: HashSet::new(),
        };
    }

    fn _check_configuration(&self, sheep: &sheep::Sheep) {
        if sheep.len() != self.dimension {
            panic!("Configuration is not of the dimension of the arena");
        }
    }

    fn add_configuration(&mut self, sheep: &sheep::Sheep) {
        self._check_configuration(sheep);
        self.configurations.insert(sheep.clone());
    }

    fn add_commit(&mut self, letter: char, sheep: &sheep::Sheep) {
        let commit = Commit {
            sheep: sheep.clone(),
            letter: letter,
        };
        self.add_configuration(&commit.sheep);
        self.commits.insert(commit);
    }

    fn set_source(&mut self, configuration: sheep::Sheep) {
        self.add_configuration(&configuration);
        self.source = configuration;
    }

    fn set_target(&mut self, configuration: sheep::Sheep) {
        self.add_configuration(&configuration);
        self.target = configuration;
    }

    fn get_edges(&self, action: char) -> HashSet<(usize, usize)> {
        self.transitions
            .iter()
            .filter(|t| t.letter == action)
            .map(|t| (t.from, t.to))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arena() {
        let mut arena = Arena::new(3);
        let configuration = sheep::Sheep::from(vec![1, 2, 3]);
        arena.add_configuration(&configuration);
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
