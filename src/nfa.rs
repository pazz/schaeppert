/*
authors @GBathie + @Numero7
 */

use crate::graph::Graph;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Transition {
    pub from: State,
    pub label: Letter,
    pub to: State,
}

pub struct Nfa {
    states: Vec<String>,
    initial: HashSet<State>,
    accepting: HashSet<State>,
    transitions: Vec<Transition>,
}

pub type State = usize;
pub type Letter = String;

impl Nfa {
    pub fn new(nb_states: usize) -> Self {
        Nfa {
            states: (0..nb_states).map(|index| index.to_string()).collect(),
            initial: HashSet::new(),
            accepting: HashSet::new(),
            transitions: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn from_states(states: &[&str]) -> Self {
        Nfa {
            states: states.iter().map(|&l| l.to_string()).collect(),
            initial: HashSet::new(),
            accepting: HashSet::new(),
            transitions: vec![],
        }
    }

    pub fn get_alphabet(&self) -> Vec<&str> {
        let mut letters = Vec::new();
        self.transitions.iter().for_each(|t| {
            let label = t.label.as_str();
            if !letters.contains(&label) {
                letters.push(label);
            }
        });
        letters
    }

    pub fn add_transition_by_index(&mut self, from: State, to: State, label: char) {
        self.check_state(from);
        self.check_state(to);
        self.transitions.push(Transition {
            from,
            label: label.to_string(),
            to,
        });
    }

    #[allow(dead_code)]
    pub fn add_transition(&mut self, from: &str, to: &str, label: &str) {
        let from = self.get_state_index(from);
        let to = self.get_state_index(to);
        self.check_state(from);
        self.check_state(to);
        self.transitions.push(Transition {
            from,
            label: label.to_string(),
            to,
        });
    }

    fn check_state(&self, q: State) {
        assert!(q < self.nb_states(), "State {} is not in the NFA", q)
    }

    pub fn add_initial_by_index(&mut self, q: State) {
        self.check_state(q);
        self.initial.insert(q);
    }

    pub fn add_final_by_index(&mut self, q: State) {
        self.check_state(q);
        self.accepting.insert(q);
    }

    pub fn add_initial(&mut self, q: &str) {
        self.initial.insert(self.get_state_index(q));
    }

    pub fn add_final(&mut self, q: &str) {
        self.accepting.insert(self.get_state_index(q));
    }

    pub fn nb_states(&self) -> usize {
        self.states.len()
    }

    pub(crate) fn initial_states(&self) -> HashSet<State> {
        self.initial.clone()
    }

    pub(crate) fn final_states(&self) -> Vec<State> {
        self.accepting.iter().cloned().collect()
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.get_alphabet().iter().all(|letter| {
            (0..self.nb_states()).all(|state| {
                self.transitions
                    .iter()
                    .any(|t| t.from == state && t.label == *letter)
            })
        })
    }

    //overload [] opertor to turn state labels to state index
    pub fn get_state_index(&self, label: &str) -> State {
        self.states
            .iter()
            .position(|x| x == label)
            .expect("State not found")
    }

    pub(crate) fn get_nfa(name: &str) -> Nfa {
        match name {
            "((a#b){a,b})#" => {
                let mut nfa = Nfa::new(6);
                nfa.add_initial_by_index(0);
                nfa.add_final_by_index(4);
                nfa.add_transition_by_index(0, 0, 'a');
                nfa.add_transition_by_index(0, 1, 'a');
                nfa.add_transition_by_index(1, 0, 'a');
                nfa.add_transition_by_index(1, 1, 'a');
                nfa.add_transition_by_index(4, 4, 'a');
                nfa.add_transition_by_index(5, 5, 'a');

                nfa.add_transition_by_index(0, 0, 'b');
                nfa.add_transition_by_index(4, 4, 'b');
                nfa.add_transition_by_index(5, 5, 'b');

                nfa.add_transition_by_index(1, 2, 'b');
                nfa.add_transition_by_index(1, 3, 'b');

                nfa.add_transition_by_index(2, 4, 'a');
                nfa.add_transition_by_index(2, 5, 'b');
                nfa.add_transition_by_index(3, 4, 'b');
                nfa.add_transition_by_index(3, 5, 'a');
                nfa
            }
            _ => panic!("Unknown NFA"),
        }
    }

    pub(crate) fn get_support(&self, action: &str) -> crate::graph::Graph {
        Graph::new(
            &self
                .transitions
                .iter()
                .filter(|t| t.label == *action)
                .map(|t| (t.from, t.to))
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let mut nfa = Nfa::from_states(&["toto", &"titi"]);
        nfa.add_transition("toto", "titi", "label1");
        nfa.add_transition("titi", "toto", "label2");
        nfa.add_initial("toto");
        nfa.add_final("titi");
    }

    #[test]
    fn parity() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition_by_index(0, 1, 'a');
        nfa.add_transition_by_index(1, 0, 'a');
        nfa.add_transition_by_index(0, 0, 'b');
        nfa.add_transition_by_index(1, 1, 'b');
        nfa.add_initial_by_index(0);
        nfa.add_final_by_index(0);

        let letters = nfa.get_alphabet();
        assert!(letters.contains(&"a"));
        assert!(letters.contains(&"b"));
        assert!(letters.len() == 2);
    }

    #[test]
    fn a_b_star() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition_by_index(0, 1, 'a');
        nfa.add_transition_by_index(1, 0, 'b');
        nfa.add_initial_by_index(0);
        nfa.add_final_by_index(0);
    }
}
