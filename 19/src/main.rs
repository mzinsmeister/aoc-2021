use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Display, Formatter};
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Point(i32, i32, i32);

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl Point {
    fn taxicab_distance(&self, p: &Point) -> u32 {
        (self.0 - p.0).abs() as u32 + (self.1 - p.1).abs() as u32 + (self.2 - p.2).abs() as u32
    }

    fn make_sorted_pair(self, other: Point) -> (Point, Point) {
        if self > other {
            (other, self)
        } else {
            (self, other)
        }
    }

    fn apply_rot(&self, rotation: Rotation, rotation_signs: (i32, i32, i32)) -> Point {
        Point(rotation_signs.0 * (rotation.x_accessor)(self),
              rotation_signs.1 * (rotation.y_accessor)(self),
              rotation_signs.2 * (rotation.z_accessor)(self)
        )
    }

    fn apply(&self, transformation: &Transformation) -> Point {
        Point(transformation.rotation_signs.0 * (transformation.rotation.x_accessor)(self) + transformation.base_shift.0,
              transformation.rotation_signs.1 * (transformation.rotation.y_accessor)(self) + transformation.base_shift.1,
              transformation.rotation_signs.2 * (transformation.rotation.z_accessor)(self) + transformation.base_shift.2
        )
    }
}

#[derive(Copy, Clone, Debug)]
struct Transformation {
    base_shift: (i32, i32, i32),
    rotation: Rotation,
    rotation_signs: (i32, i32, i32)
}

#[derive(Copy, Clone)]
struct Rotation {
    x_accessor: fn(&Point) -> i32,
    y_accessor: fn(&Point) -> i32,
    z_accessor: fn(&Point) -> i32,
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let point = Point(0, 1, 2);
        let x = (self.x_accessor)(&point);
        let y = (self.y_accessor)(&point);
        let z = (self.z_accessor)(&point);
        write!(f, "({} {} {})", x, y, z)
    }
}

// The rotation/rotation_signs combination include incorrect ones but
// it works so i don't really care. Could probably cut some time by only using correct ones

const ROTATIONS: [Rotation; 6] = [
    Rotation {
        x_accessor: |p| p.0,
        y_accessor: |p| p.1,
        z_accessor: |p| p.2
    },
    Rotation {
        x_accessor: |p| p.1,
        y_accessor: |p| p.0,
        z_accessor: |p| p.2
    },
    Rotation {
        x_accessor: |p| p.0,
        y_accessor: |p| p.2,
        z_accessor: |p| p.1
    },
    Rotation {
        x_accessor: |p| p.2,
        y_accessor: |p| p.1,
        z_accessor: |p| p.0
    },
    Rotation {
        x_accessor: |p| p.2,
        y_accessor: |p| p.0,
        z_accessor: |p| p.1
    },
    Rotation {
        x_accessor: |p| p.1,
        y_accessor: |p| p.2,
        z_accessor: |p| p.0
    }
];

const ROTATION_SIGNS_LIST: [(i32, i32, i32); 8] = [
    (1, 1, 1), (1, 1, -1), (1, -1, 1), (1, -1, -1), (-1, 1, 1), (-1, 1, -1), (-1, -1, 1), (-1, -1, -1)
];

