use crate::{Solution, SolutionPair};
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
    let re = Regex::new(r"(\d)").unwrap();
    input
        .iter()
        .map(|line| {
            let first = find_num(&re, line);
            let last = find_num(&re, &line.chars().rev().collect::<String>());
            first * 10 + last
        })
        .sum()
}

fn sol2(input: &Vec<String>) -> u64 {
    let re_first = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input
        .iter()
        .map(|line| {
            let first = find_string_num(&re_first, line);
            let last = find_string_num(&re_last, line);
            first * 10 + last
        })
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

fn find_num(re: &Regex, input: &String) -> u64 {
    return re.captures(input).unwrap()[1].parse().unwrap();
}

fn find_string_num(re: &Regex, input: &String) -> u64 {
    let capture = &re.captures(input).unwrap()[1];
    match capture {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => capture.parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day01;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day01::solve();

        assert_eq!(result.0, Solution::U64(53974));
        assert_eq!(result.1, Solution::U64(52840));
    }
}
