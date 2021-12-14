use std::collections::BTreeMap;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let mut input: Vec<u32> = input_str.split_once("\n").unwrap().0
        .split(",").map(|e| e.parse().unwrap())
        .collect();
    let mut ages: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    input.iter().for_each(|&a| ages[a as usize] += 1);
    // P1: for _ in 0..80 {
    for _ in 0..256 {
        let new = ages[0];
        for i in 0..8 {
            ages[i] = ages[i+1];
        }
        ages[8] = new;
        ages[6] += new;
    }
    let result: u64 = ages.iter().sum();
    println!("{}", result);
}
