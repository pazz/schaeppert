use crate::flow;
use crate::ideal;
use crate::nfa;
use std::collections::HashSet; // for distinct method
use std::collections::VecDeque;
use std::fmt;

pub struct FlowSemigroup {
    pub flows: HashSet<flow::Flow>,
}

impl FlowSemigroup {
    pub fn new() -> Self {
        FlowSemigroup {
            flows: HashSet::new(),
        }
    }

    pub fn compute(flows: &HashSet<flow::Flow>) -> Self {
        let mut semigroup = FlowSemigroup::new();
        for flow in flows.iter() {
            semigroup.flows.insert(flow.clone());
        }
        semigroup.close_by_product_and_iteration();
        semigroup
    }

    fn close_by_product_and_iteration(&mut self) {
        let mut fresh: VecDeque<flow::Flow> = self.flows.iter().cloned().collect();
        while !fresh.is_empty() {
            let flow = fresh.pop_front().unwrap();
            if self.flows.iter().any(|other| &flow <= other) {
                continue;
            }
            fresh.push_back(flow.iteration());
            {
                let right_products = self.flows.iter().map(|other| &flow * other);
                let left_products = self.flows.iter().map(|other| other * &flow);
                let products: HashSet<flow::Flow> = left_products.chain(right_products).collect();
                for product in products {
                    let added = self.flows.insert(product.clone());
                    if added {
                        fresh.push_back(product);
                    }
                }
            }
        }
    }

    pub fn get_winning_ideal(&self, target: &[nfa::State]) -> ideal::Ideal {
        ideal::Ideal::from_vec(
            &self
                .flows
                .iter()
                .map(|flow| flow.pre_image(target))
                .collect::<Vec<_>>(),
        )
    }
}

impl fmt::Display for FlowSemigroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flows = self
            .flows
            .iter()
            .map(|flow| flow.to_string())
            .collect::<Vec<_>>();
        flows.sort();
        write!(f, "{}", flows.join("\r\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coef::{C0, C1, OMEGA};
    use crate::flow::Flow;

    //test FlowSemigroup::compute
    #[test]
    fn test_flow_semigroup_compute() {
        let flows: HashSet<Flow> = [Flow::from_entries(2, &[OMEGA, C1, C0, OMEGA])].into();
        let semigroup = FlowSemigroup::compute(&flows);
        let flow = Flow::from_entries(2, &[OMEGA, OMEGA, C0, OMEGA]);
        assert!(semigroup.flows.contains(&flow));
    }
}
