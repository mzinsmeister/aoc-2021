use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let result: u32 = input_str.split("\n")
        .map(|l| part1_line_score(l))
        .sum();
    println!("{}", result);
    let mut scores2: Vec<u64> = input_str.split("\n")
        .map(|l| part2_line_score(l))
        .filter(|&s| s > 0)
        .collect();
    scores2.sort();
    let result2 = scores2[scores2.len() / 2];
    println!("{}", result2);
}

fn part1_line_score(line: &str) -> u32 {
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            let opening = stack.pop().unwrap();
            if !is_matching(opening, c) {
               return match c {
                   ')' => 3,
                   ']' => 57,
                   '}' => 1197,
                   '>' => 25137,
                   _ => panic!("unexpected char {}", c)
               }
            }
        }
    }
    0
}

fn part2_line_score(line: &str) -> u64 {
    let mut stack = Vec::new();
    let mut score: u64 = 0;
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            let opening = stack.pop().unwrap();
            if !is_matching(opening, c) {
                return 0;
            }
        }
    }
    while let Some(opening) = stack.pop() {
        score *= 5;
        score +=  match opening {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("unknown opening char: {}", opening)
        };
    }
    score
}

fn is_opening(c: char) -> bool {
    c == '(' || c == '<' || c == '{' || c == '['
}

fn is_matching(opening: char, closing: char) -> bool {
    closing == get_closing_for(opening)
}

fn get_closing_for(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unknown opening char: {}", opening)
    }
}
