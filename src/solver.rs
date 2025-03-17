use crate::flow;
use crate::graph::Graph;
use crate::nfa;
use crate::nfa::Nfa;
use crate::semigroup;
use crate::sheep::Sheep;
use crate::strategy::Strategy;
use log::debug;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

pub struct Solution {
    pub result: bool,
    pub maximal_winning_strategy: Strategy,
}

pub fn solve(nfa: &nfa::Nfa) -> Solution {
    let dim = nfa.nb_states();
    let source = get_omega_sheep(dim, nfa.initial_states());
    let target = get_omega_sheep(dim, nfa.final_states());

    let edges = get_edges(nfa);
    let strategy = Strategy::new(dim, &nfa.get_letters());
    let mut result = true;

    while result {
        let action_flows = compute_action_flows(&strategy, &edges);
        debug!("\nAction flows:\n{}", _flows_to_string(&action_flows));
        let semigroup = semigroup::FlowSemigroup::compute(&action_flows);
        let winning_ideal = semigroup.get_winning_ideal(&target);
        debug!("Winning ideal: {}", winning_ideal);
        let changed = strategy.restrict_to_ideal(winning_ideal);
        if !changed {
            break;
        }
        result = strategy.is_defined_on(&source);
    }
    Solution {
        result,
        maximal_winning_strategy: strategy,
    }
}

fn get_omega_sheep(dim: usize, states: HashSet<usize>) -> Sheep {
    let mut sheep = Sheep::new(dim, 0);
    for state in states {
        sheep.set(state, Sheep::OMEGA);
    }
    return sheep;
}

fn get_edges(nfa: &Nfa) -> HashMap<nfa::Letter, Graph> {
    nfa.get_letters()
        .iter()
        .map(|action| (*action, Graph::new(&nfa.transitions(), action)))
        .collect()
}

fn compute_action_flows(
    strategy: &Strategy,
    edges: &HashMap<nfa::Letter, Graph>,
) -> HashSet<flow::Flow> {
    let mut action_flows = HashSet::new();
    for (action, ideal) in strategy.iter() {
        let edges_for_action = edges.get(action).unwrap();
        for sheep in ideal.sheeps() {
            let flow = flow::Flow::from_domain_and_edges(&sheep, edges_for_action);
            action_flows.insert(flow);
        }
    }
    return action_flows;
}

fn _flows_to_string(flows: &HashSet<flow::Flow>) -> String {
    let mut vec: Vec<String> = flows.iter().map(|x| x.to_string()).collect();
    vec.sort();
    vec.join("\r\n")
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.result {
            return write!(
                f,
                "Controllable\nMaximal winning random walk:\n\t{}",
                self.maximal_winning_strategy
            );
        } else {
            write!(
                f,
                "Uncontrollable\nnMaximal winning random walk\n\t{}",
                self.maximal_winning_strategy
            )
        }
    }
}
