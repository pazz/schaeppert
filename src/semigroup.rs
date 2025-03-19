use crate::flow;
use crate::flow::FlowTrait;
use crate::ideal;
use crate::sheep;
use std::collections::HashSet; // for distinct method
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
        let mut fresh: HashSet<flow::Flow> = self.flows.clone();
        while !fresh.is_empty() {
            let flow = fresh.iter().next().unwrap().clone();
            fresh.remove(&flow);
            if self.flows.contains(&flow) {
                continue;
            }
            fresh.insert(flow.iteration());
            {
                let right_products = self.flows.iter().map(|other| flow.product(other));
                let left_products = self.flows.iter().map(|other| other.product(&flow));
                let products: HashSet<flow::Flow> = left_products.chain(right_products).collect();
                for product in products {
                    let added = self.flows.insert(product.clone());
                    if added {
                        fresh.insert(product);
                    }
                }
            }
        }
    }

    pub fn get_winning_ideal(&self, target: &sheep::Sheep) -> ideal::Ideal {
        let roundup = true;
        ideal::Ideal::from_vec(
            self.flows
                .iter()
                .map(|flow| (flow, sheep::Sheep::from_vec(flow.im())))
                .filter(|(_, im)| im.is_below(target))
                .map(|(flow, _)| sheep::Sheep::from_vec(flow.dom(roundup)))
                .collect(),
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
