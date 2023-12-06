use std::task::Wake;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Time:      7  15   30
    // Distance:  9  40  200
    let _testing = vec![(7, 9), (15, 40), (30, 200)];

    // Time:        54     70     82     75
    // Distance:   239   1142   1295   1253
    let input = vec![(54, 239), (70, 1142), (82, 1295), (75, 1253)];

    (Solution::U64(sol1(&input)), Solution::U64(sol2()))
}

fn sol1(input: &Vec<(u64, u64)>) -> u64 {
    input
        .into_iter()
        .map(|i| determine_ways(i.0, i.1))
        .fold(1, |acc, i| acc * i)
}

fn sol2() -> u64 {
    // determine_ways(71530, 940200)
    determine_ways(54708275, 239114212951253)
}

///////////////////////////////////////////////////////////////////////////////

fn determine_ways(time: u64, distance: u64) -> u64 {
    let mut valid = 0;
    for i in 0..time {
        if (time - i) * i > distance {
            valid += 1;
        }
    }
    valid
}
