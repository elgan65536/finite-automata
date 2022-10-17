use std::collections::HashMap;

use crate::dfa::*;

pub fn empty(alph: &[char]) -> DFA {
    let mut result = DFA {
        states: 1,
        alphabet: alph.to_vec(),
        accepting: vec![false],
        transition: HashMap::new(),
    };
    for &char in alph {
        result.transition.insert((0, char), 0);
    }
    result
}

pub fn all_strings(alph: &[char]) -> DFA {
    let mut result = DFA {
        states: 1,
        alphabet: alph.to_vec(),
        accepting: vec![true],
        transition: HashMap::new(),
    };
    for &char in alph {
        result.transition.insert((0, char), 0);
    }
    result
}

pub fn modulo_n(alph: &[char], chars: &[char], accept: i32, n: i32) -> Result<DFA, ()> {
    if accept >= n {
        return Err(());
    }
    let mut result = DFA {
        states: n,
        alphabet: alph.to_vec(),
        accepting: vec![false; n as usize],
        transition: HashMap::new(),
    };
    result.accepting[accept as usize] = true;
    for i in 0..n {
        for &char in alph {
            if chars.contains(&char) {
                result.transition.insert((i, char), (i + 1) % n);
            } else {
                result.transition.insert((i, char), i);
            }
        }
    }
    Ok(result.optimize())
}

pub fn exact_length(alph: &[char], chars: &[char], n: i32) -> Result<DFA, ()> {
    if n < 0 {
        return Err(());
    }
    let mut result = DFA {
        states: n + 2,
        alphabet: alph.to_vec(),
        accepting: vec![false; n as usize + 2],
        transition: HashMap::new(),
    };
    result.accepting[n as usize] = true;
    for i in 0..=n {
        for &char in alph {
            if chars.contains(&char) {
                result.transition.insert((i, char), i + 1);
            } else {
                result.transition.insert((i, char), i);
            }
        }
    }
    for &char in alph {
        result.transition.insert((n + 1, char), n + 1);
    }
    Ok(result)
}

pub fn length_or_less(alph: &[char], chars: &[char], n: i32) -> Result<DFA, ()> {
    if n < 0 {
        return Err(());
    }
    let mut result = DFA {
        states: n + 2,
        alphabet: alph.to_vec(),
        accepting: vec![false; n as usize + 2],
        transition: HashMap::new(),
    };
    for i in 0..=n {
        result.accepting[i as usize] = true;
    }
    for i in 0..=n {
        for &char in alph {
            if chars.contains(&char) {
                result.transition.insert((i, char), i + 1);
            } else {
                result.transition.insert((i, char), i);
            }
        }
    }
    for &char in alph {
        result.transition.insert((n + 1, char), n + 1);
    }
    Ok(result)
}

pub fn only_string(alph: &[char], string: &str) -> Result<DFA, ()> {
    for char in string.chars() {
        if !alph.contains(&char) {
            return Err(());
        }
    }
    let length = string.chars().count() as i32;
    let mut result = DFA {
        states: length + 2,
        alphabet: alph.to_vec(),
        accepting: vec![false; length as usize + 2],
        transition: HashMap::new(),
    };
    result.accepting[length as usize] = true;
    for i in 0..length + 2 {
        for &char in alph {
            result.transition.insert((i, char), length + 1);
        }
    }
    for (i, char) in string.chars().enumerate() {
        result.transition.insert((i as i32, char), i as i32 + 1);
    }
    Ok(result)
}

pub fn begins_with(alph: &[char], string: &str) -> Result<DFA, ()> {
    for char in string.chars() {
        if !alph.contains(&char) {
            return Err(());
        }
    }
    let length = string.chars().count() as i32;
    let mut result = DFA {
        states: length + 2,
        alphabet: alph.to_vec(),
        accepting: vec![false; length as usize + 2],
        transition: HashMap::new(),
    };
    result.accepting[length as usize] = true;
    for i in 0..length + 2 {
        for &char in alph {
            result.transition.insert((i, char), length + 1);
        }
    }
    for (i, char) in string.chars().enumerate() {
        result.transition.insert((i as i32, char), i as i32 + 1);
    }
    for &char in alph {
        result.transition.insert((length, char), length);
    }
    Ok(result)
}

pub fn ends_wtih(alph: &[char], string: &str) -> Result<DFA, ()> {
    for char in string.chars() {
        if !alph.contains(&char) {
            return Err(());
        }
    }
    let length = string.chars().count() as i32;
    let mut result = DFA {
        states: length + 1,
        alphabet: alph.to_vec(),
        accepting: vec![false; length as usize + 1],
        transition: HashMap::new(),
    };
    result.accepting[length as usize] = true;
    for i in 0..length + 1 {
        for &char in alph {
            let mut substr = string[0..i as usize].to_string();
            substr.push(char);
            result.transition.insert((i, char), substring_compare(string, &substr));
        }
    }
    for (i, char) in string.chars().enumerate() {
        result.transition.insert((i as i32, char), i as i32 + 1);
    }
    Ok(result)
}

pub fn contains_substring(alph: &[char], string: &str) -> Result<DFA, ()> {
    for char in string.chars() {
        if !alph.contains(&char) {
            return Err(());
        }
    }
    let length = string.chars().count() as i32;
    let mut result = DFA {
        states: length + 1,
        alphabet: alph.to_vec(),
        accepting: vec![false; length as usize + 1],
        transition: HashMap::new(),
    };
    result.accepting[length as usize] = true;
    for i in 0..length + 1 {
        for &char in alph {
            let mut substr = string[0..i as usize].to_string();
            substr.push(char);
            result.transition.insert((i, char), substring_compare(string, &substr));
        }
    }
    for (i, char) in string.chars().enumerate() {
        result.transition.insert((i as i32, char), i as i32 + 1);
    }
    for &char in alph {
        result.transition.insert((length, char), length);
    }
    Ok(result)
}

fn substring_compare(begin: &str, end: &str) -> i32 {
    let mut result = 0;
    for i in 1..end.len() {
        if begin[0..i] == end[end.len() - i..end.len()] {
            result = i as i32;
        }
    }
    result
}
