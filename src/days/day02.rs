use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day02.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let mut score: u64 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        score += get_score(chars[0], chars[2]);
    }
    score
}

fn sol2(input: &Vec<String>) -> u64 {
    let mut score: u64 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let my_move = determine_move(chars[0], chars[2]);
        score += get_score(chars[0], my_move);
    }
    score
}

///////////////////////////////////////////////////////////////////////////////

const ROCK: u64 = 1;
const PAPER: u64 = 2;
const SCISSORS: u64 = 3;

const WIN: u64 = 6;
const DRAW: u64 = 3;
const LOSE: u64 = 0;

fn get_score(opponent: char, my: char) -> u64 {
    match opponent {
        'A' => match my {
            'X' => ROCK + DRAW,
            'Y' => PAPER + WIN,
            'Z' => SCISSORS + LOSE,
            _ => unimplemented!(),
        },
        'B' => match my {
            'X' => ROCK + LOSE,
            'Y' => PAPER + DRAW,
            'Z' => SCISSORS + WIN,
            _ => unimplemented!(),
        },
        'C' => match my {
            'X' => ROCK + WIN,
            'Y' => PAPER + LOSE,
            'Z' => SCISSORS + DRAW,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn determine_move(opponent: char, my: char) -> char {
    match opponent {
        'A' => match my {
            'X' => 'Z',
            'Y' => 'X',
            'Z' => 'Y',
            _ => unimplemented!(),
        },
        'B' => match my {
            'X' => 'X',
            'Y' => 'Y',
            'Z' => 'Z',
            _ => unimplemented!(),
        },
        'C' => match my {
            'X' => 'Y',
            'Y' => 'Z',
            'Z' => 'X',
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
