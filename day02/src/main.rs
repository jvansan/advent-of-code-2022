use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn move_value(played: &str) -> Move {
    match played {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Invalid move")
    }
}

fn move_score(mv: &Move) -> u32 {
    match mv {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn move_outcome(played: &str, opp_mv: Move) -> Move {
    // Lose
    if played == "X" {
        if opp_mv == Move::Rock {
            return Move::Scissors
        } else if opp_mv == Move::Paper {
            return Move::Rock
        } else {
            return Move::Paper
        }
    // Draw
    } else if played == "Y" {
        return opp_mv.clone()
    // Win
    } else {
        if opp_mv == Move::Rock {
            return Move::Paper
        } else if opp_mv == Move::Paper {
            return Move::Scissors
        } else {
            return Move::Rock
        }
    }

}

fn move_win(opp_mv: Move, play_mv: Move) -> u32 {
    if opp_mv == Move::Rock {
        if play_mv == Move::Paper {
            return 6
        } else if play_mv == Move::Rock {
            return 3
        } else {
            return 0
        }
    }
    else if opp_mv == Move::Paper {
        if play_mv == Move::Scissors {
            return 6
        } else if play_mv == Move::Paper {
            return 3
        } else {
            return 0
        }
    }
    // if opp_mv == Move::Scissors {
    else {
        if play_mv == Move::Rock {
            return 6
        } else if play_mv == Move::Scissors {
            return 3
        } else {
            return 0
        }
    }
}


fn main() {
    let fname = "/home/jvansan/projects/advent-of-code-2022/day02/input";
    let mut p1_total_score = 0;
    let mut p2_total_score = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines{
            let mut p1_round_score = 0;
            let mut p2_round_score = 0;
            if let Ok(line_str) = line {
                let mut moves = line_str.split(" ").collect::<Vec<&str>>();
                let play_char = moves.pop().unwrap();
                let p1_mv = move_value(play_char);
                let opp_mv = move_value(moves.pop().unwrap());
                let p2_mv = move_outcome(play_char, opp_mv);
                p1_round_score += move_score(&p1_mv);
                p1_round_score += move_win(opp_mv, p1_mv);
                p1_total_score += p1_round_score;

                p2_round_score += move_score(&p2_mv);
                p2_round_score += move_win(opp_mv, p2_mv);
                p2_total_score += p2_round_score;
            }
        }
    }
    println!("Total score p1: {:?}", p1_total_score);
    println!("Total score p2: {:?}", p2_total_score);
}

