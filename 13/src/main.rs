use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let (dots_input_str, folds_input_str) = input_str.split_once("\n\n").unwrap();
    let dots: HashSet<(u32, u32)> = dots_input_str.split("\n")
        .map(parse_dot)
        .collect();
    let folds: Vec<Fold> = folds_input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_fold)
        .collect();

    let results1: HashSet<(u32, u32)> = dots.iter()
        .map(|&d| apply_fold(folds[0], d))
        .collect();

    println!("{}", results1.len());

    let mut current_dots = dots;

    for fold in folds {
        plot(current_dots.clone());
        println!();
        current_dots = current_dots.iter()
            .map(|&d| apply_fold(fold, d))
            .collect();
    }
    plot(current_dots);
}

#[derive(Copy, Clone)]
enum FoldDir {
    X, Y
}

#[derive(Copy, Clone)]
struct Fold{ dir: FoldDir, pos: u32 }

fn parse_dot(line: &str) -> (u32, u32) {
    let (x_str, y_str) = line.split_once(",").unwrap();
    (x_str.parse().unwrap(), y_str.parse().unwrap())
}

fn parse_fold(line: &str) -> Fold {
    let stripped = line.strip_prefix("fold along ").unwrap();
    let (dir_str, pos_str) = stripped.split_once("=").unwrap();
    let dir = match dir_str {
        "x" => FoldDir::X,
        "y" => FoldDir::Y,
        _ => panic!("unknown dir {}", dir_str)
    };
    let pos = pos_str.parse().unwrap();

    Fold{ dir, pos }
}

fn apply_fold(fold: Fold, (x, y): (u32, u32)) -> (u32, u32) {
    match fold.dir {
        FoldDir::X => {
            if x > fold.pos {
                (((-(x as i32)).rem_euclid(fold.pos as i32)) as u32, y)
            } else {
                (x, y)
            }
        }
        FoldDir::Y => {
            if y > fold.pos {
                (x, ((-(y as i32)).rem_euclid(fold.pos as i32)) as u32)
            } else {
                (x, y)
            }
        }
    }
}

fn plot(points: HashSet<(u32, u32)>) {
    let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y) ) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
