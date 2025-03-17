/*
authors @GBathie + @Numero7
 */

use std::collections::HashSet;

#[derive(Clone)]
pub struct Transition {
    pub from: State,
    pub letter: Letter,
    pub to: State,
}

pub struct Nfa {
    nb_states: usize,
    initial: HashSet<State>,
    accepting: HashSet<State>,
    transitions: Vec<Transition>,
}

pub type State = usize;
pub type Letter = char;

impl Nfa {
    pub fn new(nb_states: usize) -> Self {
        Nfa {
            nb_states,
            initial: HashSet::new(),
            accepting: HashSet::new(),
            transitions: vec![],
        }
    }

    pub fn get_letters(&self) -> Vec<Letter> {
        let mut letters = Vec::new();
        self.transitions.iter().for_each(|t| {
            if !letters.contains(&t.letter) {
                letters.push(t.letter);
            }
        });
        letters
    }

    pub fn add_transition(&mut self, from: State, to: State, label: char) {
        self._check_state(from);
        self._check_state(to);
        self.transitions.push(Transition {
            from,
            letter: label,
            to,
        });
    }

    fn _check_state(&self, q: State) {
        if q >= self.nb_states {
            panic!("State {} is not in the NFA", q);
        }
    }

    pub fn add_initial(&mut self, q: State) {
        self._check_state(q);
        self.initial.insert(q);
    }

    pub fn add_final(&mut self, q: State) {
        self._check_state(q);
        self.accepting.insert(q);
    }

    pub fn nb_states(&self) -> usize {
        self.nb_states
    }

    pub(crate) fn transitions(&self) -> Vec<Transition> {
        self.transitions.clone()
    }

    pub(crate) fn initial_states(&self) -> HashSet<State> {
        self.initial.clone()
    }

    pub(crate) fn final_states(&self) -> HashSet<State> {
        self.accepting.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parity() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'a');
        nfa.add_transition(0, 0, 'b');
        nfa.add_transition(1, 1, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);

        let letters = nfa.get_letters();
        assert!(letters.contains(&'a'));
        assert!(letters.contains(&'b'));
        assert!(letters.len() == 2);
    }

    #[test]
    fn a_b_star() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);
    }
}
