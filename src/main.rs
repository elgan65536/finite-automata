use automata::dfa_interpreter::dfa_interpreter;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    process::exit(dfa_interpreter(args))
}
