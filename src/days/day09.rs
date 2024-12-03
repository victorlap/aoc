use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day09.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::I128(sol1(&parsed)), Solution::I128(sol2(&parsed)))
}

fn sol1(_input: &Vec<String>) -> i128 {
    _input
        .into_iter()
        .map(|line| find_sequence_next(line))
        .sum()
}

fn sol2(_input: &Vec<String>) -> i128 {
    _input
        .into_iter()
        .map(|line| find_sequence_prev(line))
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

fn find_sequence_next(input: &str) -> i128 {
    let values: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let diffs = find_diffs(values);

    let mut prev_last = 0;
    for diff in diffs {
        prev_last = diff[diff.len() - 1] + prev_last;
    }

    prev_last as i128
}

fn find_sequence_prev(input: &str) -> i128 {
    let values: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let diffs = find_diffs(values);

    let mut prev_last = 0;
    for diff in diffs {
        prev_last = diff[0] - prev_last;
    }

    prev_last as i128
}

fn find_diffs(values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs = vec![values];

    loop {
        let diff = find_diff(&diffs[diffs.len() - 1]);
        if diff.len() == 0 || diff.iter().all(|&x| x == 0) {
            break;
        }

        diffs.push(diff)
    }

    diffs.reverse();
    diffs
}

fn find_diff(values: &Vec<i64>) -> Vec<i64> {
    let mut diffs: Vec<i64> = Vec::new();
    for chunk in values.windows(2) {
        diffs.push(chunk[1] - chunk[0])
    }
    diffs
}

#[cfg(test)]
mod tests {
    use crate::day09;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day09::solve();

        assert_eq!(result.0, Solution::I128(1939607039));
        assert_eq!(result.1, Solution::I128(1041));
    }
}
