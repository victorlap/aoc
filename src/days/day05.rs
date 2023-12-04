use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day03.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    0
}

fn sol2(input: &Vec<String>) -> u64 {
    0
}

///////////////////////////////////////////////////////////////////////////////
