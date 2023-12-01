use crate::{Solution, SolutionPair};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day01.txt").unwrap();
    let parsed: Vec<String> = input
        .lines()
        .map(|line| line.parse().unwrap_or_default())
        .collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    input
        .iter()
        .map(|line| {
            let first = find_num(line);
            let last = find_num(&line.chars().rev().collect::<String>());
            first * 10 + last
        })
        .sum()
}

fn sol2(input: &Vec<String>) -> u64 {
    input
        .iter()
        .map(|line| {
            let first = find_string_num(line);
            let last = find_string_num(&line.chars().rev().collect::<String>());
            first * 10 + last
        })
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

fn find_num(input: &String) -> u64 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d)").unwrap());
    return RE.captures(input).unwrap()[1].parse().unwrap();
}

fn find_string_num(input: &String) -> u64 {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap()
    });
    let capture = &RE.captures(input).unwrap()[1];
    match capture {
        "one" => 1,
        "eno" => 1,
        "two" => 2,
        "owt" => 2,
        "three" => 3,
        "eerht" => 3,
        "four" => 4,
        "ruof" => 4,
        "five" => 5,
        "evif" => 5,
        "six" => 6,
        "xis" => 6,
        "seven" => 7,
        "neves" => 7,
        "eight" => 8,
        "thgie" => 8,
        "nine" => 9,
        "enin" => 9,
        _ => capture.parse().unwrap(),
    }
}
