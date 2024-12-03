use crate::etc::lib::lcm;
use crate::{Solution, SolutionPair};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day08.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(_input: &Vec<String>) -> u64 {
    let dirs = parse_dir(_input.get(0).unwrap());
    let instructions = parse_instructions(_input);

    find_path(&dirs, &instructions, "AAA", |pos| pos == "ZZZ")
}

fn sol2(_input: &Vec<String>) -> u64 {
    let dirs = parse_dir(_input.get(0).unwrap());
    let instructions = parse_instructions(_input);

    let start_pos: Vec<&str> = instructions
        .keys()
        .map(|k| *k)
        .filter(|k| k.ends_with("A"))
        .collect();

    let steps = start_pos
        .iter()
        .map(|p| find_path(&dirs, &instructions, p, |pos| pos.ends_with("Z")))
        .collect::<Vec<u64>>();

    steps.iter().fold(1, |a, b| lcm(a, *b))
}

///////////////////////////////////////////////////////////////////////////////

type Node = str;

fn parse_dir(input: &String) -> Vec<char> {
    input.chars().collect()
}

fn parse_instructions(input: &Vec<String>) -> HashMap<&Node, (&Node, &Node)> {
    let re = Regex::new(r"\w+").unwrap();

    input.iter().fold(HashMap::new(), |mut acc, line| {
        if line.find("=").is_none() {
            return acc;
        }

        let parts: Vec<&str> = re.find_iter(line).map(|p| p.as_str()).collect();
        acc.insert(parts[0], (parts[1], parts[2]));
        acc
    })
}

fn find_path(
    dirs: &Vec<char>,
    instructions: &HashMap<&Node, (&Node, &Node)>,
    start: &str,
    end: fn(&str) -> bool,
) -> u64 {
    let mut pos: &str = start;
    let mut i = 0;

    loop {
        let instruction = instructions.get(pos).unwrap();

        if dirs[i % dirs.len()] == 'L' {
            pos = instruction.0;
        } else {
            pos = instruction.1;
        }

        i += 1;

        if end(pos) {
            break;
        }
    }

    i as u64
}

#[cfg(test)]
mod tests {
    use crate::day08;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day08::solve();

        assert_eq!(result.0, Solution::U64(19099));
        assert_eq!(result.1, Solution::U64(17099847107071));
    }
}
