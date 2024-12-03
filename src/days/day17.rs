use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day17.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U32(sol1(&parsed)), Solution::U32(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u32 {
    let grid = parse_grid(input);

    find_path(&grid, 0, 3)
}

fn sol2(input: &Vec<String>) -> u32 {
    let grid = parse_grid(input);

    find_path(&grid, 4, 10)
}

///////////////////////////////////////////////////////////////////////////////

type Coordinate = (usize, usize);
type Direction = (isize, isize);
type Edge = (Coordinate, Direction, u32);

fn parse_grid(input: &Vec<String>) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_path(grid: &Vec<Vec<u32>>, min_straight: u32, max_straight: u32) -> u32 {
    let start: Coordinate = (0, 0);
    let possible_dirs: Vec<Direction> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue: VecDeque<Edge> = VecDeque::new();
    let mut distances: HashMap<Edge, u32> = HashMap::new();
    queue.push_back((start, (0, 0), min_straight));
    distances.insert((start, (0, 0), 0), 0);

    while let Some((cur, prev_dir, path_length)) = queue.pop_front() {
        let cur_distance = distances
            .get(&(cur, prev_dir, path_length))
            .unwrap_or(&0)
            .clone();

        let dirs = if path_length >= min_straight {
            possible_dirs.clone()
        } else {
            vec![prev_dir]
        };

        for dir in dirs {
            let straight_path_length = if dir == prev_dir { path_length + 1 } else { 1 };
            if (cur.0 as isize + dir.0) < 0
                || (cur.0 as isize + dir.0) > grid.len() as isize - 1
                || (cur.1 as isize + dir.1) < 0
                || (cur.1 as isize + dir.1) > grid[0].len() as isize - 1
                || straight_path_length > max_straight
                || ((prev_dir.0 + dir.0 == 0) && (prev_dir.1 + dir.1 == 0))
            {
                continue;
            }
            let neighbor: Coordinate = (
                (cur.0 as isize + dir.0) as usize,
                (cur.1 as isize + dir.1) as usize,
            );

            let old_distance = distances
                .get(&(neighbor, dir, straight_path_length))
                .unwrap_or(&u32::MAX)
                .clone();
            let new_distance = cur_distance + grid[neighbor.0][neighbor.1];

            if new_distance < old_distance {
                distances.insert((neighbor, dir, straight_path_length), new_distance);
                queue.push_back((neighbor, dir, straight_path_length));
            }
        }
    }

    let end = (grid.len() - 1, grid[0].len() - 1);

    distances
        .iter()
        .filter(|((c, _, _), _)| *c == end)
        .map(|(_, d)| *d)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::days::day17;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day17::solve();

        assert_eq!(result.0, Solution::U32(1246));
        assert_eq!(result.1, Solution::U32(1389));
    }
}
