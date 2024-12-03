use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::ops::Range;

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

    for seeds in seeds.chunks(2) {
        let mut ranges = vec![seeds[0]..(seeds[0] + seeds[1])];

        for map in &maps {
            ranges = replace_ranges(ranges, map);
        }

        let seed_min = ranges.into_iter().map(|r| r.start).min().unwrap();
        if seed_min < min {
            min = seed_min;
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

fn replace_ranges(ranges: Vec<Range<i64>>, map: &InputMap) -> Vec<Range<i64>> {
    let mut result = vec![];
    for range in ranges.clone() {
        for change in map {
            let offset: i64 = change.destination - change.source;
            // Range fully contained in map
            if range.start >= change.source && range.end < (change.source + change.range) {
                result.push(Range {
                    start: range.start + offset,
                    end: range.end + offset,
                });
            // Range only partially changed, first part modified
            } else if range.start >= change.source
                && range.start < change.source + change.range
                && range.end >= (change.source + change.range)
            {
                result.push(Range {
                    start: range.start + offset,
                    end: change.source + change.range + offset,
                });
                result.push(Range {
                    start: change.source + change.range,
                    end: range.end,
                });
            // Range only partially changed, middle part modified
            } else if range.start <= change.source && range.end > (change.source + change.range) {
                result.push(Range {
                    start: range.start,
                    end: change.source,
                });
                result.push(Range {
                    start: change.source + offset,
                    end: change.source + change.range + offset,
                });
                result.push(Range {
                    start: change.source + change.range,
                    end: range.end,
                });
            // Range only partially changed, end part modified
            } else if range.start <= change.source
                && range.end > change.source
                && range.end < (change.source + change.range)
            {
                result.push(Range {
                    start: range.start,
                    end: change.source,
                });
                result.push(Range {
                    start: change.source + offset,
                    end: range.end + offset,
                });
            }
        }
    }

    if result.len() != 0 {
        result
    } else {
        ranges
    }
}

#[cfg(test)]
mod tests {
    use crate::day05;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day05::solve();

        assert_eq!(result.0, Solution::I64(282277027));
        assert_eq!(result.1, Solution::I64(11554135));
    }
}
