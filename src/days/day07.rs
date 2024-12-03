use crate::{Solution, SolutionPair};
use std::collections::HashMap;

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day07.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let tree = get_directory_tree(input);

    tree.iter()
        .map(|(_k, v)| *v)
        .filter(|size| size <= &(100000 as u64))
        .sum()
}

fn sol2(input: &Vec<String>) -> u64 {
    let tree = get_directory_tree(input);

    let total_size = 70000000;
    let space_needed = 30000000;
    let root_size = tree.get(&"/".to_string()).unwrap();
    let space_free = total_size - root_size;
    let space_to_delete = space_needed - space_free;

    let mut dirs_to_delete: Vec<u64> = tree
        .iter()
        .map(|(_k, v)| *v)
        .filter(|size| size >= &(space_to_delete as u64))
        .collect();

    dirs_to_delete.sort();
    dirs_to_delete[0]
}

///////////////////////////////////////////////////////////////////////////////

fn get_directory_tree(commands: &Vec<String>) -> HashMap<String, u64> {
    let mut tree: HashMap<String, u64> = HashMap::new();
    let mut current: Vec<String> = vec![String::from("")];

    for command in commands {
        let command_str = command.as_str();
        if command_str == "$ cd /" {
            current = vec![String::from("")];
        } else if command_str == "$ cd .." {
            current.pop();
        } else if command_str.starts_with("$ cd ") {
            let subfolder = command_str.replace("$ cd ", "");
            current.push(subfolder);
        } else if command_str == "$ ls" {
            // no op
        } else {
            // we have a directory listing, add the files to the
            let split: Vec<&str> = command_str.split(" ").collect();
            if split[0] == "dir" {
                continue;
            }
            let size: u64 = split[0].parse().unwrap();
            let _name: &str = split[1];

            let mut path = String::new();
            for folder in &current {
                path += &(folder.to_owned() + "/");
                let cur = *tree.entry(path.to_string()).or_insert(0);
                tree.insert(path.to_string(), cur + size);
                // let tmp = tree.entry(path).or_insert(0);
                // tmp += size;
            }
        }
    }

    tree
}
