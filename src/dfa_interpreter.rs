use std::{fs::File, io::Write};

use itertools::Itertools;

use crate::dfa::DFA;

pub fn dfa_interpreter(args: Vec<String>) -> i32 {
    if args.len() < 2 {
        help();
        return 0;
    }
    match args[1].as_str() {
        "eval" | "evaluate" => {
            if args.len() < 3 {
                println!("Correct Syntax: evaluate <dfa> <string> [string] ...");
                return 1;
            }
            if let Ok(dfa) = DFA::open_dfa_file(&args[2]) {
                if args.len() > 3 {
                    for i in 3..args.len() {
                        println!("{}: {}", &args[i], dfa.evaluate_to_string(&args[i]));
                    }
                } else {
                    println!(" : {}", dfa.evaluate_to_string(""));
                }
            } else {
                println!("unable to open file {}", args[2]);
                return 1;
            }
        }

        "negate" | "negation" => {
            if args.len() < 4 {
                println!("Correct Syntax: negate <outfilename> <dfa>");
                return 1;
            }
            if let Ok(dfa) = DFA::open_dfa_file(&args[3]) {
                if let Ok(mut file) = File::create(&args[2]) {
                    file.write_all(dfa.negation().to_string().as_bytes()).unwrap();
                } else {
                    println!("error creating file {}", args[2]);
                    return 1;
                };
            } else {
                println!("unable to open file {}", args[3]);
                return 1;
            }
        }

        "and" | "intersect" => {
            if args.len() < 5 {
                println!("Correct Syntax: intersect <outfilename> <dfa1> <dfa2> [dfa3] ...");
                return 1;
            }
            let mut exit = false;
            let dfas: Vec<DFA> = args[3..args.len()]
                .iter()
                .map(|s| {
                    if let Ok(dfa) = DFA::open_dfa_file(s) {
                        dfa
                    } else {
                        println!("unable to open file {}", args[3]);
                        exit = true;
                        DFA::new()
                    }
                })
                .collect_vec();
            if exit {
                return 1;
            }
            if let Ok(mut file) = File::create(&args[2]) {
                file.write_all(DFA::big_intersect(&dfas).to_string().as_bytes())
                    .unwrap();
            } else {
                println!("error creating file {}", args[2]);
                return 1;
            };
        }

        "or" | "union" => {
            if args.len() < 5 {
                println!("Correct Syntax: union <outfilename> <dfa1> <dfa2> [dfa3] ...");
                return 1;
            }
            let mut exit = false;
            let dfas: Vec<DFA> = args[3..args.len()]
                .iter()
                .map(|s| {
                    if let Ok(dfa) = DFA::open_dfa_file(s) {
                        dfa
                    } else {
                        println!("unable to open file {}", args[3]);
                        exit = true;
                        DFA::new()
                    }
                })
                .collect_vec();
            if exit {
                return 1;
            }
            if let Ok(mut file) = File::create(&args[2]) {
                file.write_all(DFA::big_union(&dfas).to_string().as_bytes()).unwrap();
            } else {
                println!("error creating file {}", args[2]);
                return 1;
            };
        }

        "difference" => {
            if args.len() != 5 {
                println!("Correct Syntax: difference <outfilename> <dfa1> <dfa2>");
                return 1;
            }
            let lhs = match DFA::open_dfa_file(&args[3]) {
                Ok(a) => a,
                Err(_) => {
                    println!("unable to open file {}", args[3]);
                    return 1;
                }
            };
            let rhs = match DFA::open_dfa_file(&args[4]) {
                Ok(a) => a,
                Err(_) => {
                    println!("unable to open file {}", args[4]);
                    return 1;
                }
            };
            if let Ok(mut file) = File::create(&args[2]) {
                file.write_all(lhs.difference(&rhs).to_string().as_bytes()).unwrap();
            } else {
                println!("error creating file {}", args[2]);
                return 1;
            };
        }

        _ => {
            println!("unrecognized command {}", args[1]);
            help();
        }
    }
    0
}

fn help() {
    println!("to evaluate a string in an automaton:");
    println!("evaluate <dfa> <string> [string] ...");
    println!("");
    println!("to create a new automaton from existing files:");
    println!("negate <outfilename> <dfa>");
    println!("intersect <outfilename> <dfa1> <dfa2> [dfa3] ...");
    println!("union <outfilename> <dfa1> <dfa2> [dfa3] ...");
    println!("difference <outfilename> <dfa1> <dfa2>");
}
