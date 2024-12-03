use crate::{Solution, SolutionPair};

use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day08.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let trees = parse_trees(input);

    visible_trees(&trees)
}

fn sol2(input: &Vec<String>) -> u64 {
    let trees = parse_trees(input);

    let mut scenic_scores = scenic_scores(&trees);
    scenic_scores.sort();
    scenic_scores.reverse();
    scenic_scores[0]
}

///////////////////////////////////////////////////////////////////////////////

type Coordinates = (i8, i8);

fn parse_trees(input: &Vec<String>) -> Vec<Vec<i8>> {
    input
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>()
}

fn scenic_score(trees: &Vec<Vec<i8>>, coords: Coordinates) -> u64 {
    let mut scores: Vec<u64> = vec![0; 4];
    let current_height = trees[coords.0 as usize][coords.1 as usize];

    for i in (0..coords.0).rev() {
        scores[0] += 1;
        if trees[i as usize][coords.1 as usize] >= current_height {
            break;
        }
    }

    for i in coords.0 + 1..trees.len() as i8 {
        scores[1] += 1;
        if trees[i as usize][coords.1 as usize] >= current_height {
            break;
        }
    }

    for j in (0..coords.1).rev() {
        scores[2] += 1;
        if trees[coords.0 as usize][j as usize] >= current_height {
            break;
        }
    }
    for j in coords.1 + 1..trees.len() as i8 {
        scores[3] += 1;
        if trees[coords.0 as usize][j as usize] >= current_height {
            break;
        }
    }

    scores
        .into_iter()
        .reduce(|score, prev| prev * score)
        .unwrap()
}

fn scenic_scores(trees: &Vec<Vec<i8>>) -> Vec<u64> {
    // println!("{}", matrix_to_string(&trees));

    let mut scores = vec![];
    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            scores.push(scenic_score(trees, (i as i8, j as i8)))
        }
    }
    scores
}

fn visible_trees(trees: &Vec<Vec<i8>>) -> u64 {
    // println!("{}", matrix_to_string(&trees));

    let mut result: HashMap<Coordinates, i8> = HashMap::new();
    for i in 0..trees.len() {
        // left to right
        let mut highest_tree = -1;
        for j in 0..trees[i].len() {
            let tree = trees[i][j];
            if tree > highest_tree {
                result.insert((i as i8, j as i8), 1);
                highest_tree = tree;
            }
        }

        // right to left
        highest_tree = -1;
        for j in (0..trees[i].len()).rev() {
            let tree = trees[i][j];
            if tree > highest_tree {
                result.insert((i as i8, j as i8), 1);
                highest_tree = tree;
            }
        }
    }
    for j in 0..trees[0].len() {
        // top to bottom
        let mut highest_tree = -1;
        for i in 0..trees.len() {
            let tree = trees[i][j];
            if tree > highest_tree {
                result.insert((i as i8, j as i8), 1);
                highest_tree = tree;
            }
        }

        // bottom to top
        highest_tree = -1;
        for i in (0..trees.len()).rev() {
            let tree = trees[i][j];

            if tree > highest_tree {
                result.insert((i as i8, j as i8), 1);
                highest_tree = tree;
            }
        }
    }

    // println!("{}", matrix_to_string(&coordinates_to_matrix(&result)));
    result.len() as u64
}

fn _matrix_to_string(m: &Vec<Vec<i8>>) -> String {
    m.iter().fold("".to_string(), |a, r| {
        a + &r
            .iter()
            .fold("".to_string(), |b, e| b + "\t" + &e.to_string())
            + "\n"
    })
}

fn _coordinates_to_matrix(result: &HashMap<Coordinates, i8>) -> Vec<Vec<i8>> {
    let len = 5;
    let mut matrix: Vec<Vec<i8>> = vec![vec![0; len]; len];

    for item in result.keys() {
        matrix[item.0 as usize][item.1 as usize] = 1;
    }

    matrix
}
