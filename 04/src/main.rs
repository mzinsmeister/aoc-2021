use std::fmt::{Display, Formatter, Pointer};
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let mut game: BingoGame = input_str.parse().unwrap();
    let result = game.play();
    println!("{}", result);
}

#[derive(Debug)]
struct BingoGame {
    boards: Vec<BingoBoard>,
    numbers: Vec<u8>
}

impl BingoGame {
    fn play(&mut self) -> u32 {
        for i in self.numbers.iter() {
            self.boards.iter_mut().for_each(|b| b.mark(*i));
            /* P1: for board in self.boards.iter() {
                 if board.has_bingo() {
                    println!("{}", board);
                    return board.calculate_score();
                }
            }*/
            if self.boards.len() == 1 && self.boards[0].has_bingo() {
                println!("{}", self.boards[0]);
                return self.boards[0].calculate_score();
            }
            self.boards.retain(|b| !b.has_bingo()); // ONLY P2!!
        }
        panic!("not finished")
    }
}

impl FromStr for BingoGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut partiter = s.split("\n\n");
        let numbers: Vec<u8> = partiter.next().unwrap()
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect();
        let boards: Vec<BingoBoard> = partiter.map(|i| i.parse().unwrap()).collect();
        Ok(BingoGame { numbers, boards })
    }
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<(u8, bool)>>,
    last_num: u8
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in self.board.iter() {
            for (e, m) in line.iter() {
                write!(f, "{:2}", e);
                if *m {
                    write!(f, "!");
                } else {
                    write!(f, " ");
                }
                write!(f, " ");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BingoBoard { board: s.split("\n").filter(|e| !e.is_empty())
            .map(|line|
            line.split(" ")
                .filter(|e| !e.is_empty())
                .map(|e| (e.parse().unwrap(), false))
                .collect()
        ).collect(), last_num: 0})
    }
}

impl BingoBoard {

    fn mark(&mut self, num: u8) {
        self.last_num = num;
        let mut pos: Vec<(usize, usize)> = Vec::new();
        for (i, line) in self.board.iter().enumerate() {
            for (j, elem) in line.iter().enumerate() {
                if elem.0 as u8 == num {
                    pos.push((i, j));
                }
            }
        }
        pos.iter().for_each(|&(i, j)| self.board[i][j].1 = true)
    }

    fn has_bingo(&self) -> bool {
        for i in 0..5 {
            if self.board.iter().all(|l| l[i].1) {
                return true;
            }
        }
        self.board.iter().map(|l| l.iter().all(|i| i.1)).any(|e| e)
    }

    fn calculate_score(&self) -> u32 {
        self.board.iter().flatten().filter(|(_, m)| !*m).map(|(i, _)| *i as u32)
            .sum::<u32>() * self.last_num as u32
    }
}