use crate::arena;
use std::fmt;

pub struct Solver {
    arena: arena::Arena,
}

pub struct Solution {
    pub result: bool,
    pub witness: String,
}

impl Solver {
    pub fn new(arena: &arena::Arena) -> Self {
        return Solver {
            arena: arena.clone(),
        };
    }

    pub fn solve(mut self) -> Solution {
        /*
        fix point computation
         */
        self.arena
            .shrink_to_largest_subarena_without_deadend_nor_sink();
        let result = self.arena.initial_configuration_belong_to_the_arena();
        if result {
            return Solution {
                result: true,
                witness: String::from("witness"),
            };
        } else {
            return Solution {
                result: false,
                witness: String::from(""),
            };
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.result, self.witness)
    }
}
