use std::{cmp::Ordering, collections::HashMap, fmt::Display, fs::File, io::Read, vec};

#[derive(Debug, Clone)]
pub struct DFA {
    pub alphabet: Vec<char>,
    pub states: i32,
    pub accepting: Vec<bool>,
    pub transition: HashMap<(i32, char), i32>,
}

#[derive(Debug)]
pub enum DFAEvaluationError {
    InvalidChar(char),
    InvalidState(i32),
    NoTransition((i32, char)),
}

impl Default for DFA {
    fn default() -> Self {
        Self::new()
    }
}

impl DFA {
    pub fn new() -> Self {
        Self {
            alphabet: Vec::new(),
            states: 0,
            accepting: Vec::new(),
            transition: HashMap::new(),
        }
    }

    pub fn open_dfa_file(path: &str) -> Result<DFA, ()> {
        let mut file1 = File::open(path);
        if file1.is_err() {
            file1 = File::open(format!("{path}.dfa"));
        }
        let mut contents1 = String::new();
        match file1 {
            Ok(_) => {
                file1.unwrap().read_to_string(&mut contents1).unwrap();
                Ok(DFA::from_string(contents1))
            }
            Err(_) => Err(()),
        }
    }

    pub fn from_string(s: String) -> Self {
        let lines: Vec<&str> = s.split('\n').collect();
        let mut result = DFA::new();
        for (i, line) in lines.iter().enumerate() {
            match i {
                0 => match line.split('%').next().unwrap().trim().parse::<i32>() {
                    Ok(a) => {
                        result.states = a;
                        result.accepting = vec![false; a as usize]
                    }
                    Err(_) => result.states = 0,
                },
                1 => {
                    for s in line.split('%').next().unwrap().split(',') {
                        match s.trim().parse::<usize>() {
                            Ok(a) => result.accepting[a] = true,
                            Err(_) => continue,
                        }
                    }
                }
                2 => {
                    for s in line.split('%').next().unwrap().split(',') {
                        result.alphabet.push(match s.trim().chars().next() {
                            Some(a) => a,
                            None => continue,
                        });
                    }
                }
                _ => {
                    for (j, s) in line.split('%').next().unwrap().split(',').enumerate() {
                        if j < result.alphabet.len() {
                            result.transition.insert(
                                (i as i32 - 3, result.alphabet[j]),
                                match s.trim().parse::<i32>() {
                                    Ok(a) => a,
                                    Err(_) => continue,
                                },
                            );
                        }
                    }
                }
            }
        }
        result
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.states.to_string());
        s.push_str(" %states\n");
        for i in 0..self.states {
            if self.accepting[i as usize] {
                s.push(',');
                s.push_str(&i.to_string());
            }
        }
        s.push_str(" %accepting states\n");
        for char in &self.alphabet {
            s.push(',');
            s.push(*char);
        }
        s.push_str(" %alphabet\n");
        for i in 0..self.states {
            for char in &self.alphabet {
                s.push(',');
                s.push_str(&self.transition.get(&(i, *char)).unwrap().to_string());
            }
            s.push_str(" %");
            s.push_str(&i.to_string());
            s.push_str(":\n");
        }
        s.replace("\n,", "\n")
    }

    pub fn evaluate(&self, string: &str) -> Result<bool, DFAEvaluationError> {
        use DFAEvaluationError::*;
        let mut state = 0;
        for char in string.chars() {
            if !self.alphabet.contains(&char) {
                return Err(InvalidChar(char));
            }
            if state >= self.states {
                return Err(InvalidState(state));
            }
            match self.transition.get(&(state, char)) {
                Some(&a) => state = a,
                None => return Err(NoTransition((state, char))),
            }
        }
        Ok(self.accepting[state as usize])
    }

    pub fn evaluate_to_string(&self, s: &str) -> String {
        use DFAEvaluationError::*;
        match self.evaluate(s) {
            Ok(true) => String::from("true"),
            Ok(false) => String::from("false"),
            Err(InvalidChar(c)) => format!("invalid character: {c}"),
            Err(InvalidState(i)) => format!("invalid state: {i}"),
            Err(NoTransition((i, c))) => {
                format!("no transition found for character {c} and state {i}")
            }
        }
    }

    pub fn add_char_ignore(&self, char: char) -> Self {
        let mut result = self.clone();
        if self.alphabet.contains(&char) {
            return result;
        }
        result.alphabet.push(char);
        result
            .transition
            .extend((0..self.states).map(|state| ((state, char), state)));
        result
    }

    pub fn add_char_imitate(&self, char: char, other: char) -> Self {
        let mut result = self.clone();
        if self.alphabet.contains(&char) {
            return result;
        }
        if !self.alphabet.contains(&other) {
            return result;
        }
        result.alphabet.push(char);
        result
            .transition
            .extend((0..self.states).map(|state| ((state, char), *self.transition.get(&(state, other)).unwrap())));
        result
    }

    pub fn add_char_accept(&self, char: char, accept: bool) -> Self {
        let mut result = self.clone();
        if self.alphabet.contains(&char) {
            return result;
        }
        result.states += 1;
        result.accepting.push(accept);
        result.alphabet.push(char);
        result
            .transition
            .extend((0..self.states).map(|state| ((state, char), self.states)));
        result
            .transition
            .extend(result.alphabet.iter().map(|c: &char| ((self.states, *c), self.states)));
        result
    }

    pub fn intersect(&self, rhs: &Self) -> Self {
        if self.states == 0 {
            return rhs.clone();
        }
        let mut lhs_clone = self.clone();
        let mut rhs_clone = rhs.clone();
        for &char in &self.alphabet {
            rhs_clone = rhs_clone.add_char_accept(char, false)
        }
        for &char in &rhs.alphabet {
            lhs_clone = lhs_clone.add_char_accept(char, false)
        }
        let mut result = DFA::new();
        result.alphabet = lhs_clone.alphabet.clone();
        result.states = lhs_clone.states * rhs_clone.states;
        result.accepting = vec![false; result.states as usize];
        for i in 0..lhs_clone.states {
            for j in 0..rhs_clone.states {
                result.accepting[(i * rhs_clone.states + j) as usize] =
                    lhs_clone.accepting[i as usize] && rhs_clone.accepting[j as usize];
                for char in &result.alphabet {
                    result.transition.insert(
                        (i * rhs_clone.states + j, *char),
                        *lhs_clone.transition.get(&(i, *char)).unwrap() * rhs_clone.states
                            + *rhs_clone.transition.get(&(j, *char)).unwrap(),
                    );
                }
            }
        }
        result.optimize()
    }

    pub fn negation(&self) -> Self {
        let mut result = self.clone();
        result.accepting = result.accepting.iter().map(|a| !a).collect();
        result.optimize()
    }

    pub fn union(&self, rhs: &Self) -> Self {
        self.negation().intersect(&rhs.negation()).negation()
    }

    pub fn difference(&self, rhs: &Self) -> Self {
        self.intersect(&rhs.negation())
    }

    pub fn xor(&self, rhs: &Self) -> Self {
        self.union(&rhs).difference(&self.intersect(&rhs))
    }

    pub fn big_intersect(dfas: &[DFA]) -> Self {
        dfas.iter().fold(DFA::new(), |old, new| old.intersect(&new))
    }

    pub fn big_union(dfas: &[DFA]) -> Self {
        dfas.iter()
            .fold(DFA::new(), |old, new| old.intersect(&new.negation()))
            .negation()
    }

    pub fn remove_state(&self, state: i32, mut replacement: i32) -> Result<Self, ()> {
        if state >= self.states || replacement >= self.states || replacement == self.states {
            return Err(());
        }
        if state == 0 && replacement > 1 {
            return Err(());
        }
        if replacement > state {
            replacement -= 1;
        }
        let mut result = DFA {
            alphabet: self.alphabet.clone(),
            states: self.states - 1,
            accepting: vec![false; self.states as usize - 1],
            transition: HashMap::new(),
        };
        for i in 0..self.states {
            match i.cmp(&state) {
                Ordering::Less => result.accepting[i as usize] = self.accepting[i as usize],
                Ordering::Equal => continue,
                Ordering::Greater => result.accepting[i as usize - 1] = self.accepting[i as usize],
            }
            for &char in &self.alphabet {
                if i < state {
                    match self.transition.get(&(i, char)).unwrap().cmp(&state) {
                        Ordering::Less => result
                            .transition
                            .insert((i, char), *self.transition.get(&(i, char)).unwrap()),
                        Ordering::Equal => result.transition.insert((i, char), replacement),
                        Ordering::Greater => result
                            .transition
                            .insert((i, char), *self.transition.get(&(i, char)).unwrap() - 1),
                    };
                } else {
                    match self.transition.get(&(i, char)).unwrap().cmp(&state) {
                        Ordering::Less => result
                            .transition
                            .insert((i - 1, char), *self.transition.get(&(i, char)).unwrap()),
                        Ordering::Equal => result.transition.insert((i - 1, char), replacement),
                        Ordering::Greater => result
                            .transition
                            .insert((i - 1, char), *self.transition.get(&(i, char)).unwrap() - 1),
                    };
                }
            }
        }
        Ok(result)
    }

    pub fn states_reachable_from(&self, state: i32) -> Vec<i32> {
        let mut checked = vec![];
        let mut reached = vec![state];
        loop {
            let mut new_checks = false;
            let mut new_reached = vec![];
            for &state in &reached {
                if checked.contains(&state) {
                    continue;
                }
                checked.push(state);
                new_checks = true;
                for &char in &self.alphabet {
                    if !reached.contains(self.transition.get(&(state, char)).unwrap()) {
                        new_reached.push(*self.transition.get(&(state, char)).unwrap());
                    }
                }
            }
            reached.extend(new_reached);
            if !new_checks {
                break;
            }
        }
        reached.sort();
        reached
    }

    pub fn get_unreachable_states(&self) -> Vec<i32> {
        let reached = self.states_reachable_from(0);
        let mut result: Vec<i32> = (0..self.states).filter(|i| !reached.contains(i)).collect();
        result.reverse();
        result
    }

    fn is_permanently(&self, state: i32, accept: bool) -> bool {
        let states = self.states_reachable_from(state);
        for i in states {
            if self.accepting[i as usize] ^ accept {
                return false;
            }
        }
        true
    }

    pub fn is_permanently_accepting(&self, state: i32) -> bool {
        self.is_permanently(state, true)
    }

    pub fn is_permanently_rejecting(&self, state: i32) -> bool {
        self.is_permanently(state, false)
    }

    fn reduce_like_states(&self, accept: bool) -> Self {
        let mut result = self.clone();
        let mut permanent: Vec<i32> = (0..result.states)
            .filter(|i| result.is_permanently(*i, accept))
            .collect();
        permanent.reverse();
        if permanent.len() > 1 {
            for i in 0..permanent.len() - 1 {
                result = result
                    .remove_state(permanent[i as usize], permanent[i as usize + 1])
                    .unwrap();
            }
        }
        result
    }

    pub fn reduce_accepting_states(&self) -> Self {
        self.reduce_like_states(true)
    }

    pub fn reduce_rejecting_states(&self) -> Self {
        self.reduce_like_states(false)
    }

    pub fn remove_unreachable_states(&self) -> Self {
        // let mut result = self.clone();
        // for i in self.get_unreachable_states() {
        //     result = result.remove_state(i, 0).unwrap();
        // }
        // result
        self.get_unreachable_states()
            .iter()
            .fold(self.clone(), |dfa, state| dfa.remove_state(*state, 0).unwrap())
    }

    pub fn remove_indistinguishable_states(&self) -> Self {
        let mut result = self.clone();
        let mut highest = result.states;
        'i: loop {
            for i in (0..highest).rev() {
                for j in (0..i).rev() {
                    if result.states_indistinguishable(i, j) {
                        result = result.remove_state(i, j).unwrap();
                        highest = i - 1;
                        continue 'i;
                    }
                }
            }
            break 'i;
        }
        result
    }

    pub fn states_indistinguishable(&self, i: i32, j: i32) -> bool {
        if self.accepting[i as usize] != self.accepting[j as usize] {
            return false;
        }
        for &char in &self.alphabet {
            if self.transition.get(&(i, char)) == self.transition.get(&(j, char))
                || self.transition.get(&(i, char)) == Some(&i) && self.transition.get(&(j, char)) == Some(&j)
                || self.transition.get(&(i, char)) == Some(&j) && self.transition.get(&(j, char)) == Some(&i)
            {
                continue;
            }
            return false;
        }
        true
    }

    pub fn optimize(&self) -> Self {
        self.remove_unreachable_states()
            .reduce_accepting_states()
            .reduce_rejecting_states()
            .remove_indistinguishable_states()
    }
}

impl Display for DFA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
