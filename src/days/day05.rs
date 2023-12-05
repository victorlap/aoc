use crate::{Solution, SolutionPair};
use std::{f32::INFINITY, fs::read_to_string};
use std::thread;

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
    let mut children = vec![];

    for seeds in seeds.chunks(2) {
        let seed_start = seeds[0];
        let seed_end = seeds[0] + seeds[1];
        let own_map = maps.to_vec();

        children.push(thread::spawn(move || {
            let mut local_thread_min = i64::MAX;
            for seed in seed_start..seed_end {
                let seed_min = find_min(seed, &own_map);
                if seed_min < local_thread_min {
                    local_thread_min = seed_min;
                }
            }
            local_thread_min
        }))
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let thread_min = child.join().unwrap();
        if thread_min < min {
            min = thread_min;
        }
    }

    min
}

///////////////////////////////////////////////////////////////////////////////

type Seed = i64;
type InputMap = Vec<RangeInfo>;

#[derive(Debug, Clone)]
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
