use std::fs::read_to_string;
use std::ops::{Add, Deref, DerefMut};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<TupValue> = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| parse_tup_value(l).0)
        .collect();

    let mut res: TupValue = input[0].clone();

    for tup in input.iter().skip(1) {
        res = &res + tup;
    }
    println!("{}", res.magnitude());

    let mut max_magnitude = 0u32;
    for (i, tup1) in input.iter().enumerate() {
        for (j, tup2) in input.iter().enumerate() {
            if i != j {
                let magnitude = (tup1 + tup2).magnitude();
                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                }
            }
        }
    }
    println!("{}", max_magnitude);
    println!("time: {}", start.elapsed().unwrap().as_millis())
}

#[derive(Debug, Clone)]
enum TupValue {
    Literal(u32),
    Tuple(Box<(TupValue, TupValue)>)
}

impl TupValue {
    fn reduce(&mut self) {
        loop {
            if self.reduce_d(0).is_some() {
                // do nothing
            } else if !self.reduce_split() {
                return;
            }
            //println!("{}", self);
        }
    }

    fn reduce_d(&mut self, depth: u32) -> Option<(u32, u32)> {
        if let TupValue::Tuple(tup) = self {
            if depth >= 4 {
                if let (TupValue::Literal(l), TupValue::Literal(r)) = **tup {
                    let ret = (l, r);
                    *self = TupValue::Literal(0);
                    return Some(ret);
                }
            }
            if let Some((l, r)) = tup.deref_mut().0.reduce_d(depth + 1) {
                return if r > 0 {
                    tup.deref_mut().1.add_l(r);
                    Some((l, 0))
                } else {
                    Some((l, r))
                }
            }
            if let Some((l, r)) = tup.deref_mut().1.reduce_d(depth + 1) {
                return if l > 0 {
                    tup.deref_mut().0.add_r(l);
                    Some((0, r))
                } else {
                    Some((l, r))
                }
            }
            None
        } else {
            None
        }
    }

    fn add_l(&mut self, num: u32) {
        match self {
            TupValue::Literal(v) => { *self = TupValue::Literal(*v + num) },
            TupValue::Tuple(tup) => tup.deref_mut().0.add_l(num)
        }
    }

    fn add_r(&mut self, num: u32) {
        match self {
            TupValue::Literal(v) => { *self = TupValue::Literal(*v + num) },
            TupValue::Tuple(tup) => tup.deref_mut().1.add_r(num)
        }
    }

    fn reduce_split(&mut self) -> bool {
        match self {
            TupValue::Literal(v) => {
                if *v >= 10 {
                    *self = TupValue::Tuple(Box::new(
                        (TupValue::Literal(*v / 2),
                         TupValue::Literal((*v / 2) + (*v % 2)))));
                    true
                } else {
                    false
                }
            },
            TupValue::Tuple(tup) => {
                if tup.deref_mut().0.reduce_split() {
                    return true;
                }
                if tup.deref_mut().1.reduce_split() {
                    return true;
                }
                false
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            TupValue::Literal(v) => *v,
            TupValue::Tuple(tup) =>
                3 * tup.deref().0.magnitude() + 2 * tup.deref().1.magnitude()
        }
    }
}

impl Add for &TupValue {
    type Output = TupValue;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = TupValue::Tuple(Box::new((self.clone(), rhs.clone())));
        result.reduce();
        result
    }
}

impl Display for TupValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TupValue::Literal(v) => write!(f, "{}", v),
            TupValue::Tuple(tup) => write!(f, "[{},{}]", tup.deref().0, tup.deref().1)
        }
    }
}

fn parse_tup_value(input: &str) -> (TupValue, &str) {
    if input.chars().next().unwrap() == '[' {
        let (val1, next) = parse_tup_value(&input[1..]);
        let next = next.strip_prefix(",").unwrap();
        let (val2, next) = parse_tup_value(next);
        (TupValue::Tuple(Box::new((val1, val2))), &next[1..])
    } else {
        let num_str: String = input.chars().take_while(|c| c.is_numeric()).collect();
        (TupValue::Literal(num_str.parse().unwrap()), &input[num_str.len()..])
    }
}
