mod coef;
mod flow;
mod graph;
mod ideal;
mod nfa;
mod partitions;
mod semigroup;
mod sheep;
mod solver;
mod strategy;

fn main() {
    env_logger::init();
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
        4.  symbolic flow. stores the set of OMEGA and the set of 1 or OMEGA
        5.  instance of the symbolic path problem
        6.  symbolic monoid
         */
    let nfa = nfa::Nfa::get_nfa("((a#b){a,b})#");
    let solution = solver::solve(&nfa);
    println!("{}", solution);
}
