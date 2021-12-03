use std::collections::{HashSet, LinkedList};
use std::fs::read_to_string;
use std::ops::Not;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<Vec<char>> = input_str.split("\n").filter(|l| !l.is_empty()).map(|i| i.chars().collect()).collect();
    let inputlen = input.len();
    let mut remainder: HashSet<&Vec<char>> = HashSet::from_iter(input.iter());
    let mut pos = 0usize;
    while remainder.len() > 1 {
        let amt = remainder.iter().filter(|l| l[pos] == '1').count();
        let more = if amt >= (remainder.len() / 2) + remainder.len() % 2 { '1' } else { '0' };
        remainder.retain(|e| e[pos] == more);
        pos += 1;
    }
    let ox_str: String = remainder.iter().next().unwrap().iter().collect();
    let mut remainder: HashSet<&Vec<char>> = HashSet::from_iter(input.iter());
    let mut pos = 0usize;
    while remainder.len() > 1 {
        let amt = remainder.iter().filter(|l| l[pos] == '1').count();
        let less = if amt < (remainder.len() / 2) + (remainder.len() % 2) { '1' } else { '0' };
        remainder.retain(|e| e[pos] == less);
        pos += 1;
    }
    let co2_str: String = remainder.iter().next().unwrap().iter().collect();

    let ox: u32 = u32::from_str_radix(&ox_str, 2).unwrap();
    let co2: u32 = u32::from_str_radix(&co2_str, 2).unwrap();
    let result = ox * co2;

    /* P1: let mut pos_count: Vec<u32> = Vec::with_capacity(input[0].len());
    let linelen = input[0].len();
    for _ in 0..input[0].len() {
        pos_count.push(0u32);
    }
    for line in input {
        for (i, &c) in line.iter().enumerate() {
            if c == '1' {
                pos_count[i] += 1;
            }
        }
    }
    let mut gamma = 0u32;
    for &pos in pos_count.iter() {
        gamma <<= 1;
        if pos > (inputlen as u32 / 2) {
            gamma |= 1
        }
    }
    let epsilon = !gamma & (u32::MAX >> (32 - linelen));
    let result = epsilon * gamma;*/

    println!("{}", result);
}
