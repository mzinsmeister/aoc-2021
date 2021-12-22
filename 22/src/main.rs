use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet};
use std::fs::read_to_string;
use std::ops::{RangeInclusive};
use std::time::SystemTime;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let instructions = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_instruction)
        .collect::<Vec<Instruction>>();

    let start = SystemTime::now();

    let mut size = 0;
    for (i, instruction) in preprocess_instructions_1(&instructions).iter().enumerate() {
        let mut cuboid = CuboidWithCutouts::new(instruction.cuboid);
        if !instruction.on {
            cuboid.cutout(&cuboid.cuboid.clone());
        }
        for instruction2 in instructions.iter().take(i) {
            if instruction2.on == instruction.on{
                cuboid.cutout(&instruction2.cuboid)
            } else {
                cuboid.add(&instruction2.cuboid)
            }
        }
        if instruction.on {
            size += cuboid.size();
        } else {
            size -= cuboid.size();
        }
    }

    println!("P1: {}", size);
    println!("in: {}ms", start.elapsed().unwrap().as_millis());

    let start = SystemTime::now();

    let mut size = 0;
    for (i, instruction) in instructions.iter().enumerate() {
        let mut cuboid = CuboidWithCutouts::new(instruction.cuboid);
        if !instruction.on {
            cuboid.cutout(&cuboid.cuboid.clone());
        }
        for instruction2 in instructions.iter().take(i) {
            if instruction2.on == instruction.on{
                cuboid.cutout(&instruction2.cuboid)
            } else {
                cuboid.add(&instruction2.cuboid)
            }
        }
        if instruction.on {
            size += cuboid.size();
        } else {
            size -= cuboid.size();
        }
    }

    println!("{}", size);
    println!("in: {}ms", start.elapsed().unwrap().as_millis());
}

fn parse_instruction(input: &str) -> Instruction {
    let (rest, on) = if input.starts_with("on") {
        (&input[3..], true)
    } else {
        (&input[4..], false)
    };
    let mut range_iter = rest.split(",");
    let (x_start_str, x_end_str) = range_iter.next().unwrap()[2..].split_once("..").unwrap();
    let (y_start_str, y_end_str) = range_iter.next().unwrap()[2..].split_once("..").unwrap();
    let (z_start_str, z_end_str) = range_iter.next().unwrap()[2..].split_once("..").unwrap();
    /*Instruction {
        on,
        x_range: x_start_str.parse().unwrap()..=x_end_str.parse().unwrap(),
        y_range: y_start_str.parse().unwrap()..=y_end_str.parse().unwrap(),
        z_range: z_start_str.parse().unwrap()..=z_end_str.parse().unwrap(),
    }*/
    Instruction {
        on,
        cuboid: Cuboid {
            x_range: (x_start_str.parse().unwrap(), x_end_str.parse().unwrap()),
            y_range: (y_start_str.parse().unwrap(), y_end_str.parse().unwrap()),
            z_range: (z_start_str.parse().unwrap(), z_end_str.parse().unwrap()),
        }
    }
}
fn preprocess_instructions_1(input_instructions: &Vec<Instruction>) -> Vec<Instruction> {
    input_instructions.iter().map(|i| Instruction {
        on: i.on,
        cuboid: Cuboid {
            x_range: (max(i.cuboid.x_range.0, -50), min(i.cuboid.x_range.1, 50)),
            y_range: (max(i.cuboid.y_range.0, -50), min(i.cuboid.y_range.1, 50)),
            z_range: (max(i.cuboid.z_range.0, -50), min(i.cuboid.z_range.1, 50))
        }
    }).collect()
}

#[derive(Clone, Debug)]
struct Instruction {
    on: bool,
    cuboid: Cuboid,
    /*x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>*/
}

#[derive(Copy, Clone, Debug)]
struct Cuboid {
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32)
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let x_intersection_start = max(self.x_range.0, other.x_range.0);
        let x_intersection_end = min(self.x_range.1, other.x_range.1);
        let y_intersection_start = max(self.y_range.0, other.y_range.0);
        let y_intersection_end = min(self.y_range.1, other.y_range.1);
        let z_intersection_start = max(self.z_range.0, other.z_range.0);
        let z_intersection_end = min(self.z_range.1, other.z_range.1);
        let cuboid = Cuboid {
            x_range: (x_intersection_start, x_intersection_end),
            y_range: (y_intersection_start, y_intersection_end),
            z_range: (z_intersection_start, z_intersection_end),
        };
        if cuboid.exists() {
            Some(cuboid)
        } else {
            None
        }
    }

    fn exists(&self) -> bool {
        self.x_range.0 <= self.x_range.1 && self.y_range.0 <= self.y_range.1
            && self.z_range.0 <= self.z_range.1
    }

    fn size(&self) -> u64 {
        (self.x_range.1 - self.x_range.0 + 1) as u64 * (self.y_range.1 - self.y_range.0 + 1) as u64
            * (self.z_range.1 - self.z_range.0 + 1) as u64
    }
}

#[derive(Clone, Debug)]
struct CuboidWithCutouts {
    cuboid: Cuboid,
    cutouts: Vec<CuboidWithCutouts>
}

impl CuboidWithCutouts {
    fn new(cuboid: Cuboid) -> CuboidWithCutouts {
        CuboidWithCutouts {
            cuboid,
            cutouts: Vec::new(),
        }
    }

    fn cutout(&mut self, cuboid: &Cuboid) {
        if let Some(intersection) = self.cuboid.intersection(cuboid) {
            self.cutouts.iter_mut().for_each(|c| c.add(&intersection));
            self.cutouts.push(CuboidWithCutouts::new(intersection));
        }
    }

    fn add(&mut self, cuboid: &Cuboid) {
        if let Some(intersection) = self.cuboid.intersection(cuboid) {
            self.cutouts.iter_mut().for_each(|c| c.cutout(&intersection));
        }
    }

    fn intersection_size(&self, other:  &Cuboid) -> u64 {
        if let Some(intersection) = self.cuboid.intersection(other) {
            //println!("\n\n{:#?}, {:#?}", self, intersection);
            let mut intersection_size = intersection.size();
            for (i, c) in self.cutouts.iter().enumerate() {
                if let Some(intersection2) = c.cuboid.intersection(other) {
                    let size = c.intersection_size(other);
                    let cutout_intersections = self.cutout_intersections_to(i, &intersection2);
                    //println!("{}: {} - {} ({})", i, size, cutout_intersections, intersection_size);
                    intersection_size -= (size - cutout_intersections);
                }
            }
            intersection_size
        } else {
            0
        }
    }

    fn cutout_intersections_to(&self, i: usize, other: &Cuboid) -> u64 {
        let mut size = 0;
        //println!("{:#?}, i: {}", self, i);
        for j in 0..i {
            if let Some(intersection) = self.cutouts[j].cuboid.intersection(other) {
                let cutout_intersections = self.cutout_intersections_to(j, &intersection);
                //println!("  {}: {:#?} -> {:#?} -> {}", j, other, intersection, cutout_intersections);
                size += self.cutouts[j].intersection_size(&intersection) - cutout_intersections;
                //println!("newsize: {}", size);
            }
        }
        size
    }

    fn size(&self) -> u64 {
        self.intersection_size(&self.cuboid)
    }
}
