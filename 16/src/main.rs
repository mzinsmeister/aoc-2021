use std::fs::read_to_string;
use std::str::FromStr;
use num_bigint::BigUint;
use num_traits::Num;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input_line = input.split_once("\n").unwrap().0;
    let binary: String = input_line.chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .map(|d| format!("{:04b}", d))
        .collect();

    let (pkg, nexti) = parse_packet(&binary);
    println!("{:#?} {}\n", pkg, nexti);
    println!("{}\n", pkg.version_sum());
    println!("{}\n", pkg.data.eval());
}

fn parse_packet(input: &str) -> (BITSPacket, usize) {
    let version = u8::from_str_radix(&input[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&input[3..6], 2).unwrap();
    let (data, next) = match type_id {
        4 => parse_literal(&input[6..]),
        _ => parse_operator(type_id, &input[6..])
    };
    (BITSPacket {
        version,
        data
    }, next + 6)
}

fn parse_literal(input: &str) -> (BITSData, usize) {
    let mut current_num: String = String::new();
    let mut current_i = 0;
    loop {
        current_num += &input[(current_i + 1)..(current_i + 5)];
        current_i += 5;
        if input.chars().nth(current_i-5).unwrap() != '1' {
            break;
        }
    }
    (BITSData::Literal(BigUint::from_str_radix(&current_num, 2).unwrap()), current_i)
}

fn parse_operator(type_id: u8, input: &str) -> (BITSData, usize) {
    let length_type = input.chars().next().unwrap();
    let (length, first) = if length_type == '0' {
        (usize::from_str_radix(&input[1..16], 2).unwrap(), 16usize)
    } else {
        (usize::from_str_radix(&input[1..12], 2).unwrap(), 12)
    };
    if length_type == '0' {
        let subpkts = &input[first..];
        let mut next = 0;
        let mut pkts = Vec::new();
        while next < length as usize {
            let (pkg, nexti) = parse_packet(&subpkts[next..]);
            next += nexti;
            println!("{}", next);
            pkts.push(pkg);
        }
        (BITSData::new_operator(type_id, pkts), first + next)
    } else {
        let subpkts = &input[first..];
        let mut next: usize = 0;
        let mut pkts = Vec::new();
        for _ in 0..length {
            let (pkg, nexti) = parse_packet(&subpkts[next..]);
            next += nexti;
            pkts.push(pkg);
        }
        (BITSData::new_operator(type_id, pkts), next + first)
    }
}

#[derive(Debug)]
struct BITSPacket {
    version: u8,
    data: BITSData
}

impl BITSPacket {
    fn version_sum(&self) -> u32 {
        return self.version as u32 + match &self.data {
            BITSData::Literal(_) => 0,
            BITSData::Sum(v) => paketvec_sum(v),
            BITSData::Product(v) => paketvec_sum(v),
            BITSData::Minimum(v) => paketvec_sum(v),
            BITSData::Maximum(v) => paketvec_sum(v),
            BITSData::GreaterThan(v) => paketvec_sum(v),
            BITSData::LessThan(v) => paketvec_sum(v),
            BITSData::Equal(v) => paketvec_sum(v),
        }
    }
}

fn paketvec_sum(pkts: &Vec<BITSPacket>) -> u32 {
    pkts.iter()
        .map(|p| p.version_sum())
        .sum()
}

#[derive(Debug)]
enum BITSData {
    Literal(BigUint),
    Sum(Vec<BITSPacket>),
    Product(Vec<BITSPacket>),
    Minimum(Vec<BITSPacket>),
    Maximum(Vec<BITSPacket>),
    GreaterThan(Vec<BITSPacket>),
    LessThan(Vec<BITSPacket>),
    Equal(Vec<BITSPacket>)
}

impl BITSData {
    fn new_operator(type_id: u8, pkgs: Vec<BITSPacket>) -> BITSData {
        match type_id {
            0 => BITSData::Sum(pkgs),
            1 => BITSData::Product(pkgs),
            2 => BITSData::Minimum(pkgs),
            3 => BITSData::Maximum(pkgs),
            5 => BITSData::GreaterThan(pkgs),
            6 => BITSData::LessThan(pkgs),
            7 => BITSData::Equal(pkgs),
            _ => panic!("Inalid type id for operator")
        }
    }

    fn eval(&self) -> BigUint {
        match self {
            BITSData::Literal(b) => b.clone(),
            BITSData::Sum(v) => v.iter().map(|p| p.data.eval()).sum(),
            BITSData::Product(v) => v.iter()
                .map(|p| p.data.eval())
                .fold(BigUint::from(1u8), |a, v| a * v),
            BITSData::Minimum(v) => v.iter().map(|p| p.data.eval()).min().unwrap(),
            BITSData::Maximum(v) => v.iter().map(|p| p.data.eval()).max().unwrap(),
            BITSData::GreaterThan(v) =>
                if v[0].data.eval() > v[1].data.eval() { BigUint::from(1u8) } else { BigUint::from(0u8) },
            BITSData::LessThan(v) =>
                if v[0].data.eval() < v[1].data.eval() { BigUint::from(1u8) } else { BigUint::from(0u8) },
            BITSData::Equal(v) =>
                if v[0].data.eval() == v[1].data.eval() { BigUint::from(1u8) } else { BigUint::from(0u8) }
        }
    }
}
