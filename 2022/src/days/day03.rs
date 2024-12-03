use crate::{Solution, SolutionPair};

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day03.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;
    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let compartment1: Vec<char> = chars[0..chars.len() / 2].to_vec();
        let compartment2: Vec<char> = chars[chars.len() / 2..chars.len()].to_vec();
        let matching = find_matching(compartment1, compartment2);

        sum += get_priority(*matching.first().unwrap());
    }
    sum
}

fn sol2(input: &Vec<String>) -> u64 {
    let groups = input.chunks(3);
    let mut sum: u64 = 0;

    for group in groups {
        let compartment1: Vec<char> = group[0].chars().map(|chars| chars).collect();
        let compartment2: Vec<char> = group[1].chars().map(|chars| chars).collect();
        let compartment3: Vec<char> = group[2].chars().map(|chars| chars).collect();

        let matching = &find_matching(compartment1, compartment2);
        let badge = &find_matching(matching.clone(), compartment3);

        sum += get_priority(*badge.first().unwrap());
    }
    sum
}

///////////////////////////////////////////////////////////////////////////////

fn find_matching(compartment1: Vec<char>, compartment2: Vec<char>) -> Vec<char> {
    let mut matching: Vec<char> = vec![];
    for letter in compartment1.iter() {
        if compartment2.contains(letter) {
            matching.push(*letter);
        }
    }

    matching
}

fn get_priority(input: char) -> u64 {
    // input as u64 - 64
    let priority = input as u64;
    match priority >= 97 {
        true => priority - 96,
        false => priority - 38,
    }
}
