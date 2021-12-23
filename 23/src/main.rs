use std::collections::BTreeMap;
use std::fs::read_to_string;
use lazy_static::lazy_static;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let input_chars = input_str.chars().collect::<Vec<char>>();
    let room1 = (input_chars[31], input_chars[45]);
    let room2 = (input_chars[33], input_chars[47]);
    let room3 = (input_chars[35], input_chars[49]);
    let room4 = (input_chars[37], input_chars[51]);

    let solution = solve_outer(&['.'; 11], &[vec![room1.0, room1.1], vec![room2.0, room2.1], vec![room3.0, room3.1], vec![room4.0, room4.1]], u32::MAX, 0).unwrap();

    for (hallway, rooms, c) in solution.0 {
        println!("{}", hallway.iter().collect::<String>());
        println!("  {} {} {} {}", rooms[0][0], rooms[1][0], rooms[2][0], rooms[3][0]);
        println!("  {} {} {} {}", rooms[0][1], rooms[1][1], rooms[2][1], rooms[3][1]);
        println!("{}\n", c);
    }

    println!("\n\n ==> {}", solution.1);

    let solution2 = solve_outer(&['.'; 11], &[vec![room1.0, 'D', 'D', room1.1], vec![room2.0, 'C', 'B', room2.1], vec![room3.0, 'B', 'A', room3.1], vec![room4.0, 'A', 'C', room4.1]], u32::MAX, 0).unwrap();

    for (hallway, rooms, c) in solution2.0 {
        println!("{}", hallway.iter().collect::<String>());
        println!("  {} {} {} {}", rooms[0][0], rooms[1][0], rooms[2][0], rooms[3][0]);
        println!("  {} {} {} {}", rooms[0][1], rooms[1][1], rooms[2][1], rooms[3][1]);
        println!("{}\n", c);
    }

    println!("\n\n ==> {}", solution2.1);
}

type Rooms = [Vec<char>; 4];
type Hallway = [char; 11];

lazy_static! {
    static ref COSTS: BTreeMap<char, u32> = BTreeMap::from([
        ('A', 1),
        ('B', 10),
        ('C', 100),
        ('D', 1000)
    ]);
}

lazy_static! {
    static ref GOALS: BTreeMap<char, usize> = BTreeMap::from([
        ('A', 0),
        ('B', 1),
        ('C', 2),
        ('D', 3)
    ]);
}

