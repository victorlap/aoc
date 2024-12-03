use crate::{Solution, SolutionPair};

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day04.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();
    let sections: Vec<SectionPair> = parsed
        .iter()
        .map(|line| section_pair_from_line(line))
        .collect();

    let sol1: u64 = sol1(&sections);
    let sol2: u64 = sol2(&sections);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<SectionPair>) -> u64 {
    let mut sum = 0;
    for pair in input {
        if pair.0.includes_all(&pair.1) || pair.1.includes_all(&pair.0) {
            sum += 1;
        }
    }
    sum.try_into().unwrap()
}

fn sol2(input: &Vec<SectionPair>) -> u64 {
    let mut sum = 0;
    for pair in input {
        if pair.0.includes_any(&pair.1) || pair.1.includes_any(&pair.0) {
            sum += 1;
        }
    }
    sum.try_into().unwrap()
}

///////////////////////////////////////////////////////////////////////////////

type SectionPair = (Section, Section);

#[derive(Debug)]
struct Section {
    areas: Vec<u8>,
}
impl Section {
    fn includes_all(&self, other: &Section) -> bool {
        self.areas.iter().all(|area| other.areas.contains(area))
    }
    fn includes_any(&self, other: &Section) -> bool {
        self.areas.iter().any(|area| other.areas.contains(area))
    }
}

fn section_from_line(line: &str) -> Section {
    let mut areas: Vec<u8> = vec![];
    let minus = line.find('-').unwrap();
    let begin: i32 = line[0..minus].parse().unwrap();
    let end: i32 = line[minus + 1..line.len()].parse().unwrap();

    for i in begin..=end {
        areas.push(i.try_into().unwrap());
    }

    Section { areas }
}

fn section_pair_from_line(line: &String) -> SectionPair {
    let comma = line.find(',').unwrap();
    let first = &line[0..comma];
    let second = &line[comma + 1..line.len()];

    (section_from_line(first), section_from_line(second))
}
