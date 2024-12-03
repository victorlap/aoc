use crate::{Solution, SolutionPair};

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day05.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: String = sol1(&parsed);
    let sol2: String = sol2(&parsed);

    (Solution::Str(sol1), Solution::Str(sol2))
}

fn sol1(input: &Vec<String>) -> String {
    let mut crates = get_crates(&input);

    for line in input {
        if !line.starts_with("move") {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();

        move_stack(
            &mut crates,
            parts[1].parse().unwrap(),
            parts[3].parse().unwrap(),
            parts[5].parse().unwrap(),
        );
    }
    get_top_row(&crates)
}

fn sol2(input: &Vec<String>) -> String {
    let mut crates = get_crates(&input);

    for line in input {
        if !line.starts_with("move") {
            continue;
        }
        let parts: Vec<&str> = line.split(" ").collect();

        move_stack_at_once(
            &mut crates,
            parts[1].parse().unwrap(),
            parts[3].parse().unwrap(),
            parts[5].parse().unwrap(),
        );
    }
    get_top_row(&crates)
}

///////////////////////////////////////////////////////////////////////////////

fn get_crates(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<char>> = vec![];

    for line in input {
        let mut index = 0;
        for content in line.chars().collect::<Vec<char>>().chunks(4) {
            if content[0] == '[' && content[1] != ' ' {
                if index as usize >= crates.len() {
                    crates.resize(index as usize + 1, vec![]);
                }
                crates[index as usize].push(content[1])
            }
            index += 1;
        }

        if line == "" {
            break;
        }
    }
    crates
}

fn move_stack(crates: &mut Vec<Vec<char>>, qty: u8, from: usize, to: usize) {
    for _i in 0..qty {
        let moved = crates[from - 1].remove(0);
        crates[to - 1].insert(0, moved);
    }
}

fn move_stack_at_once(crates: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize) {
    for i in 0..qty {
        let moved = crates[from - 1].remove(qty - i - 1);
        crates[to - 1].insert(0, moved);
    }
}

fn get_top_row(crates: &Vec<Vec<char>>) -> String {
    crates
        .iter()
        .map(|c| c.first().unwrap().to_string())
        .reduce(|cur, nxt| cur + &nxt)
        .unwrap()
}
