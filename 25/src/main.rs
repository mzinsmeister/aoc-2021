use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<Vec<Spot>> = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| match c {
            '.' => Spot::EMPTY,
            '>' => Spot::EAST,
            'v' => Spot::SOUTH,
            _ => panic!("unknown char {}", c)
        }).collect())
        .collect();

    let mut current = input.clone();
    let mut moved = true;
    let mut rounds = 0;
    while moved {
        let mut new_map = current.clone();
        moved = false;
        rounds += 1;
        /*for y in 0..input.len() {
            for x in 0..input[y].len() {
                print!("{}", current[y][x]);
            }
            println!();
        }
        println!();*/
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if current[y][x] == Spot::EAST
                    && current[y][(x + 1) % input[y].len()] == Spot::EMPTY {
                    new_map[y][x] = Spot::EMPTY;
                    new_map[y][(x + 1) % input[y].len()] = Spot::EAST;
                    moved = true;
                    //println!("moving {},{} to {},{}", x, y, (x + 1) % input[y].len(), y)
                }
            }
        }
        current = new_map;
        let mut new_map = current.clone();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if current[y][x] == Spot::SOUTH
                    && current[(y + 1) % input.len()][x] == Spot::EMPTY {
                    new_map[y][x] = Spot::EMPTY;
                    new_map[(y + 1) % input.len()][x] = Spot::SOUTH;
                    moved = true;
                    //println!("moving {},{} to {},{}", x, y, x, (y + 1) % input.len())
                }
            }
        }
        current = new_map;
    }
    println!("{}", rounds);
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Spot {
    EMPTY, SOUTH, EAST
}

impl Display for Spot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Spot::EMPTY => ".",
            Spot::SOUTH => "v",
            Spot::EAST => ">"
        })
    }
}