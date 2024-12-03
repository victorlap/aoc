use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day12.txt").unwrap();
    let parsed: Vec<Row> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<Row>) -> u64 {
    input
        .into_iter()
        .map(|line| find_arrangements(line.springs.clone(), line.groups.clone()))
        .sum()
}

fn sol2(input: &Vec<Row>) -> u64 {
    input
        .into_iter()
        .map(|row| {
            let expanded = expand(row);
            find_arrangements(expanded.springs, expanded.groups)
        })
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Row {
    springs: String,
    groups: Vec<usize>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let springs = parts.get(0).unwrap().parse().unwrap();
        let groups = parts
            .get(1)
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Row { springs, groups })
    }
}

fn find_arrangements(input: String, groups: Vec<usize>) -> u64 {
    static CACHE: Lazy<Mutex<HashMap<(String, Vec<usize>), u64>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    if CACHE
        .lock()
        .unwrap()
        .contains_key(&(input.clone(), groups.clone()))
    {
        return CACHE.lock().unwrap().get(&(input, groups)).unwrap().clone();
    }

    if input.len() == 0 {
        return (groups.len() == 0) as u64;
    }

    if groups.len() == 0 {
        return !input.contains('#') as u64;
    }

    let first_char = input.chars().next().unwrap();
    let first_group = *groups.get(0).unwrap();
    let mut res = 0;

    if first_char == '.' || first_char == '?' {
        res += find_arrangements(input[1..].to_string(), groups.clone());
    }
    if (first_char == '#' || first_char == '?')
        && first_group <= input.len()
        && !input.as_str()[0..first_group].contains('.')
    {
        if first_group == input.len() {
            res += find_arrangements("".to_string(), groups[1..].to_vec());
        } else if input.chars().nth(first_group).unwrap() != '#' {
            res += find_arrangements(input[first_group + 1..].to_string(), groups[1..].to_vec());
        }
    }

    CACHE.lock().unwrap().insert((input, groups), res);

    res
}

fn expand(input: &Row) -> Row {
    Row {
        springs: (0..4).fold(input.springs.clone(), |acc, _| {
            acc + "?" + input.springs.as_str()
        }),
        groups: input.groups.repeat(5),
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day12;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day12::solve();
        assert_eq!(result.0, Solution::U64(7260));
        assert_eq!(result.1, Solution::U64(1909291258644));
    }
}
