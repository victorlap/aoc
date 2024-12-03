use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day16.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    let grid = parse_grid(input);

    check_beams(&grid, ((0, 0), Direction::Right))
}

fn sol2(input: &Vec<String>) -> u64 {
    let grid = parse_grid(input);
    let mut max = 0;
    let y_max = grid.len() - 1;
    let x_max = grid[0].len() - 1;

    for y in 0..=y_max {
        let from_left = check_beams(&grid, ((0, y as isize), Direction::Right));
        let from_right = check_beams(&grid, ((x_max as isize, y as isize), Direction::Left));

        max = max.max(from_left).max(from_right);
    }

    for x in 0..=x_max {
        let from_top = check_beams(&grid, ((x as isize, 0), Direction::Down));
        let from_bottom = check_beams(&grid, ((x as isize, y_max as isize), Direction::Up));

        max = max.max(from_top).max(from_bottom);
    }

    max
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Pos = (isize, isize);
type Beam = (Pos, Direction);

fn parse_grid(input: &Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn check_beams(grid: &Vec<Vec<char>>, start: Beam) -> u64 {
    let mut energized = HashSet::new();
    let mut beams: VecDeque<Beam> = VecDeque::new();
    let mut visited = HashSet::new();
    beams.push_back(start.clone());

    while let Some((beam_pos, beam_dir)) = beams.pop_front() {
        energized.insert(beam_pos);

        let next_dir = match grid[beam_pos.1 as usize][beam_pos.0 as usize] {
            '/' => match beam_dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            '\\' => match beam_dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            '-' => match beam_dir {
                Direction::Up | Direction::Down => {
                    // Add second beam
                    beams.push_back((beam_pos, Direction::Right));
                    Direction::Left
                }
                _ => beam_dir,
            },
            '|' => match beam_dir {
                Direction::Left | Direction::Right => {
                    // Add second beam
                    beams.push_back((beam_pos, Direction::Up));
                    Direction::Down
                }
                _ => beam_dir,
            },
            _ => beam_dir,
        };

        let next_pos: (isize, isize) = match next_dir {
            Direction::Up => (beam_pos.0, beam_pos.1 - 1),
            Direction::Down => (beam_pos.0, beam_pos.1 + 1),
            Direction::Left => (beam_pos.0 - 1, beam_pos.1),
            Direction::Right => (beam_pos.0 + 1, beam_pos.1),
        };

        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 > grid[0].len() as isize - 1
            || next_pos.1 > grid.len() as isize - 1
            || visited.contains(&(next_pos, next_dir.clone()))
        {
            continue;
        }

        beams.push_back((next_pos, next_dir.clone()));
        visited.insert((next_pos, next_dir));
    }

    energized.len() as u64
}

#[cfg(test)]
mod tests {
    use crate::days::day16;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day16::solve();

        assert_eq!(result.0, Solution::U64(7236));
        assert_eq!(result.1, Solution::U64(7521));
    }
}
