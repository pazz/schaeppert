mod arena;
mod nfa;
mod sheep;

fn main() {
    println!("Hello, world!");
    /*
        The objective is to be able to perform the following operations:
        1.	Load an NFA from a file.
        2.	Compute a symbolic representation of the set of winning configurations,
            using a black box that solves the symbolic path problem.
        3. implement the black box that solves the symbolic path problem.
            3a. we need to saturate by two operations: product and iteration
    For this, we need several objects:
        1.	NFAs (Non-deterministic Finite Automata).
        2.	Symbolic configuration
        3.  arena (i.e. set of symbolic configuration)
        4.  symbolic flow. stores the set of omega and the set of 1 or omega
        5.  instance of the symbolic path problem
        6.  symbolic monoid
         */
}
