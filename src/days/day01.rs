use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day01.txt").unwrap();
    let parsed: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(depths: &Vec<i64>) -> u64 {
    let mut previous: i64 = 0;
    let mut increases: i64 = 0;

    for depth in depths {
        if previous > 0 && depth > &previous {
            increases += 1;
        }

        previous = *depth;
    }

    increases.try_into().unwrap()
}

fn sol2(depths: &Vec<i64>) -> u64 {
    let threesome_sum: Vec<i64> = depths.windows(3).map(|n| n.iter().sum()).collect();
    sol1(&threesome_sum)
}
