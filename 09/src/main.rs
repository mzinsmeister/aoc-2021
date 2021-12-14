use std::collections::BTreeSet;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<Vec<u8>> = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    let height = input.len();
    let width = input[0].len();
    let mut lowpoints: Vec<(usize, usize, u8)> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let curr = input[i][j];
            let mut lowest = true;
            if i > 0 {
                lowest &= curr < input[i-1][j];
            }
            if j > 0 {
                lowest &= curr < input[i][j-1];
            }
            if j < width - 1{
                lowest &= curr < input[i][j+1];
            }
            if i < height - 1{
                lowest &= curr < input[i+1][j];
            }

            if lowest {
                lowpoints.push((i, j, curr));
            }
        }
    }
    let result1: u32 = lowpoints.iter().map(|&(_, _, h)| h as u32 + 1).sum();
    println!("{}", result1);

    let mut basins: BTreeSet<BTreeSet<(usize, usize)>> = BTreeSet::new();
    for (i, j, _) in lowpoints {
        let mut basin = BTreeSet::from([(i, j)]);
        let mut last_p = BTreeSet::from([(i, j)]);
        let mut oldsize= 0;
        while oldsize < basin.len() {
            oldsize = basin.len();
            let mut new_p: BTreeSet<(usize, usize)> = BTreeSet::new();
            for (it, jt) in last_p.iter() {
                if *it > 0 {
                    if !basin.contains(&(it - 1, *jt))
                    && input[it - 1][*jt] < 9 && input[it - 1][*jt] > input[*it][*jt] {
                    basin.insert((it - 1, *jt));
                    new_p.insert((it - 1, *jt));
                }}
                if *jt > 0 {
                    if !basin.contains(&(*it, jt - 1))
                    && input[*it][jt - 1] < 9 && input[*it][jt - 1] > input[*it][*jt] {
                    basin.insert((*it, jt - 1));
                    new_p.insert((*it, jt - 1));
                }}
                if *it < height - 1 {
                    if !basin.contains(&(it + 1, *jt))
                    && input[it + 1][*jt] < 9 && input[it + 1][*jt] > input[*it][*jt] {
                    basin.insert((it + 1, *jt));
                    new_p.insert((it + 1, *jt));
                }}
                if *jt < width - 1 {
                    if !basin.contains(&(*it, jt + 1))
                    && input[*it][jt + 1] < 9 && input[*it][jt + 1] > input[*it][*jt] {
                    basin.insert((*it, jt + 1));
                    new_p.insert((*it, jt + 1));
                }}
            }
            last_p = new_p;
        }
        basins.insert(basin);
    }

    let mut basin_vec = Vec::from_iter(basins.iter());
    basin_vec.sort_by(|a, b| b.len().partial_cmp(&(a.len())).unwrap());


    let result = basin_vec.iter().take(3)
        .map(|b| b.len())
        .fold(1usize, |a, e| a * e);
    println!("{}", result);
}
