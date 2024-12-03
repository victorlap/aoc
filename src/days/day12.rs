use crate::{Solution, SolutionPair};

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day12.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let mut field = parse_field(input);
    let start = find_elevation(&field, -14);
    let end = find_elevation(&field, -28);
    replace_start_end_field(&mut field, start, end);

    find_distance(&field, start, end).unwrap() as u64
}

fn sol2(input: &Vec<String>) -> u64 {
    let mut field = parse_field(input);
    let start = find_elevation(&field, -14);
    let end = find_elevation(&field, -28);
    replace_start_end_field(&mut field, start, end);

    let mut shortest: i32 = i32::MAX;
    let distances = find_distances(&field, end, is_allowed_route_down);

    let a = parse_elevation('a');
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] != a {
                continue;
            }

            let cur: Coordinate = (i as i32, j as i32);
            if let Some(distance) = distances.get(&cur) {
                if distance < &shortest {
                    shortest = *distance;
                }
            }
        }
    }

    shortest as u64
}

///////////////////////////////////////////////////////////////////////////////

type Field = Vec<Vec<i32>>;
type Coordinate = (i32, i32);

fn parse_field(input: &Vec<String>) -> Field {
    input
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|char| parse_elevation(char))
                .collect::<Vec<i32>>()
        })
        .collect::<Field>()
}

fn replace_start_end_field(field: &mut Field, start: Coordinate, end: Coordinate) {
    field[start.0 as usize][start.1 as usize] = parse_elevation('a');
    field[end.0 as usize][end.1 as usize] = parse_elevation('z');
}

fn parse_elevation(elevation: char) -> i32 {
    elevation as i32 - 'a' as i32
}

fn find_elevation(field: &Field, elevation: i32) -> Coordinate {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == elevation {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("Elevation not found!")
}

fn is_out_of_bounds(field: &Field, coordinate: Coordinate) -> bool {
    coordinate.0 < 0
        || coordinate.1 < 0
        || coordinate.0 >= field.len() as i32
        || coordinate.1 >= field[0].len() as i32
}

fn is_allowed_route_up(field: &Field, start: Coordinate, end: Coordinate) -> bool {
    field[end.0 as usize][end.1 as usize] - field[start.0 as usize][start.1 as usize] <= 1
}

fn is_allowed_route_down(field: &Field, start: Coordinate, end: Coordinate) -> bool {
    field[start.0 as usize][start.1 as usize] - field[end.0 as usize][end.1 as usize] <= 1
}

fn find_distance(field: &Field, start: Coordinate, end: Coordinate) -> Option<i32> {
    let distances = find_distances(field, start, is_allowed_route_up);
    distances.get(&end).copied()
}

fn find_distances(
    field: &Field,
    start: Coordinate,
    is_allowed_route: fn(&Field, Coordinate, Coordinate) -> bool,
) -> HashMap<Coordinate, i32> {
    let possible_dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut distances: HashMap<Coordinate, i32> = HashMap::new();
    let mut paths: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();
    let mut queue: VecDeque<Coordinate> = VecDeque::new();
    queue.push_back(start);
    distances.insert(start, 0);

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            let mut map = Vec::new();
            let cur: Coordinate = (i as i32, j as i32);

            for dir in possible_dirs {
                let neighbor: Coordinate = (cur.0 + dir.0, cur.1 + dir.1);
                if is_out_of_bounds(&field, neighbor) {
                    continue;
                }
                if !is_allowed_route(&field, cur, neighbor) {
                    continue;
                }
                map.push(neighbor);
            }

            paths.insert(cur, map);
        }
    }

    while let Some(cur) = queue.pop_front() {
        if let Some(neighbors) = paths.get(&cur) {
            let cur_distance = distances.get(&cur).unwrap().clone();
            for neighbor in neighbors.into_iter() {
                let neighbor_distance = distances.get(&neighbor);
                // println!("From {:?} to {:?}: {:?}", cur, neighbor, neighbor_distance);

                if neighbor_distance == None || neighbor_distance.unwrap() > &(cur_distance + 1) {
                    distances.insert(*neighbor, cur_distance + 1);
                    queue.push_back(*neighbor);
                }
            }
        }
    }

    distances
}
