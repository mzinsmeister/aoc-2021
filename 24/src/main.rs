use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

// It's not beautiful but it works (P2 rather fast

fn main() {
    let input_str = read_to_string("demo2.txt").unwrap();

    /* P1: let t1 = std::thread::spawn(|| {
        check_digits(1, 2);
    });

    let t2 = std::thread::spawn(|| {
        check_digits(3, 4);
    });

    let t3 = std::thread::spawn(|| {
        check_digits(5, 6);
    });

    let t4 = std::thread::spawn(|| {
        check_digits(7, 7);
    });

    let t5 = std::thread::spawn(|| {
        check_digits(8, 8);
    });

    let t6 = std::thread::spawn(|| {
        check_digits(9, 9);
    });*/

    let t1 = std::thread::spawn(|| {
        check_digits_2(1, 1);
    });

    let t2 = std::thread::spawn(|| {
        check_digits_2(2, 2);
    });

    let t3 = std::thread::spawn(|| {
        check_digits_2(3, 3);
    });

    let t4 = std::thread::spawn(|| {
        check_digits_2(4, 5);
    });

    let t5 = std::thread::spawn(|| {
        check_digits_2(6, 7);
    });

    let t6 = std::thread::spawn(|| {
        check_digits_2(8, 9);
    });

    t1.join();
    t2.join();
    t3.join();
    t4.join();
    t6.join();
    //let inputs: VecDeque<i64> = VecDeque::from([14]);

    //let result = interpret(&input_str, inputs);

    //println!("{}", result);
}

