use std::collections::{BTreeMap, BTreeSet, LinkedList};
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let adjacencies: Vec<(&str, &str)> = input_str.split("\n")
        .filter(|l| !l.is_empty()).map(|l| l.split_once("-").unwrap())
        .collect();

    let mut adj_map: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (from, to) in adjacencies {
        if !adj_map.contains_key(from) {
            adj_map.insert(from, BTreeSet::new());
        }
        adj_map.get_mut(from).unwrap().insert(to);
        if !adj_map.contains_key(to) {
            adj_map.insert(to, BTreeSet::new());
        }
        adj_map.get_mut(to).unwrap().insert(from);
    }


    let paths = get_possible_paths(&adj_map, Vec::new(), "start");
    println!("{}", paths.len());

    let paths2 = get_possible_paths_2(&adj_map, Vec::new(), false, "start");
    println!("{}", paths2.len());
}

fn get_possible_paths<'a>(adj_map: &'a BTreeMap<&str, BTreeSet<&str>>, path: Vec<&'a str>, from: &'a str) -> Vec<Vec<&'a str>> {
    let current_path: Vec<&str> = path.iter().chain(std::iter::once(&from)).map(|s| *s).collect();
    if from == "end" {
        return vec![current_path];
    }
    let mut paths: Vec<Vec<&str>> = Vec::new();
    for node in adj_map[from].iter() {
        if node.chars().all(|c| c.is_uppercase()) || !path.contains(node) {
            let mut paths_new = get_possible_paths(adj_map, current_path.clone(), node);
            paths.append(&mut paths_new)
        }
    }
    paths
}

fn get_possible_paths_2<'a>(adj_map: &'a BTreeMap<&str, BTreeSet<&str>>, path: Vec<&'a str>, visited_twice: bool, from: &'a str) -> Vec<Vec<&'a str>> {
    let current_path: Vec<&str> = path.iter().chain(std::iter::once(&from)).map(|s| *s).collect();
    if from == "end" {
        return vec![current_path];
    }
    let mut paths: Vec<Vec<&str>> = Vec::new();
    for node in adj_map[from].iter() {
        if node.chars().all(|c| c.is_uppercase()) || !path.contains(node) {
            let mut paths_new = get_possible_paths_2(adj_map, current_path.clone(), visited_twice, node);
            paths.append(&mut paths_new)
        } else if !visited_twice && *node != "start" {
            let mut paths_new = get_possible_paths_2(adj_map, current_path.clone(), true, node);
            paths.append(&mut paths_new)
        }
    }
    paths
}
