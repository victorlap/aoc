use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day15.txt").unwrap();
    let parsed: Vec<String> = input.split(',').map(|s| s.to_string()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    input.iter().map(|s| hash(s) as u64).sum()
}

fn sol2(input: &Vec<String>) -> u64 {
    let boxes = input.iter().fold(
        HashMap::new(),
        |mut map: HashMap<u8, Vec<(String, u8)>>, s| {
            let label = if s.contains('-') {
                &s[..s.find('-').unwrap()]
            } else {
                &s[..s.find('=').unwrap()]
            };
            let h = hash(label);
            let boxx = map.entry(h).or_insert(vec![]);
            let entry = boxx.iter().find_position(|(l, _)| l == label);

            if s.contains('-') {
                if entry.is_some() {
                    boxx.remove(entry.unwrap().0);
                }
            } else {
                let focal: u8 = s.chars().last().unwrap().to_digit(10).unwrap() as u8;
                if entry.is_none() {
                    boxx.push((label.to_string(), focal));
                } else {
                    boxx.splice(
                        entry.unwrap().0..entry.unwrap().0 + 1,
                        vec![(label.to_string(), focal)],
                    );
                }
            }

            map
        },
    );

    boxes.into_iter().fold(0, |acc, (i, boxx)| {
        acc + boxx
            .into_iter()
            .enumerate()
            .fold(0, |acc, (j, (_, focal))| {
                acc + ((i as u64 + 1) * (j as u64 + 1) * focal as u64)
            })
    })
}

///////////////////////////////////////////////////////////////////////////////

fn hash(input: &str) -> u8 {
    input
        .bytes()
        .fold(0 as u8, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

#[cfg(test)]
mod tests {
    use crate::days::day15;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day15::solve();

        assert_eq!(result.0, Solution::U64(513643));
        assert_eq!(result.1, Solution::U64(265345));
    }
}
