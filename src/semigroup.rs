use crate::flow;
use crate::flow::FlowTrait;
use crate::ideal;
use crate::sheep;
use std::collections::HashSet; // for distinct method

pub struct FlowSemigroup {
    pub flows: HashSet<flow::Flow>,
}

impl FlowSemigroup {
    pub fn new() -> Self {
        FlowSemigroup {
            flows: HashSet::new(),
        }
    }

    pub fn compute(action_flows: &HashSet<flow::Flow>) -> Self {
        let mut semigroup = FlowSemigroup::new();
        for flow in action_flows.iter() {
            semigroup.flows.insert(flow.clone());
        }
        semigroup.close_by_product_and_iteration();
        semigroup
    }

    fn close_by_product_and_iteration(&mut self) {
        let mut fresh: HashSet<flow::Flow> = self.flows.iter().cloned().collect();
        while !fresh.is_empty() {
            let flow = fresh.iter().next().unwrap().clone();
            fresh.remove(&flow);
            if self.flows.contains(&flow) {
                continue;
            }
            {
                let iterations = flow.iteration();
                for iteration in iterations {
                    let added = self.flows.insert(iteration.clone());
                    if added {
                        fresh.insert(iteration);
                    }
                }
            }
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
