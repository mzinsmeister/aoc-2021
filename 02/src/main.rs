use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input = parse_input(&input_str);
    /* P1: let (depth, horizontal) = input.iter()
        .fold((0u32, 0u32), |(d, h), MoveCommand(dir, amount)|
            match dir {
                MoveDirection::FORWARD => (d, h + amount),
                MoveDirection::UP => (d - amount, h),
                MoveDirection::DOWN => (d + amount, h)
            });*/
    let (depth, horizontal, _) = input.iter()
        .fold((0i32, 0i32, 0i32), |(d, h, a), MoveCommand(dir, amount)|
            match dir {
                MoveDirection::FORWARD => (d + (a * *amount as i32), h + *amount as i32, a),
                MoveDirection::UP => (d, h, a - *amount as i32),
                MoveDirection::DOWN => (d, h, a + *amount as i32)
            }
        );
    println!("{}, {}", depth, horizontal);
    println!("{}", depth * horizontal);
}

enum MoveDirection {
    FORWARD,
    UP,
    DOWN
}

struct MoveCommand(MoveDirection, u32);

fn parse_input(input: &str) -> Vec<MoveCommand> {
    input.split("\n")
        .filter(|i| !i.is_empty())
        .map(|i| parse_line(i))
        .collect()
}

fn parse_line(input: &str) -> MoveCommand {
    let mut iter = input.split(" ");
    let (direction_str, amount_str) = (iter.next().unwrap(), iter.next().unwrap());
    let direction = match direction_str {
        "forward" => MoveDirection::FORWARD,
        "up" => MoveDirection::UP,
        "down" => MoveDirection::DOWN,
        _ => panic!("unknown command")
    };
    let amount: u32 = amount_str.parse().unwrap();
    MoveCommand(direction, amount)
}
