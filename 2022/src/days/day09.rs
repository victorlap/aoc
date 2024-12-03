use crate::{Solution, SolutionPair};

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day09.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    count_visits(input, &mut vec![(0, 0); 2])
}

fn sol2(input: &Vec<String>) -> u64 {
    count_visits(input, &mut vec![(0, 0); 10])
}

///////////////////////////////////////////////////////////////////////////////

type Coordinates = (i32, i32);

fn update_position(rope: &mut Vec<Coordinates>) {
    for i in 1..rope.len() {
        let (mut x, mut y) = rope[i];
        let (px, py) = rope[i - 1];

        if (x - px).abs() > 1 || (y - py).abs() > 1 {
            // println!("{:?}", (px, py, x, y));
            if x > px {
                x -= 1
            }
            if x < px {
                x += 1
            }
            if y > py {
                y -= 1
            }
            if y < py {
                y += 1
            }

            rope[i] = (x, y);
        }
    }
}

fn count_visits(input: &Vec<String>, rope: &mut Vec<Coordinates>) -> u64 {
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let directions: HashMap<String, Coordinates> = HashMap::from([
        ("L".to_string(), (-1, 0)),
        ("R".to_string(), (1, 0)),
        ("U".to_string(), (0, 1)),
        ("D".to_string(), (0, -1)),
    ]);

    // println!("{:?}", rope);
    for instruction in input {
        let line: Vec<&str> = instruction.split(" ").collect();
        // println!("{:?}", line);

        let dir: String = line[0].parse().unwrap();
        let qty: usize = line[1].parse().unwrap();

        for _i in 0..qty {
            rope[0].0 += directions[&dir].0;
            rope[0].1 += directions[&dir].1;
            update_position(rope);
            visited.insert(rope[rope.len() - 1]);

            // println!("{:?}", rope);
        }
    }
    visited.len() as u64
}