#[inline]
fn check_digits(d1_lower: i64, d1_upper: i64) {
    for d0 in (2..=9).rev() {
        let state = check_digit(ALUState::new(), d0, 12, 11, 1);
        for d1 in (d1_lower..=d1_upper).rev() {
            let state = check_digit(state, d1, 1, 11, 11);
            for d2 in (1..=9).rev() {
                let state = check_digit(state, d2, 1, 14, 1);
                for d3 in (1..=9).rev() {
                    let state = check_digit(state, d3, 1, 11, 11);
                    for d4 in (1..=9).rev() {
                        let state = check_digit(state, d4, 26, -8, 2);
                        println!("{}{}{}{}{}...", d0, d1, d2, d3, d4);
                        for d5 in (1..=9).rev() {
                            let state = check_digit(state, d5, 26, -5, 9);
                            for d6 in (1..=9).rev() {
                                let state = check_digit(state, d6, 1, 11, 7);
                                for d7 in (1..=9).rev() {
                                    let state = check_digit(state, d7, 26, -13, 11);
                                    for d8 in (1..=9).rev() {
                                        let state = check_digit(state, d8, 1, 12, 6);
                                        for d9 in (1..=9).rev() {
                                            let state = check_digit(state, d9, 26, -1, 15);
                                            for d10 in (1..=9).rev() {
                                                let state = check_digit(state, d10, 1, 14, 7);
                                                for d11 in (1..=9).rev() {
                                                    let state = check_digit(state, d11, 26, -5, 1);
                                                    for d12 in (1..=9).rev() {
                                                        let state = check_digit(state, d12, 26, -4, 8);
                                                        for d13 in (1..=9).rev() {
                                                            let state = check_digit(state, d13, 26, -8, 6);
                                                            //let inputs = VecDeque::from([d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13]);
                                                            //let results = interpret(&input_str, inputs);
                                                            if state.z == 0 {
                                                                println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13);
                                                                return;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[inline]
fn check_digits_2(d1_lower: i64, d1_upper: i64) {
    let mut cache: HashSet<(usize, i64)> = HashSet::new();
    for d0 in 1..=9 {
        let state = check_digit(ALUState::new(), d0, 12, 11, 1);
        if cache.contains(&(0, state.z)) {
            continue;
        }
        for d1 in d1_lower..=d1_upper {
            let state = check_digit(state, d1, 1, 11, 11);
            println!("{}{}...", d0, d1);
            if cache.contains(&(1, state.z)) {
                continue;
            }  else {
                cache.insert((1, state.z));
            }
            for d2 in 1..=9 {
                let state = check_digit(state, d2, 1, 14, 1);
                if cache.contains(&(2, state.z)) {
                    continue;
                } else {
                    cache.insert((2, state.z));
                }
                for d3 in 1..=9 {
                    let state = check_digit(state, d3, 1, 11, 11);
                    if cache.contains(&(3, state.z)) {
                        continue;
                    } else {
                        cache.insert((3, state.z));
                    }
                    for d4 in 1..=9 {
                        let state = check_digit(state, d4, 26, -8, 2);
                        if cache.contains(&(4, state.z)) {
                            continue;
                        } else {
                            cache.insert((4, state.z));
                        }
                        for d5 in 1..=9 {
                            let state = check_digit(state, d5, 26, -5, 9);
                            if cache.contains(&(5, state.z)) {
                                continue;
                            } else {
                                cache.insert((5, state.z));
                            }
                            for d6 in 1..=9 {
                                let state = check_digit(state, d6, 1, 11, 7);
                                if cache.contains(&(6, state.z)) {
                                    continue;
                                } else {
                                    cache.insert((6, state.z));
                                }
                                for d7 in 1..=9 {
                                    let state = check_digit(state, d7, 26, -13, 11);
                                    if cache.contains(&(7, state.z)) {
                                        continue;
                                    } else {
                                        cache.insert((7, state.z));
                                    }
                                    for d8 in 1..=9 {
                                        let state = check_digit(state, d8, 1, 12, 6);
                                        if cache.contains(&(8, state.z)) {
                                            continue;
                                        }else {
                                            cache.insert((8, state.z));
                                        }
                                        for d9 in 1..=9 {
                                            let state = check_digit(state, d9, 26, -1, 15);
                                            if cache.contains(&(9, state.z)) {
                                                continue;
                                            } else {
                                                cache.insert((9, state.z));
                                            }
                                            for d10 in 1..=9 {
                                                let state = check_digit(state, d10, 1, 14, 7);
                                                if cache.contains(&(10, state.z)) {
                                                    continue;
                                                } else {
                                                    cache.insert((10, state.z));
                                                }
                                                for d11 in 1..=9 {
                                                    let state = check_digit(state, d11, 26, -5, 1);
                                                    for d12 in 1..=9 {
                                                        let state = check_digit(state, d12, 26, -4, 8);
                                                        for d13 in 1..=9 {
                                                            let state = check_digit(state, d13, 26, -8, 6);
                                                            //let inputs = VecDeque::from([d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13]);
                                                            //let results = interpret(&input_str, inputs);
                                                            if state.z == 0 {
                                                                println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13);
                                                                return;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        cache.insert((0, state.z));
    }
}

#[derive(Copy, Clone)]
struct ALUState {
    w: i64,
    x: i64,
    y: i64,
    z: i64
}

impl ALUState {
    fn new() -> ALUState {
        ALUState {
            w: 0,
            x: 0,
            y: 0,
            z: 0
        }
    }

    fn load(&mut self, var: char, val: i64) {
        match var {
            'w' => self.w = val,
            'x' => self.x = val,
            'y' => self.y = val,
            'z' => self.z = val,
            _ => panic!("unknown var {}", var)
        }
    }

    fn get_val(&self, var: &str) -> i64 {
        if let Ok(nr) = var.parse() {
            nr
        } else {
            match var {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => panic!("unknown var {}", var)
            }
        }
    }

    fn get_val_mut(&mut self, var: char) -> &mut i64 {
        match var {
            'w' => &mut self.w,
            'x' => &mut self.x,
            'y' => &mut self.y,
            'z' => &mut self.z,
            _ => panic!("unknown var {}", var)
        }
    }
}

impl Display for ALUState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ALU(w: {}, x: {}, y: {}, z: {})", self.w, self.x, self.y, self.z)
    }
}

fn interpret(program: &str, mut inputs: VecDeque<i64>) -> ALUState {
    let mut state = ALUState::new();
    for line in program.split("\n").filter(|l| !l.is_empty()) {
        let (inst, params_str) = line.split_once(" ").unwrap();
        let params = params_str.split(" ").collect::<Vec<&str>>();
        let param0 = params[0].chars().next().unwrap();
        match inst {
            "inp" => state.load(param0, inputs.pop_front().unwrap()),
            "add" => *state.get_val_mut(param0) += state.get_val(params[1]),
            "mul" => *state.get_val_mut(param0) *= state.get_val(params[1]),
            "div" => *state.get_val_mut(param0) /= state.get_val(params[1]),
            "mod" => *state.get_val_mut(param0) %= state.get_val(params[1]),
            "eql" => {
                let result = if state.get_val(params[0]) == state.get_val(params[1]) {
                    1i64
                } else { 0 };
                *state.get_val_mut(param0) = result;
            }
            _ => panic!("unknown instruction {}", inst)
        }
    }
    state
}

fn check_digit(mut state: ALUState, digit: i64, param1: i64, param2: i64, param3: i64) -> ALUState {
    state.x = state.z; //mul x 0 + add x z
    state.x %= 26; //mod x 26
    state.z /= param1; //div z [param1]
    state.x += param2;//add x [param2]
    state.x = if state.x == digit { 0 } else { 1 };//eql x w + eql x 0
    state.y = 25;//mul y 0 + add y 25
    state.y *= state.x;//mul y x
    state.y += 1;//add y 1
    state.z *= state.y;//mul z y
    state.y = digit + param3;//mul y 0 + add y w + add y [param3]
    state.y *= state.x;// mul y x
    state.z += state.y; //add z y
    state
}