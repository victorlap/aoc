use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day11.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    let grid = &parse_grid(input);

    let galaxies = find_galaxies(&grid);
    let pairs = find_pairs(&galaxies);
    let distance = find_distance(&pairs, find_empty_spaces(&grid), 2);

    distance
}

fn sol2(input: &Vec<String>) -> u64 {
    let grid = &parse_grid(input);

    let galaxies = find_galaxies(&grid);
    let pairs = find_pairs(&galaxies);
    let distance = find_distance(&pairs, find_empty_spaces(&grid), 1_000_000);

    distance
}

///////////////////////////////////////////////////////////////////////////////

type Location = (usize, usize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    EMPTY,
    GALAXY,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::EMPTY,
            '#' => Cell::GALAXY,
            _ => panic!("Cannot convert {} to Cell", value),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Cell::EMPTY => ".",
            Cell::GALAXY => "#",
        };
        write!(f, "{}", str)
    }
}

fn parse_grid(input: &Vec<String>) -> Vec<Vec<Cell>> {
    input
        .iter()
        .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

fn find_empty_spaces(input: &Vec<Vec<Cell>>) -> (HashSet<usize>, HashSet<usize>) {
    let mut empty_rows: HashSet<usize> = (0..input.len()).into_iter().collect();
    let mut empty_cols: HashSet<usize> = (0..input[0].len()).into_iter().collect();

    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Cell::GALAXY {
                empty_rows.remove(&y);
                empty_cols.remove(&x);
            }
        }
    }

    (empty_rows, empty_cols)
}

fn find_galaxies(input: &Vec<Vec<Cell>>) -> HashSet<Location> {
    let mut res = HashSet::new();

    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Cell::GALAXY {
                res.insert((x, y));
            }
        }
    }

    res
}

fn find_pairs(galaxies: &HashSet<Location>) -> HashSet<(Location, Location)> {
    let mut res = HashSet::new();

    for (x, y) in galaxies.iter() {
        for (x2, y2) in galaxies.iter() {
            if x == x2 && y == y2 {
                continue;
            }

            if (*x, *y) > (*x2, *y2) {
                res.insert(((*x, *y), (*x2, *y2)));
            } else {
                res.insert(((*x2, *y2), (*x, *y)));
            }
        }
    }

    res
}

fn find_distance(
    pairs: &HashSet<(Location, Location)>,
    (empty_rows, empty_cols): (HashSet<usize>, HashSet<usize>),
    multiplier: u64,
) -> u64 {
    pairs.into_iter().fold(0, |acc, (a, b)| {
        let empty_rows_inbetween = empty_rows
            .iter()
            .filter(|&y| *y > a.1.min(b.1) && *y < a.1.max(b.1))
            .count() as u64;

        let empty_cols_inbetween = empty_cols
            .iter()
            .filter(|&x| *x > a.0.min(b.0) && *x < a.0.max(b.0))
            .count() as u64;

        let distance = a.0.abs_diff(b.0) as u64
            + empty_rows_inbetween * (multiplier - 1)
            + a.1.abs_diff(b.1) as u64
            + empty_cols_inbetween * (multiplier - 1);

        acc + distance
    })
}

fn _print_grid(grid: &Vec<Vec<Cell>>) {
    for line in grid.iter() {
        println!("{}", line.iter().map(|c| c.to_string()).collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day11;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day11::solve();

        assert_eq!(result.0, Solution::U64(9329143));
        assert_eq!(result.1, Solution::U64(710674907809));
    }
}
