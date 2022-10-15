# finite-automata

this program simulates finite automata.
automata can be loaded from `.dfa` files. See `begins_with_ab.dfa` for example.

to build, Rust must be installed.
simply build using `cargo build`.

when running, enter the desired command as arguments.
For example, to evaluate the string `abab` on the automaton specified in the file `begins_with_ab.dfa`, run
`cargo run evaluate begins_with_ab.dfa abab` which should return true.

run without arguments for the list of currently implemented commands.
