use crate::nfa;
use std::{collections::HashSet, fmt};

pub struct Graph(HashSet<(usize, usize)>);

impl Graph {
    pub fn new(transitions: &Vec<nfa::Transition>, letter: &nfa::Letter) -> Self {
        Graph(
            transitions
                .iter()
                .filter(|t| t.letter == *letter)
                .map(|t| (t.from, t.to))
                .collect(),
        )
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(usize, usize)> {
        self.0.iter()
    }

    pub(crate) fn from_vec(to_vec: Vec<(usize, usize)>) -> Graph {
        Graph(to_vec.into_iter().collect())
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vec: Vec<String> = self.0.iter().map(|x| format!("{:?}", x)).collect();
        vec.sort();
        write!(f, "{}", vec.join("\r\n\t"))
    }
}
