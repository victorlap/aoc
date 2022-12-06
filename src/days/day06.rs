use crate::{Solution, SolutionPair};
use std::collections::HashSet;

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day06.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed[0]);
    let sol2: u64 = sol2(&parsed[0]);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &String) -> u64 {
    let chars: Vec<char> = input.chars().map(|c| c).collect();
    find_unique_characters(chars, 4)
}

fn sol2(input: &String) -> u64 {
    let chars: Vec<char> = input.chars().map(|c| c).collect();
    find_unique_characters(chars, 14)
}

///////////////////////////////////////////////////////////////////////////////

fn find_unique_characters(chars: Vec<char>, num: usize) -> u64 {
    let mut i: u64 = 0;

    for window in chars.windows(num) {
        let mut uniq = HashSet::new();
        if window.into_iter().all(move |x| uniq.insert(x)) {
            return i + (num as u64);
        }
        i += 1;
    }

    panic!("There should always be a unique window")
}
