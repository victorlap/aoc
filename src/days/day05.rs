use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day05.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::I64(sol1(&parsed)), Solution::I64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> i64 {
    let seeds = parse_seeds(input.get(0).unwrap());
    let maps = parse_maps(input);

    find_min(seeds, maps)
}

fn sol2(input: &Vec<String>) -> i64 {
    let seeds = parse_seeds_ranges(input.get(0).unwrap());
    let maps = parse_maps(input);

    find_min(seeds, maps)
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

fn parse_seeds_ranges(input: &String) -> Vec<Seed> {
    input[7..input.len()]
        .split_whitespace()
        .into_iter()
        .collect::<Vec<&str>>()
        .chunks(2)
        .flat_map(|range| {
            let min: i64 = range[0].parse().unwrap();
            let len: i64 = range[1].parse().unwrap();
            (min..(min + len)).collect::<Vec<i64>>()
        })
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

fn find_min(seeds: Vec<Seed>, maps: Vec<InputMap>) -> i64 {
    // let mut i: i64 = 0;
    seeds
        .into_iter()
        .map(|seed| {
            // i += 1;
            // if i % 1000000 == 0 {
            //     println!(
            //         "progress: {}/{} = {}%",
            //         i,
            //         1679109896,
            //         (i * 100 / 1679109896)
            //     );
            // }
            let mut value = seed;
            for map in &maps {
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
        })
        .min()
        .unwrap()
}