// I know it's probably some of the ugliest and dirtiest code i've written in years...

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input: Vec<Vec<Point>> = input_str.split("\n\n")
        .map(parse_scanner)
        .collect();

    let mut input_pairs: Vec<BTreeMap<u32, BTreeSet<(Point,Point)>>> = Vec::new();
    for scn_vec in input.iter() {
        let mut scn_pairs = BTreeMap::new();
        for i in 0..scn_vec.len() {
            for j in (i + 1)..scn_vec.len() {
                let dist = scn_vec[i].taxicab_distance(&scn_vec[j]);
                if !scn_pairs.contains_key(&dist) {
                    scn_pairs.insert(dist, BTreeSet::new());
                }
                scn_pairs.get_mut(&dist).unwrap()
                    .insert(scn_vec[i].make_sorted_pair(scn_vec[j]));
            }
        }
        input_pairs.push(scn_pairs);
    }

    let mut transformations: BTreeMap<(usize, usize), Transformation> = BTreeMap::new();

    let mut tempmatches: BTreeSet<(Point, Point)> = BTreeSet::new();

    for (i, scn_pairs) in input_pairs.iter().enumerate() {
        for (j, scn2_pairs) in input_pairs.iter().enumerate().filter(|(j, _)| *j != i) {
            let mut candidates: Vec<((Point, Point), (Point, Point))> = Vec::new();
            for (&dist, point_pairs) in scn_pairs.iter() {
                if let Some(points2) = scn2_pairs.get(&dist) {
                    for p1 in point_pairs {
                        for p2 in points2 {
                            candidates.push((*p1, *p2));
                        }
                    }
                }
            }
            'cand_loop_inner:
            for (p1, p2) in candidates.iter() {
                for rot in ROTATIONS {
                    for rot_signs in ROTATION_SIGNS_LIST {
                        let p10_rotated = p1.0.apply_rot(rot, rot_signs);
                        let p10_base_shift = (p2.0.0 - p10_rotated.0,
                                              p2.0.1 - p10_rotated.1, p2.0.2 - p10_rotated.2);
                        let transformation = Transformation {
                            rotation: rot,
                            rotation_signs: rot_signs,
                            base_shift: p10_base_shift
                        };

                        let mut done = false;
                        // if this isn't true, the transformation can't be right
                        if p1.1.apply(&transformation) == p2.1 {
                            let mut matches: BTreeSet<(Point, Point)> = BTreeSet::new();
                            for (pt1, pt2) in candidates.iter() {
                                if pt1.0.apply(&transformation) == pt2.0 {
                                    matches.insert((pt1.0, pt2.0));
                                    if pt1.1.apply(&transformation) == pt2.1 {
                                        matches.insert((pt1.1, pt2.1));
                                    }
                                } else if pt1.1.apply(&transformation) == pt2.0 {
                                    matches.insert((pt1.1, pt2.0));
                                    if pt1.0.apply(&transformation) == pt2.1 {
                                        matches.insert((pt1.0, pt2.1));
                                    }
                                }
                            }
                            if matches.len() >= 12 {
                                transformations.insert((i, j), transformation);
                                done = true;
                            }
                        }
                        if !done {
                            let p11_rotated = p1.1.apply_rot(rot, rot_signs);
                            let p11_base_shift = (p2.0.0 - p11_rotated.0,
                                                  p2.0.1 - p11_rotated.1, p2.0.2 - p11_rotated.2);
                            let transformation = Transformation {
                                rotation: rot,
                                rotation_signs: rot_signs,
                                base_shift: p11_base_shift
                            };
                            // if this isn't true, the transformation can't be right
                            if p1.0.apply(&transformation) == p2.1 {
                                let mut matches: BTreeSet<(Point, Point)> = BTreeSet::new();
                                for (pt1, pt2) in candidates.iter() {
                                    if pt1.0.apply(&transformation) == pt2.0 {
                                        matches.insert((pt1.0, pt2.0));
                                        if pt1.1.apply(&transformation) == pt2.1 {
                                            matches.insert((pt1.1, pt2.1));
                                        }
                                    } else if pt1.1.apply(&transformation) == pt2.0 {
                                        matches.insert((pt1.1, pt2.0));
                                        if pt1.0.apply(&transformation) == pt2.1 {
                                            matches.insert((pt1.0, pt2.1));
                                        }
                                    }
                                }
                                if matches.len() >= 12 {
                                    transformations.insert((i, j), transformation);
                                    done = true;
                                }
                            }
                        }
                        if done {
                            break 'cand_loop_inner;
                        }
                    }
                }
            }
        }
    }

    /*for t in transformations.iter() {
        println!("{}->{}", t.0.0, t.0.1);
    }*/

    let mut scn0consistent: BTreeMap<Point, BTreeSet<usize>> = input[0].iter().map(|p| (*p, BTreeSet::from([0]))).collect();
    for (i, points) in input.iter().enumerate().skip(1) {
        let path = search_transform(&transformations, i, 0, vec![]).expect(&format!("No path found: {}->0", i));

        for point in points {
            let mut point = *point;
            let mut last_t = i;
            for &t_n in path.iter().skip(1) {
                point = point.apply(&transformations[&(last_t, t_n)]);
                last_t = t_n;
            }
            if !scn0consistent.contains_key(&point) {
                scn0consistent.insert(point, BTreeSet::new());
            }
            scn0consistent.get_mut(&point).unwrap().insert(i);
        }
    }

    for p in scn0consistent.iter() {
       // println!("{},{},{}   [{}]", p.0.0, p.0.1, p.0.2, p.1.iter().map(|v| v.to_string() + ",").collect::<String>());
    }

    println!("{}", scn0consistent.len());

    let mut scn_positions = Vec::new();
    for i in 0..input.len() {
        let path = search_transform(&transformations, i, 0, vec![]).expect(&format!("No path found: {}->0", i));

        let mut point = Point(0,0,0);
        let mut last_t = i;
        for &t_n in path.iter().skip(1) {
            point = point.apply(&transformations[&(last_t, t_n)]);
            last_t = t_n;
        }
        scn_positions.push(point);
    }

    let mut largest = 0;

    for p1 in scn_positions.iter() {
        for p2 in scn_positions.iter() {
            let dist = p1.taxicab_distance(p2);
            if dist > largest {
                largest = dist;
            }
        }
    }

    println!("{}", largest);

}

fn search_transform(ts: &BTreeMap<(usize, usize), Transformation>, from: usize, to: usize, stack: Vec<usize>) -> Option<Vec<usize>> {
    for ((_, tempto), _) in ts.iter().filter(|((f, t), _)| *f == from) {
        if *tempto == to {
            return Some(vec![from, to]);
        } else {
            if !stack.contains(tempto) {
                if let Some(v) = search_transform(ts, *tempto, to, stack.iter().copied().chain(std::iter::once(from)).collect()) {
                    return Some(std::iter::once(from).chain(v.iter().copied()).collect());
                }
            }
        }
    }
    None
}

fn seen_points(pairs: &Vec<((Point, Point),(Point, Point))>) -> usize {
    let mut seen_points: BTreeSet<Point> = BTreeSet::new();
    for p in pairs {
        seen_points.insert(p.0.0);
        seen_points.insert(p.0.1);
    }
    //println!("{}", seen_points.len());
    seen_points.len()
}

fn parse_scanner(scn_in: &str) -> Vec<Point> {
    scn_in.split("\n")
        .filter(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let mut iter = l.split(",");
            Point(iter.next().unwrap().parse().unwrap(),
             iter.next().unwrap().parse().unwrap(),
             iter.next().unwrap().parse().unwrap())
        })
        .collect()
}
