use std::collections::BTreeMap;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let (start, mappings_in) = input_str.split_once("\n\n").unwrap();
    let mappings: BTreeMap<(char, char), char> = mappings_in.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_mapping)
        .collect();
    let mut current_val: BTreeMap<(char, char), u64> = BTreeMap::new();
    let mut input_chars = start.chars();
    let mut last_char = input_chars.next().unwrap();
    for c in input_chars {
        *current_val.entry((last_char, c)).or_default() += 1;
        last_char = c;
    }
    current_val.insert((last_char, 'e'), 1);
    let mut current_val2 = current_val.clone();
    for _ in 0..10 {
        current_val = iteration(&current_val, &mappings);
    }
    for ((k1, k2), v) in current_val.iter() {
        println!("{}{}: {}", k1, k2, v);
    }
    println!("{}", score(&current_val));

    for _ in 0..40 {
        current_val2 = iteration(&current_val2, &mappings);
    }

    println!("{}", score(&current_val2));
}

fn parse_mapping(l: &str) -> ((char, char), char) {
    let (from, to) = l.split_once(" -> ").unwrap();
    let mut chars = from.chars();
    ((chars.next().unwrap(), chars.next().unwrap()), to.chars().next().unwrap())
}

fn iteration(input: &BTreeMap<(char, char), u64>, mappings: &BTreeMap<(char, char), char>) -> BTreeMap<(char, char), u64> {
    let mut result: BTreeMap<(char, char), u64> = BTreeMap::new();

    for (cs, count) in input {
        if let Some(cin) = mappings.get(cs) {
            *result.entry((cs.0, *cin)).or_default() += count;
            *result.entry((*cin, cs.1)).or_default() += count;
        } else {
            *result.entry(*cs).or_default() += count;
        }
    }
    result
}

fn score(result: &BTreeMap<(char, char), u64>) -> u64 {
    let mut counts: BTreeMap<char, u64> = BTreeMap::new();

    for ((c, _), v) in result.iter() {
        if !counts.contains_key(c) {
            counts.insert(*c, 0);
        }
        *counts.get_mut(c).unwrap() += v;
    }
    let pairs: Vec<(char, u64)> = counts.iter().map(|e| (*e.0, *e.1)).collect();
    let max = pairs.iter().max_by(|(_, i), (_, i2)| i.partial_cmp(i2).unwrap()).unwrap();
    let min = pairs.iter().min_by(|(_, i), (_, i2)| i.partial_cmp(i2).unwrap()).unwrap();
    max.1 - min.1
}