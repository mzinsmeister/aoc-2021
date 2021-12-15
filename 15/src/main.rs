use std::arch::x86_64::_addcarry_u32;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeSet};
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<Vec<u32>> = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect())
        .collect();
    //let (route, score) = find_min_route(&input, &vec![(0, 0)], input[0][0]).unwrap();
    let score = dijkstra(&input);
    println!("{}", score);
    let mut input2: Vec<Vec<u32>> = Vec::with_capacity(input.len() * 5);
    for _ in 0..input.len() * 5 {
        input2.push(Vec::with_capacity(input[0].len() * 5));
    }
    for i in 0..5 {
        for j in 0..5 {
            input.iter()
                .map(|l| l.iter().map(|n| ((n + j + i - 1) % 9) + 1))
                .enumerate()
                .for_each(|(k, v)| input2[(i as usize * input.len()) + k].extend(v));
        }
    }
    let score = dijkstra(&input2);
    println!("{}", score);
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(field: &Vec<Vec<u32>>) -> u32 {
    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    let mut distances: Vec<Vec<u32>> = Vec::with_capacity(field.len());
    for _ in 0..field.len() {
        distances.push((0..field[0].len()).into_iter().map(|_| u32::MAX).collect());
    }
    distances[0][0] = 0;
    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(field.len());
    for _ in 0..field.len() {
        visited.push((0..field[0].len()).into_iter().map(|_| false).collect());
    }
    open_set.push(State { position: (0, 0), cost: 0 });
    while let Some(State { cost, position: current }) = open_set.pop() {
        if current.0 == field[0].len() - 1 && current.1 == field.len() - 1 {
            return distances[current.1][current.0];
        }

        if current.0 > 0 {
            let p = (current.0 - 1, current.1);
            if !visited[p.1][p.0] {
                let new_d = distances[current.1][current.0] + field[p.1][p.0];
                if new_d < distances[p.1][p.0] {
                    distances[p.1][p.0] = new_d;
                    open_set.push(State { position: p, cost: new_d })
                }
            }
        }
        if current.0 < (field[0].len() - 1) {
            let p = (current.0 + 1, current.1);
            if !visited[p.1][p.0] {
                let new_d = distances[current.1][current.0] + field[p.1][p.0];
                if new_d < distances[p.1][p.0] {
                    distances[p.1][p.0] = new_d;
                    open_set.push(State { position: p, cost: new_d })
                }
            }
        }
        if current.1 > 0 {
            let p = (current.0, current.1 - 1);
            if !visited[p.1][p.0] {
                let new_d = distances[current.1][current.0] + field[p.1][p.0];
                if new_d < distances[p.1][p.0] {
                    distances[p.1][p.0] = new_d;
                    open_set.push(State { position: p, cost: new_d })
                }
            }
        }
        if current.1 < (field.len() - 1) {
            let p = (current.0, current.1 + 1);
            if !visited[p.1][p.0] {
                let new_d = distances[current.1][current.0] + field[p.1][p.0];
                if new_d < distances[p.1][p.0] {
                    distances[p.1][p.0] = new_d;
                    open_set.push(State { position: p, cost: new_d })
                }
            }
        }
        visited[current.1][current.0] = true;
    }
    panic!()
}

fn find_min_route(field: &Vec<Vec<u32>>, current_path: &Vec<(usize, usize)>, current_score: u32) -> Option<(Vec<(usize, usize)>, u32)> {
    let (x, y) = *current_path.last().unwrap();
    if x == field[0].len() - 1 && y == field.len() - 1 {
        println!("{}", current_score);
        return Option::Some((current_path.to_owned(), current_score));
    }
    //println!("{}", current_path.len());
    let mut current_min: Option<(Vec<(usize, usize)>, u32)> = Option::None;
    if x > 0 {
        let p = (x - 1, y);
        current_min = try_new_path(field, current_path, p, current_score + field[p.1][p.0]);
    }
    let mut current_min: Option<(Vec<(usize, usize)>, u32)> = Option::None;
    if x < (field[0].len() - 1) {
        let p = (x + 1, y);
        let new_op = try_new_path(field, current_path, p, current_score + field[p.1][p.0]);
        if let Some(new) = new_op {
            if new.1 < current_min.as_ref().map_or_else(|| u32::MAX, |c| c.1) {
                current_min = Option::Some(new);
            }
        }
    }
    if y > 0 {
        let p = (x, y - 1);
        let new_op = try_new_path(field, current_path, p, current_score + field[p.1][p.0]);
        if let Some(new) = new_op {
            if new.1 < current_min.as_ref().map_or_else(|| u32::MAX, |c| c.1) {
                current_min = Option::Some(new);
            }
        }
    }
    if y < (field.len() - 1) {
        let p = (x, y + 1);
        let new_op = try_new_path(field, current_path, p, current_score + field[p.1][p.0]);
        if let Some(new) = new_op {
            if new.1 < current_min.as_ref().map_or_else(|| u32::MAX, |c| c.1) {
                current_min = Option::Some(new);
            }
        }
    }
    return current_min;
}

fn try_new_path(field: &Vec<Vec<u32>>, current_path: &Vec<(usize, usize)>,
                new_p: (usize, usize), current_score: u32) -> Option<(Vec<(usize, usize)>, u32)> {
    if !current_path.contains(&new_p) {
        let new_path = current_path.iter()
            .chain(std::iter::once(&new_p))
            .cloned()
            .collect();
        return find_min_route(field, &new_path, current_score);
    }
    Option::None
}