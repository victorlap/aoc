use crate::{Solution, SolutionPair};
use std::{f32::INFINITY, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day05.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::I64(sol1(&parsed)), Solution::I64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> i64 {
    let seeds = parse_seeds(input.get(0).unwrap());
    let maps = parse_maps(input);

    seeds.into_iter().map(|s| find_min(s, &maps)).min().unwrap()
}

fn sol2(input: &Vec<String>) -> i64 {
    let seeds = parse_seeds(input.get(0).unwrap());
    let maps = parse_maps(input);

    let mut min = i64::MAX;

    let mut i: i64 = 0;

    for seeds in seeds.chunks(2) {
        for seed in seeds[0]..(seeds[0] + seeds[1]) {
            i += 1;
            if i % 1_679_1098 == 0 {
                println!(
                    "progress: {}/{} = {}%",
                    i,
                    1679109896,
                    (i * 100 / 1_679_109_896)
                );
            }
            let seed_min = find_min(seed, &maps);
            if seed_min < min {
                min = seed_min;
            }
        }
    }

    min
}

///////////////////////////////////////////////////////////////////////////////

type Seed = i64;
type InputMap = Vec<RangeInfo>;

#[derive(Debug)]
struct RangeInfo {
    source: i64,
    destination: i64,
    range: i64,
}

fn parse_seeds(input: &String) -> Vec<Seed> {
    input[7..input.len()]
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap_or_default())
        .collect()
}

fn parse_maps(input: &Vec<String>) -> Vec<InputMap> {
    let mut result: Vec<InputMap> = vec![];
    let mut cur: InputMap = vec![];

    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if line == "" || parts.len() != 3 {
            if cur.len() != 0 {
                cur.sort_by(|a, b| a.source.cmp(&b.source));
                result.push(cur);
                cur = vec![];
            }
        } else {
            cur.push(RangeInfo {
                destination: parts[0].parse().unwrap(),
                source: parts[1].parse().unwrap(),
                range: parts[2].parse().unwrap(),
            })
        }
    }

    cur.sort_by(|a, b| a.source.cmp(&b.source));
    result.push(cur);

    result
}

fn find_min(seed: Seed, maps: &Vec<InputMap>) -> i64 {
    let mut value = seed;
    for map in maps {
        let ri = map.into_iter().find(|ri| {
            return ri.source + ri.range > value;
        });

        if let Some(found_ri) = ri {
            if value >= found_ri.source && found_ri.source + found_ri.range > value {
                let offset: i64 = found_ri.destination - found_ri.source;
                value += offset;
            }
        }
    }
    value
}
