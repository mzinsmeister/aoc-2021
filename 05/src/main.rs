use std::cmp::max;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input = parse(&input_str);
    /* P1: let input_use: Vec<Line> = input.iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .map(|&e| e)
        .collect(); */
    let input_use = input;
    let xmax = input_use.iter().map(|l| max(l.start.x, l.end.x)).max().unwrap();
    let ymax = input_use.iter().map(|l| max(l.start.y, l.end.y)).max().unwrap();
    let mut field: Vec<Vec<u32>> = Vec::with_capacity((ymax + 1) as usize);
    for i in 0..=ymax {
        let mut line = Vec::with_capacity((xmax + 1) as usize);
        for j in 0..=xmax {
            line.push(0);
        }
        field.push(line);
    }
    for line in input_use {
        let mut x = line.start.x;
        let mut y = line.start.y;
        let xstep = (line.end.x as i32 - line.start.x as i32).signum();
        let ystep = (line.end.y as i32 - line.start.y as i32).signum();
        while y != line.end.y || x != line.end.x {
            field[y as usize][x as usize] += 1;
            x = (x as i32 + xstep) as u32;
            y = (y as i32 + ystep) as u32;
        }
        field[y as usize][x as usize] += 1;
    }
    /*for line in field.iter() {
        for item in line.iter() {
            print!("{}", item);
        }
        print!("\n");
    }*/
    let result = field.iter().flatten().filter(|&&f| f >= 2).count();
    println!("{}", result);
}

#[derive(Copy, Clone)]
struct Line {
    start: Point2D,
    end: Point2D
}

#[derive(Copy,Clone)]
struct Point2D {
    x: u32,
    y: u32
}

fn parse(input: &str) -> Vec<Line> {
    input.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Line {
    let (left, right) = line.split_once(" -> ").unwrap();
    let (x1, y1) = left.split_once(",").unwrap();
    let (x2, y2) = right.split_once(",").unwrap();
    Line { start: Point2D { x: x1.parse().unwrap(), y: y1.parse().unwrap() },
        end: Point2D { x: x2.parse().unwrap(), y: y2.parse().unwrap() } }
}
