mod arena;
mod nfa;
mod sheep;
mod solver;

fn main() {
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
    let nfa = get_nfa();
    let arena = arena::Arena::from_nfa(nfa);
    let solver = solver::Solver::new(arena);
    let solution = solver.solve();
    println!("Result: {}", solution.to_string());
}

fn get_nfa() -> nfa::Nfa {
    let mut nfa = nfa::Nfa::new(5);
    /* it all starts in 0 */
    nfa.add_initial(0);

    /* 4 is the unique final state, absorbing */
    nfa.add_transition(4, 4, 'a');
    nfa.add_transition(4, 4, 'b');
    nfa.add_transition(4, 4, 'c');
    nfa.add_final(4);

    /* play a for some time, until a single token remains in 0 */
    nfa.add_transition(0, 1, 'a');
    nfa.add_transition(1, 1, 'a');

    /* then play b, the token might move to either 2 or 3, while other tokens stay in 1 */
    nfa.add_transition(0, 2, 'b');
    nfa.add_transition(0, 3, 'b');
    nfa.add_transition(1, 1, 'b');

    /* designate the side of the token, and save it to the final state 5 */
    nfa.add_transition(2, 4, 'l');
    nfa.add_transition(3, 4, 'r');

    /* other tokens are back to 0, ready to proceed with another round */
    nfa.add_transition(1, 0, 'l');
    nfa.add_transition(1, 0, 'r');

    return nfa;
}
