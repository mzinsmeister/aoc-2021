use std::collections::BTreeMap;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let (start, mappings_in) = input_str.split_once("\n\n").unwrap();
    let mappings: BTreeMap<(char, char), char> = mappings_in.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_mapping)
        .collect();
    let mut current_val = start.to_owned();
    for _ in 0..10 {
        current_val = iteration(&current_val, &mappings);
    }
    println!("{}", score(&current_val));

    let mut current_val2 = start.to_owned();
    for _ in 0..30 {
        current_val2 = iteration(&current_val2, &mappings);
    }

    println!("{}", score(&current_val2));
}

fn parse_mapping(l: &str) -> ((char, char), char) {
    let (from, to) = l.split_once(" -> ").unwrap();
    let mut chars = from.chars();
    ((chars.next().unwrap(), chars.next().unwrap()), to.chars().next().unwrap())
}

fn iteration(input: &str, mappings: &BTreeMap<(char, char), char>) -> String {
    let mut changes: Vec<(usize, char)> = Vec::new();
    let mut input_chars = input.chars();
    let mut last_char = input_chars.next().unwrap();
    for (i, c) in input_chars.enumerate() {
        if let Some(cin) = mappings.get(&(last_char, c)) {
            changes.push((i + 1, *cin));
        }
        last_char = c;
    }
    let mut next_insert = 0usize;
    let mut result = String::with_capacity(input.len() + changes.len());
    for (pos, c) in changes {
        result.push_str(&input[next_insert..pos]);
        next_insert = pos;
        result.push(c);
    }
    result.push_str(&input[next_insert..]);
    result
}

fn score(s: &str) -> u64 {
    let mut counts: BTreeMap<char, u64> = BTreeMap::new();

    for c in s.chars() {
        if !counts.contains_key(&c) {
            counts.insert(c, 0);
        }
        *counts.get_mut(&c).unwrap() += 1;
    }
    let pairs: Vec<(char, u64)> = counts.iter().map(|e| (*e.0, *e.1)).collect();
    let max = pairs.iter().max_by(|(_, i), (_, i2)| i.partial_cmp(i2).unwrap()).unwrap();
    let min = pairs.iter().min_by(|(_, i), (_, i2)| i.partial_cmp(i2).unwrap()).unwrap();
    max.1 - min.1
}