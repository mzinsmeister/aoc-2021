use std::fs::read_to_string;

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let (p1_line, p2_line) = input_str.strip_suffix("\n").unwrap().split_once("\n").unwrap();
    let p1_start_str = p1_line.strip_prefix("Player 1 starting position: ").unwrap();
    let p2_start_str = p2_line.strip_prefix("Player 2 starting position: ").unwrap();
    let p1_start: u8 = p1_start_str.parse().unwrap();
    let p2_start: u8 = p2_start_str.parse().unwrap();

    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut die_pos = 0u32;
    // Having scores 0-9 instead of 1-10 is easier to handle
    let mut p1_pos = p1_start as u32 - 1;
    let mut p2_pos = p2_start as u32 - 1;
    let mut next_p = 1;
    let mut die_rolled = 0;
    while p1_score < 1000 && p2_score < 1000 {
        let die_value = die_pos * 3 + 6;
        die_pos += 3;
        die_rolled += 3;
        if next_p == 1 {
            p1_pos += die_value;
            p1_pos %= 10;
            p1_score += p1_pos + 1;
            next_p = 2;
        } else {
            p2_pos += die_value;
            p2_pos %= 10;
            p2_score += p2_pos + 1;
            next_p = 1;
        }
    }
    let losing_player_score = if p1_score < p2_score {
        p1_score
    } else {
        p2_score
    };
    println!("{}", losing_player_score * die_rolled);


    // Pt. 2. I know i could add a cache here to make it way faster but
    // didn't see a need to do so since it runs in < 1s especially with --release for me

    let (p1_wins, p2_wins) = play_game(p1_start as u32 - 1, p2_start as u32 - 1, 0, 0, 1);
    println!("{},{}", p1_wins, p2_wins);

}

const FACTORS: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];
const WIN_THRESHOLD: u32 = 21;

fn play_game(p1_pos: u32, p2_pos: u32, p1_score: u32, p2_score: u32, next_p: u32) -> (u64, u64) {
    // println!("{},{},{},{},{}", p1_pos, p2_pos, p1_score, p2_score, next_p);
    let (mut p1_wins, mut p2_wins) = (0u64, 0u64);
    for roll in 3..=9 {
        let (tp1_wins, tp2_wins) = if next_p == 1 {
            let p1_pos_new = (p1_pos + roll) % 10;
            let p1_score_new = p1_score + p1_pos_new + 1;
            if p1_score_new >= WIN_THRESHOLD {
                (1, 0)
            } else {
                play_game(p1_pos_new, p2_pos, p1_score_new, p2_score, 2)
            }
        } else {
            let p2_pos_new = (p2_pos + roll) % 10;
            let p2_score_new = p2_score + p2_pos_new + 1;
            if p2_score_new >= WIN_THRESHOLD {
                (0, 1)
            } else {
                play_game(p1_pos, p2_pos_new, p1_score, p2_score_new, 1)
            }
        };
        let roll_factor = FACTORS[roll as usize - 3];
        p1_wins += roll_factor * tp1_wins;
        p2_wins += roll_factor * tp2_wins;
    }
    (p1_wins, p2_wins)
}

