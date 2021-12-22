use std::fs::read_to_string;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let input_str = read_to_string("input.txt").unwrap();
    let (algo_in, data_in) = input_str.split_once("\n\n").unwrap();
    let algo: Vec<char> = algo_in.chars().collect();
    let data: Vec<Vec<char>> = data_in.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let result1 = enhance_n_times(&data, &algo, 2);
    println!("{}", result1.iter().flatten().filter(|&&c| c == '#').count());

    println!("in {}ms", SystemTime::elapsed(&start_time).unwrap().as_millis());

    let result2 = enhance_n_times(&data, &algo, 50);
    println!("{}", result2.iter().flatten().filter(|&&c| c == '#').count());

    println!("in {}ms", SystemTime::elapsed(&start_time).unwrap().as_millis());
}

fn enhance_n_times(data: &Vec<Vec<char>>, algo: &Vec<char>, n: usize) -> Vec<Vec<char>> {
    let mut current = data.clone();
    let mut default = '.';
    for _ in 0..n { // P1: 2
        current = enhance(&current, &algo, default);
        if algo[0] == '#' && algo[511] == '.' {
            if default == '.' {
                default = '#';
            } else {
                default = '.'
            }
        }
    }
    current
}


fn enhance(data: &Vec<Vec<char>>, algo: &Vec<char>, default: char) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::with_capacity(data.len() + 2);
    for y in -1..=data.len() as isize {
        let mut output_line: Vec<char> = Vec::new();
        for x in -1..=data[0].len() as isize {
            let c = algo[read_n(&data, x, y, default)];
            output_line.push(c);
        }
        output.push(output_line);
    }
    output
}

fn read_n(data: &Vec<Vec<char>>, x: isize, y: isize, default: char) -> usize {
    let mut chars: Vec<char> = Vec::with_capacity(9);
    chars.push(get_char(data, x - 1, y - 1, default));
    chars.push(get_char(data, x, y - 1, default));
    chars.push(get_char(data, x + 1, y - 1, default));
    chars.push(get_char(data, x - 1, y, default));
    chars.push(get_char(data, x, y, default));
    chars.push(get_char(data, x + 1, y, default));
    chars.push(get_char(data, x - 1, y + 1, default));
    chars.push(get_char(data, x, y + 1, default));
    chars.push(get_char(data, x + 1, y + 1, default));
    let mut result = 0;
    for c in chars.iter() {
        result <<= 1;
        if *c == '#' {
            result |= 1;
        }
    }
    let alternative_str = chars.iter().map(|c| if *c == '#' { '1' } else { '0' }).collect::<String>();
    let alternative = usize::from_str_radix(&alternative_str, 2).unwrap();
    assert_eq!(alternative, result);
    result
}

fn get_char(data: &Vec<Vec<char>>, x: isize, y: isize, default: char) -> char {
    if x < 0 || y < 0 || x >= data[0].len() as isize || y >= data.len() as isize {
        default
    } else {
        data[y as usize][x as usize]
    }
}
