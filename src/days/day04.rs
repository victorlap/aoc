use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, str::FromStr};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day04.txt").unwrap();
    let parsed: Vec<Card> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<Card>) -> u64 {
    input
        .into_iter()
        .map(|c| {
            let intersection = c.winning.intersection(&c.having).count() as u32;
            if intersection == 0 {
                0
            } else {
                2_u64.pow(intersection - 1)
            }
        })
        .sum()
}

fn sol2(input: &Vec<Card>) -> u64 {
    let mut extra_copies: HashMap<u64, u64> = HashMap::new();

    input
        .into_iter()
        .map(|c| {
            let cur_winning = c.winning.intersection(&c.having).count();
            let local_extra = extra_copies.get(&c.id).unwrap_or(&0).clone();

            for i in (1..cur_winning + 1) {
                let index = c.id + i as u64;
                if let Some(extra) = extra_copies.get(&index) {
                    extra_copies.insert(index, extra + local_extra + 1);
                } else {
                    extra_copies.insert(index, local_extra + 1);
                }
            }

            local_extra + 1
        })
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Card {
    id: u64,
    winning: HashSet<u64>,
    having: HashSet<u64>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let colon = input.find(":").unwrap();
        let pipe = input.find("|").unwrap();
        let id: u64 = input[5..colon].trim().parse().unwrap();
        let winning = parse_number_list(&input[colon + 2..pipe]);
        let having = parse_number_list(&input[pipe + 1..input.len()]);

        Ok(Card {
            id,
            winning,
            having,
        })
    }
}

fn parse_number_list(input: &str) -> HashSet<u64> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    parts.iter().map(|p| p.parse().unwrap()).collect()
}
