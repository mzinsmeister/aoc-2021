use std::cmp::min;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<u32> = input_str.split_once("\n").unwrap().0
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut min_cost = u64::MAX;
    for pos in min..=max {
        let cost: u64 = input.iter()
            .map(|&e| (e as i64 - pos as i64).abs() as u64)
            .map(|e| ((e * e) + e) / 2) // Remove this line for P1
            .sum();
        min_cost = std::cmp::min(cost, min_cost);
    }
    println!("{}", min_cost);
}
