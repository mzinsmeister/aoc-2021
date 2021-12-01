use std::collections::LinkedList;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input_parsed = parse_input_string(&input);
    /* 1: let mut prev = input_parsed[0];
    let mut result = 0;
    for val in input_parsed.iter().skip(1) {
        if *val > prev {
            result += 1;
        }
        prev = *val;
    }
    println!("{}", result)*/

    let mut curr: LinkedList<u32> = input_parsed.iter().take(3).map(|i| *i).collect();
    let mut result = 0;
    for val in input_parsed.iter().skip(3) {
        let prev: u32 = curr.iter().sum();
        curr.pop_front();
        curr.push_back(*val);
        let new: u32 = curr.iter().sum();
        if new > prev {
            result += 1;
        }
    }
    println!("{}", result);
}

fn parse_input_string(input: &str) -> Vec<u32> {
    input.split("\n").filter(|i| !i.is_empty()).map(|i| i.parse().unwrap()).collect()
}
