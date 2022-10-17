use std::{fs::File, io::Write};

use itertools::Itertools;

use crate::{dfa::DFA, dfa_gen};

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

        "gen" | "generate" => {
            if let Some(dfa) = generate(&args) {
                if let Ok(mut file) = File::create(&args[3]) {
                    file.write_all(dfa.to_string().as_bytes()).unwrap();
                } else {
                    println!("error creating file {}", args[2]);
                    return 1;
                };
            } else {
                _gen_help()
            }
        }

        _ => {
            println!("unrecognized command {}", args[1]);
            help();
        }
    }
    0
}

fn generate(args: &[String]) -> Option<DFA> {
    if args.len() < 5 {
        return None;
    }
    let alph: Vec<char> = args[4].split(',').map(|s| s.chars().next().unwrap_or(' ')).collect();
    match args[2].trim().to_lowercase().as_str() {
        "empty" => Some(dfa_gen::empty(&alph)),
        "equal" => {
            if args.len() < 7 {
                return None;
            }
            let chars: Vec<char> = args[5].split(',').map(|s| s.chars().next().unwrap_or(' ')).collect();
            if let Ok(amount) = args[6].parse::<i32>() {
                Some(dfa_gen::exact_length(&alph, &chars, amount).unwrap())
            } else {
                None
            }
        }
        "less" | "less_or_equal" | "less_than_or_equal" => {
            if args.len() < 7 {
                return None;
            }
            let chars: Vec<char> = args[5].split(',').map(|s| s.chars().next().unwrap_or(' ')).collect();
            if let Ok(amount) = args[6].parse::<i32>() {
                Some(dfa_gen::length_or_less(&alph, &chars, amount).unwrap())
            } else {
                None
            }
        }
        "mod" | "modulo" => {
            if args.len() < 8 {
                return None;
            }
            let chars: Vec<char> = args[5].split(',').map(|s| s.chars().next().unwrap_or(' ')).collect();
            if let Ok(amount) = args[6].parse::<i32>() {
                if let Ok(amount2) = args[7].parse::<i32>() {
                    Some(dfa_gen::modulo_n(&alph, &chars, amount, amount2).unwrap())
                } else {
                    None
                }
            } else {
                None
            }
        }
        "only" => {
            if args.len() < 6 {
                return None;
            }
            Some(dfa_gen::only_string(&alph, &args[5]).unwrap())
        }
        "begins" | "begins_with" | "starts" | "starts_wtih" => {
            if args.len() < 6 {
                return None;
            }
            Some(dfa_gen::begins_with(&alph, &args[5]).unwrap())
        }
        "ends" | "ends_with" => {
            if args.len() < 6 {
                return None;
            }
            Some(dfa_gen::ends_wtih(&alph, &args[5]).unwrap())
        }
        "contains" | "substring" | "contains_substring" => {
            if args.len() < 6 {
                return None;
            }
            Some(dfa_gen::contains_substring(&alph, &args[5]).unwrap())
        }
        _ => None,
    }
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
    println!("");
    println!("to generate an automaton from presets:");
    println!("gen <preset> <outfilename> <args...>");
    println!("use 'gen help' to see list of presets.");
}

fn _gen_help() {
    println!("for all generators, alph is the alphabet with all charactersseparated by commas (no spaces),");
    println!("s is a string of characters in alph, chars is a subset of alph, and x and y are integers.");
    println!("empty language: gen empty <outfilename> <alph>");
    println!("strings with number of chars equal to i: gen equal <outfilename> <alph> <chars> <i>");
    println!(
        "strings with number of chars less than or equal to i: gen less_or_equal <outfilename> <alph> <chars> <i>"
    );
    println!("strings with number of chars congruent to i modulo j: gen mod <outfilename> <alph> <chars> <i> <j>");
    println!("only one string: gen only <outfilename> <alph> <string>");
    println!("begins with a certain substring: gen begins <outfilename> <alph> <string>");
    println!("ends with a certain substring: gen ends <outfilename> <alph> <string>");
    println!("contains a certain substring: gen contains <outfilename> <alph> <string>");
}