fn solve_outer(hallway: &Hallway, rooms: &Rooms, cutoff: u32, prev_cost: u32) -> Option<(Vec<(Hallway, Rooms, u32)>, u32)> {
    //println!("\n{}", hallway.iter().collect::<String>());
    //println!("  {} {} {} {}", rooms[0].0, rooms[1].0, rooms[2].0, rooms[3].0);
    //println!("  {} {} {} {}", rooms[0].1, rooms[1].1, rooms[2].1, rooms[3].1);
    //println!("{}", cutoff);
    if prev_cost > cutoff {
        return None;
    }
    if check_rooms(rooms) {
        return Some((vec![(*hallway, rooms.clone(), 0)], 0));
    }
    let mut min: Option<(Vec<(Hallway, Rooms, u32)>, u32)> = Option::None;
    let mut min_additional_cost = 0u32;
    for (p, c) in hallway.iter().enumerate().filter(|(_, c)| **c != '.') {
        let goal = GOALS[c];
        let hallway_goal = room_to_hallway_index(goal);
        for (i, &slot) in rooms[goal].iter().enumerate().rev() {
            if slot == '.' {
                let check_range = if hallway_goal > p { p + 1..=hallway_goal } else { hallway_goal..=p - 1 };
                if check_range.clone().all(|tp| hallway[tp] == '.') {
                    let mut hallway_new = *hallway;
                    hallway_new[p] = '.';
                    let mut rooms_new = rooms.clone();
                    rooms_new[goal][i] = *c;
                    let hallway_steps = check_range.end() - check_range.start();
                    let additional_cost = COSTS[c] * (hallway_steps as u32 + i as u32 + 2);
                    let cost_until = prev_cost + additional_cost;
                    if cost_until < cutoff {
                        let cutoff_new = min.as_ref().map_or(u32::MAX, |m| m.1 + prev_cost);
                        if let Some(solution) = solve(&hallway_new, &rooms_new, cutoff_new, cost_until) {
                            let cost = solution.1 + additional_cost;
                            if cost < min.as_ref().map_or(u32::MAX, |(_, c)| *c) {
                                min = Some((solution.0, cost));
                                min_additional_cost = additional_cost;
                            }
                        }
                    }
                }
                break;
            }
            if slot != *c {
                break;
            }
        }
    }
    for (i, room) in rooms.iter().enumerate() {
        println!("room {}", i);
        if !room.iter().all(|&c| c == '.' || i == GOALS[&c]) {
            let p = room_to_hallway_index(i);
            for (j, &slot) in room.iter().enumerate() {
                if slot != '.' {
                    for g in [0, 1, 3, 5, 7, 9, 10] {
                        println!("{}", g);
                        let check_range = if g > p { p+1..=g } else { g..=p-1 };
                        if check_range.clone().all(|tp| hallway[tp] == '.') {
                            let mut hallway_new = *hallway;
                            hallway_new[g] = slot;
                            let mut rooms_new = rooms.clone();
                            rooms_new[i][j] = '.';
                            let hallway_steps = check_range.end() - check_range.start();
                            let additional_cost = COSTS[&slot] * (hallway_steps as u32 + j as u32 + 2);
                            let cost_until = prev_cost + additional_cost;
                            if cost_until < cutoff {
                                let cutoff_new = min.as_ref().map_or(u32::MAX, |m| m.1 + prev_cost);
                                if let Some(solution) = solve(&hallway_new, &rooms_new, cutoff_new, cost_until) {
                                    let cost = solution.1 + additional_cost;
                                    if cost < min.as_ref().map_or(u32::MAX, |(_, c)| *c) {
                                        min = Some((solution.0, cost));
                                        min_additional_cost = additional_cost;
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
    min.map(|(v, c)| (std::iter::once((*hallway, rooms.clone(), min_additional_cost)).chain(v.iter().map(|(a, b, c)| (*a, b.clone(), *c))).collect(), c))
}

fn solve(hallway: &Hallway, rooms: &Rooms, cutoff: u32, prev_cost: u32) -> Option<(Vec<(Hallway, Rooms, u32)>, u32)> {
    //println!("\n{}", hallway.iter().collect::<String>());
    //println!("  {} {} {} {}", rooms[0][0], rooms[1][0], rooms[2][0], rooms[3][0]);
    //println!("  {} {} {} {}", rooms[0][1], rooms[1][1], rooms[2][1], rooms[3][1]);
    //println!("{}", cutoff);
    if prev_cost > cutoff {
        return None;
    }
    if check_rooms(rooms) {
        //println!("{} {}", prev_cost, cutoff);
        return Some((vec![(*hallway, rooms.clone(), 0)], 0));
    }
    let mut min: Option<(Vec<(Hallway, Rooms, u32)>, u32)> = Option::None;
    let mut min_additional_cost = 0u32;
    for (p, c) in hallway.iter().enumerate().filter(|(_, c)| **c != '.') {
        let goal = GOALS[c];
        let hallway_goal = room_to_hallway_index(goal);
        for (i, &slot) in rooms[goal].iter().enumerate().rev() {
            if slot == '.' {
                let check_range = if hallway_goal > p { p + 1..=hallway_goal } else { hallway_goal..=p - 1 };
                if check_range.clone().all(|tp| hallway[tp] == '.') {
                    let mut hallway_new = *hallway;
                    hallway_new[p] = '.';
                    let mut rooms_new = rooms.clone();
                    rooms_new[goal][i] = *c;
                    let hallway_steps = check_range.end() - check_range.start();
                    let additional_cost = COSTS[c] * (hallway_steps as u32 + i as u32 + 2);
                    let cost_until = prev_cost + additional_cost;
                    if cost_until < cutoff {
                        let cutoff_new = min.as_ref().map_or(u32::MAX, |m| m.1 + prev_cost);
                        if let Some(solution) = solve(&hallway_new, &rooms_new, cutoff_new, cost_until) {
                            let cost = solution.1 + additional_cost;
                            if cost < min.as_ref().map_or(u32::MAX, |(_, c)| *c) {
                                min = Some((solution.0, cost));
                                min_additional_cost = additional_cost;
                            }
                        }
                    }
                }
                break;
            }
            if slot != *c {
                break;
            }
        }
    }
    for (i, room) in rooms.iter().enumerate() {
        if !room.iter().all(|&c| c == '.' || i == GOALS[&c]) {
            let p = room_to_hallway_index(i);
            for (j, &slot) in room.iter().enumerate() {
                if slot != '.' {
                    for g in [0, 1, 3, 5, 7, 9, 10] {
                        let check_range = if g > p { p+1..=g } else { g..=p-1 };
                        if check_range.clone().all(|tp| hallway[tp] == '.') {
                            let mut hallway_new = *hallway;
                            hallway_new[g] = slot;
                            let mut rooms_new = rooms.clone();
                            rooms_new[i][j] = '.';
                            let hallway_steps = check_range.end() - check_range.start();
                            let additional_cost = COSTS[&slot] * (hallway_steps as u32 + j as u32 + 2);
                            let cost_until = prev_cost + additional_cost;
                            if cost_until < cutoff {
                                let cutoff_new = min.as_ref().map_or(u32::MAX, |m| m.1 + prev_cost);
                                if let Some(solution) = solve(&hallway_new, &rooms_new, cutoff_new, cost_until) {
                                    let cost = solution.1 + additional_cost;
                                    if cost < min.as_ref().map_or(u32::MAX, |(_, c)| *c) {
                                        min = Some((solution.0, cost));
                                        min_additional_cost = additional_cost;
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
    min.map(|(v, c)| (std::iter::once((*hallway, rooms.clone(), min_additional_cost)).chain(v.iter().map(|(a, b, c)| (*a, b.clone(), *c))).collect(), c))
}

#[inline]
fn room_to_hallway_index(room: usize) -> usize {
    2 + room * 2
}

fn check_rooms(rooms: &Rooms) -> bool {
    for (i, room) in rooms.iter().enumerate() {
        if room.iter().any(|c| *c == '.' || GOALS[c] != i) {
            return false;
        }
    }
    true
}