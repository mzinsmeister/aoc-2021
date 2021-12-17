use std::cmp::max;
use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input_str_stripped = input_str.strip_prefix("target area: ").unwrap().strip_suffix("\n").unwrap();
    let (x_range_input_str, y_range_input_str) =
        input_str_stripped.split_once(", ").unwrap();
    let (x_range_start_str, x_range_end_str) =
        x_range_input_str.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let (y_range_start_str, y_range_end_str) =
        y_range_input_str.strip_prefix("y=").unwrap().split_once("..").unwrap();
    let x_range_start: i64 = x_range_start_str.parse().unwrap();
    let x_range_end: i64 = x_range_end_str.parse().unwrap();
    let y_range_start: i64 = y_range_start_str.parse().unwrap();
    let y_range_end: i64 = y_range_end_str.parse().unwrap();
    let target = Target { x_range: (x_range_start, x_range_end), y_range: (y_range_start, y_range_end) };

    let r = simulate_until_unreachable(target, (6, 0));


    let mut max_y = 0;
    let mut total_hit = 0;
    for x_v in -500..500 {
        for y_v in -500..500 {
            if let Some(m) = simulate_until_unreachable(target, (x_v, y_v)) {
                total_hit += 1;
                max_y = max(max_y, m);
                //println!("{}, {}", x_v, y_v);
            }
        }
    }

    println!("{}", max_y);
    println!("{}", total_hit);
}

#[derive(Copy, Clone)]
struct Target {
    x_range: (i64, i64),
    y_range: (i64, i64)
}

impl Target {
    fn is_in(&self, p: (i64, i64)) -> bool {
        p.0 >= self.x_range.0 && p.0 <= self.x_range.1
            && p.1 >= self.y_range.0 && p.1 <= self.y_range.1
    }

    fn is_above(&self, p: (i64, i64)) -> bool {
        p.1 >= self.y_range.0
    }
}

struct State {
    pos: (i64, i64),
    velocity: (i64, i64)
}

impl State {

    fn simulate_step(&self) -> State {
        let pos = (self.pos.0 + self.velocity.0, self.pos.1 + self.velocity.1);
        let velocity = (self.velocity.0 - self.velocity.0.signum(), self.velocity.1 - 1);
        State { pos, velocity }
    }

}

fn simulate_until_unreachable(target: Target, velocity: (i64, i64)) -> Option<i64> {
    let mut current_state = State { pos: (0, 0), velocity };
    let mut max_y = 0;
    //println!("{},{}", velocity.0, velocity.1);
    while !target.is_in(current_state.pos) && (target.is_above(current_state.pos)
        || current_state.velocity.1 >= 0) {
        current_state = current_state.simulate_step();
        //println!("{}, {}", current_state.pos.0, current_state.pos.1);
        max_y = max(max_y, current_state.pos.1);
    }
    if target.is_in(current_state.pos) {
        Some(max_y)
    } else {
        None
    }
}