/*
authors @GBathie + @Numero7
 */

use std::collections::HashSet;

pub struct Transition {
    from: State,
    letter: Letter,
    to: State,
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
        return Nfa {
            nb_states: nb_states,
            initial: HashSet::new(),
            accepting: HashSet::new(),
            transitions: vec![],
        };
    }

    pub fn add_transition(&mut self, from: State, to: State, label: char) {
        self._check_state(from);
        self._check_state(to);
        self.transitions.push(Transition {
            from: from,
            letter: label,
            to: to,
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

    fn step(&self, states: HashSet<State>, a: char) -> HashSet<State> {
        return states
            .into_iter()
            .flat_map(|q| {
                self.transitions
                    .iter()
                    .filter(move |t| t.from == q && t.letter == a)
                    .map(|t| t.to.clone())
            })
            .collect();
    }

    pub fn accepts(&self, s: &str) -> bool {
        let mut reachable_states = self.initial.clone();
        for a in s.chars() {
            reachable_states = self.step(reachable_states, a)
        }
        return reachable_states.iter().any(|q| self.accepting.contains(q));
    }

    pub fn nb_states(&self) -> usize {
        self.nb_states
    }

    pub fn states(&self) -> Vec<State> {
        (0..self.nb_states).collect()
    }

    pub fn is_initial(&self, state: &State) -> bool {
        self.initial.contains(state)
    }

    pub fn is_final(&self, state: &State) -> bool {
        self.accepting.contains(state)
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

        assert!(nfa.accepts(""));
        assert!(nfa.accepts("ababbaba"));
        assert!(nfa.accepts("aabbaa"));
        assert!(!nfa.accepts("abbaa"));
        assert!(!nfa.accepts("aaa"));
    }

    #[test]
    fn a_b_star() {
        let mut nfa = Nfa::new(2);
        nfa.add_transition(0, 1, 'a');
        nfa.add_transition(1, 0, 'b');
        nfa.add_initial(0);
        nfa.add_final(0);

        assert!(nfa.accepts(""));
        assert!(nfa.accepts("ababab"));
        assert!(nfa.accepts("abab"));
        assert!(!nfa.accepts("aba"));
        assert!(!nfa.accepts("aababa"));
        assert!(!nfa.accepts("abababba"));
    }
}
