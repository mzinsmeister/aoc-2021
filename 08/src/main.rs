use std::collections::{BTreeMap, BTreeSet};
use std::collections::hash_set::Union;
use std::fs::read_to_string;
use std::ops::Sub;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let p2_result: u32 = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_line)
        //.inspect(|e| println!("{:#?}", e.result))
        .map(|l| deduct(l))
        .sum();
    println!("{}", p2_result);
}

struct Line {
    patterns: Vec<BTreeSet<char>>,
    result: Vec<BTreeSet<char>>
}

fn parse_line(line: &str) -> Line {
    let (left, right) = line.split_once(" | ").unwrap();
    let patterns = left.split(" ").map(|e| e.chars().collect()).collect();
    let result = right.split(" ").map(|e| e.chars().collect()).collect();
    Line {
        patterns,
        result,
    }
}


fn deduct(line: Line) -> u32 {
    let mut patternsleft: BTreeSet<BTreeSet<char>> = BTreeSet::from_iter(line.patterns.iter().map(|e| e.to_owned()));
    let mut numbersfound: BTreeMap<BTreeSet<char>, u32> = BTreeMap::new();
    let mut charsfound: BTreeMap<char, char> = BTreeMap::new();
    let pat1 = line.patterns.iter().find(|l| l.len() == 2).unwrap();
    numbersfound.insert(pat1.to_owned(), 1);
    let pat7 = line.patterns.iter().find(|l| l.len() == 3).unwrap();
    numbersfound.insert(pat7.to_owned(), 7);
    let pat4 = line.patterns.iter().find(|l| l.len() == 4).unwrap();
    numbersfound.insert(pat4.to_owned(), 4);
    let pat8 = line.patterns.iter().find(|l| l.len() == 7).unwrap();
    numbersfound.insert(pat8.to_owned(), 8);

    patternsleft.remove(pat1);
    patternsleft.remove(pat7);
    patternsleft.remove(pat4);
    patternsleft.remove(pat8);

    let a = (pat7 - pat1).iter().next().unwrap().to_owned();
    charsfound.insert(a,'a');

    let g: char = patternsleft.iter()
        .filter(|l| l.len() == 6)
        .map(|m| {
            let t1 = m.sub(pat7);
            &t1 - pat4
        })
        .filter(|f| f.len() == 1)
        .next().unwrap().iter().next().unwrap().to_owned();
    charsfound.insert(g, 'g');
    let mut pat9 = pat4.clone();
    pat9.extend(pat7.iter());
    pat9.insert(g);
    patternsleft.remove(&pat9);
    numbersfound.insert(pat9.to_owned(), 9);

    let e = (pat8 - &pat9).iter().next().unwrap().to_owned();
    charsfound.insert(e,'e');

    let pat6 = patternsleft.iter()
        .filter(|l| l.len() == 6 && l.intersection(pat1).count() == 1).next().unwrap().to_owned();
    numbersfound.insert(pat6.to_owned(), 6);
    patternsleft.remove(&pat6);

    let c = (pat8 - &pat6).iter().next().unwrap().to_owned();

    charsfound.insert(c, 'c');

    let f = (pat1 - &BTreeSet::from([c])).iter().next().unwrap().to_owned();

    charsfound.insert(f, 'f');

    let pat0 = patternsleft.iter()
        .filter(|l| l.len() == 6).next().unwrap().to_owned();
    patternsleft.remove(&pat0);
    numbersfound.insert(pat0.to_owned(), 0);

    let t1 = &pat0 - pat7;
    let b = (&t1 - &BTreeSet::from([g, e])).iter().next().unwrap().to_owned();

    charsfound.insert(b, 'b');

    let d = (pat4 - &BTreeSet::from([c, f, b])).iter().next().unwrap().to_owned();

    charsfound.insert(d, 'd');

    numbersfound.insert(BTreeSet::from([a, c, d, e, g]), 2);
    numbersfound.insert(BTreeSet::from([a, c, d, f, g]), 3);
    numbersfound.insert(BTreeSet::from([a, b, d, f, g]), 5);

    let num = 1000 * numbersfound[&line.result[0]] + 100 * numbersfound[&line.result[1]] + 10 * numbersfound[&line.result[2]] + numbersfound[&line.result[3]];

    return num
}

fn part_1(input_str: &str) {
    let p1_result: usize = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_line)
        //.inspect(|e| println!("{:#?}", e.result))
        .map(|l| l.result.iter()
            .filter(|e| (e.len() >= 2 && e.len() <= 4) || e.len() == 7).count())
        .sum();
    println!("{}", p1_result);
}