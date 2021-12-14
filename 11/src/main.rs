use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<Vec<u32>> = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect())
        .collect();

    let mut map: Vec<Vec<u32>> = input.iter().map(|l| l.clone()).collect();
    let mut round = 0;
    loop {
        round += 1;
        map.iter_mut().flatten().for_each(|n| *n += 1);
        let mut flashedps: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut flashed = true;
        while flashed {
            flashed = false;

            for i in 0..map.len() {
                let linelen = map[i].len();
                for j in 0..linelen {
                    let num = map[i][j];
                    if num > 9 && !flashedps.contains(&(i, j)) {
                        flashedps.insert((i, j));
                        flashed = true;
                        for di in -1..=1 {
                            for dj in -1..=1 {
                                let inew = i as i32 + di;
                                let jnew = j as i32 + dj;
                                if inew >= 0 && jnew >= 0 && inew < map.len() as i32 && jnew < linelen as i32 {
                                    map[inew as usize][jnew as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        map.iter_mut().flatten().filter(|n| **n > 9).for_each(|n| *n = 0);
        if flashedps.len() == 100 {
            println!("{}", round);
            return;
        }
        /*for line in map.iter() {
            for n in line.iter() {
                print!("{}", n);
            }
            print!("\n");
        }
        println!();*/
    }
}

fn part1(input_str: &str) {
    let input: Vec<Vec<u32>> = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect())
        .collect();

    let mut map: Vec<Vec<u32>> = input.iter().map(|l| l.clone()).collect();
    let mut flashes = 0;
    for _ in 0..100 {
        map.iter_mut().flatten().for_each(|n| *n += 1);
        let mut flashedps: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut flashed = true;
        while flashed {
            flashed = false;

            for i in 0..map.len() {
                let linelen = map[i].len();
                for j in 0..linelen {
                    let num = map[i][j];
                    if num > 9 && !flashedps.contains(&(i, j)) {
                        flashedps.insert((i, j));
                        flashed = true;
                        for di in -1..=1 {
                            for dj in -1..=1 {
                                let inew = i as i32 + di;
                                let jnew = j as i32 + dj;
                                if inew >= 0 && jnew >= 0 && inew < map.len() as i32 && jnew < linelen as i32 {
                                    map[inew as usize][jnew as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        map.iter_mut().flatten().filter(|n| **n > 9).for_each(|n| *n = 0);
        flashes += flashedps.len();
        /*for line in map.iter() {
            for n in line.iter() {
                print!("{}", n);
            }
            print!("\n");
        }
        println!();*/
    }
    println!("{}", flashes);
}
