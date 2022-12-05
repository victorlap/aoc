use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day01.txt").unwrap();
    let parsed: Vec<i64> = input
        .lines()
        .map(|line| line.parse().unwrap_or_default())
        .collect();

    let elves: Vec<&[i64]> = parsed.split(|val| *val == 0).collect();

    let sol1: u64 = sol1(&elves);
    let sol2: u64 = sol2(&elves);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<&[i64]>) -> u64 {
    sum_calories(input)[0].try_into().unwrap()
}

fn sol2(input: &Vec<&[i64]>) -> u64 {
    let best_three = &sum_calories(input)[0..3];
    println!("{:?}", best_three);
    let sum: i64 = best_three.iter().sum();

    sum.try_into().unwrap()
}

///////////////////////////////////////////////////////////////////////////////

fn sum_calories(elves: &Vec<&[i64]>) -> Vec<i64> {
    let mut sum: Vec<i64> = elves.iter().map(|n| n.iter().sum()).collect();
    sum.sort_by(|a, b| b.partial_cmp(a).unwrap());
    sum
}
