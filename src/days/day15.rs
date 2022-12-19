use crate::{Solution, SolutionPair};

use std::{collections::HashSet, fs::read_to_string, ops::RangeInclusive};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day15.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let ranges = parse_field(&input);
    let line = 2_000_000;

    let mut result = ranges
        .iter()
        .filter_map(|sensor| range_on_line(sensor, line))
        .flatten()
        .collect::<HashSet<i64>>();

    ranges.iter().for_each(|info| {
        if info.1 .1 == line {
            result.remove(&info.1 .0);
        }
    });

    result.len() as u64
}

fn sol2(input: &Vec<String>) -> u64 {
    let ranges = parse_field(&input);
    let max = 4_000_000;

    for line in 0..=max {
        let result = ranges
            .iter()
            .filter_map(|sensor| range_on_line(sensor, line))
            .collect::<Vec<RangeInclusive<i64>>>();

        let (range, remainder) = combine_ranges(&result);

        if !remainder.is_empty() {
            result.iter().for_each(|_r| {
                // println!("- {:?}", r);
            });
            let x = if range.end() < remainder.get(0).unwrap().start() {
                range.end() + 1
            } else {
                range.start() - 1
            } as i64;
            return ((x * max) + line) as u64;
        }
    }
    0
}

///////////////////////////////////////////////////////////////////////////////

type Coordinate = (i64, i64);
type SensorInfo = (Coordinate, Coordinate);

fn parse_field(input: &Vec<String>) -> Vec<SensorInfo> {
    input
        .iter()
        .map(|part| string_to_sensor_info(part))
        .collect()
}

fn range_on_line(info: &SensorInfo, line: i64) -> Option<RangeInclusive<i64>> {
    let sensor = info.0;
    let beacon = info.1;
    let max_dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
    let y_needed = (sensor.1 - line).abs();
    if y_needed > max_dist {
        // out of reach
        return None;
    }
    let deviation = (max_dist - y_needed) as i64;
    Some((sensor.0 - deviation)..=(sensor.0 + deviation))
}

fn combine_ranges(
    ranges: &[RangeInclusive<i64>],
) -> (RangeInclusive<i64>, Vec<RangeInclusive<i64>>) {
    let mut indexes = (1..ranges.len()).collect::<HashSet<usize>>();
    let mut range = ranges[0].to_owned();

    loop {
        let mut indexes_to_remove: Vec<usize> = vec![];
        for index in indexes.iter() {
            let curr_range = &ranges[*index];
            if range.contains(curr_range.start()) {
                indexes_to_remove.push(*index);
                if !range.contains(curr_range.end()) {
                    let new_range = *range.start()..=*curr_range.end();
                    range = new_range;
                }
            } else if curr_range.contains(range.start()) {
                indexes_to_remove.push(*index);
                if !curr_range.contains(range.end()) {
                    let new_range = *curr_range.start()..=*range.end();
                    range = new_range;
                } else {
                    range = curr_range.to_owned();
                }
            }
        }
        if indexes_to_remove.is_empty() {
            let mut remainder = indexes
                .iter()
                .map(|i| ranges[*i].to_owned())
                .collect::<Vec<RangeInclusive<i64>>>();
            return if remainder.len() < 2 {
                (range, remainder)
            } else {
                remainder.push(range);
                combine_ranges(&remainder)
            };
        } else {
            indexes_to_remove.iter().rev().for_each(|i| {
                indexes.remove(i);
            });
        }
    }
}

fn string_to_sensor_info(input: &str) -> SensorInfo {
    let parts: Vec<Vec<i64>> = input
        .replace("Sensor at ", "")
        .replace(" closest beacon is at ", "")
        .split(":")
        .map(|part| {
            part.replace("x=", "")
                .replace(" y=", "")
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    ((parts[0][0], parts[0][1]), (parts[1][0], parts[1][1]))
}
