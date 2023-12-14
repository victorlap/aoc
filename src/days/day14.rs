use crate::{Solution, SolutionPair};
use itertools::Either;
use std::fs::read_to_string;
use std::time::Instant;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day14.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    let grid = parse_grid(input);
    let moved = move_grid(&grid, Direction::NORTH);

    calculate_load(&moved)
}

fn sol2(input: &Vec<String>) -> u64 {
    let grid = parse_grid(input);

    // assume cycle happens early
    let moved = spin(&grid, 1_000);

    calculate_load(&moved)
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    EMPTY,
    ROUNDED,
    CUBE,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::EMPTY,
            'O' => Cell::ROUNDED,
            '#' => Cell::CUBE,
            _ => panic!("Cannot convert {} to Cell", value),
        }
    }
}

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn parse_grid(input: &Vec<String>) -> Vec<Vec<Cell>> {
    input
        .into_iter()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn move_grid(input: &Vec<Vec<Cell>>, dir: Direction) -> Vec<Vec<Cell>> {
    let mut grid = input.clone();

    let (change_y, change_x) = match dir {
        Direction::NORTH => (-1, 0),
        Direction::WEST => (0, -1),
        Direction::SOUTH => (1, 0),
        Direction::EAST => (0, 1),
    };

    let y_iter = if change_y < 0 {
        Either::Left(0..grid.len())
    } else {
        Either::Right((0..grid.len()).rev())
    };

    for y in y_iter {
        let x_iter = if change_x < 0 {
            Either::Left(0..grid[y].len())
        } else {
            Either::Right((0..grid[y].len()).rev())
        };

        for x in x_iter {
            if grid[y][x] != Cell::ROUNDED {
                continue;
            }

            let (mut new_y, mut new_x) = (y as i64, x as i64);

            for i in 1.. {
                let (next_y, next_x) = (
                    y as i64 + (change_y * i) as i64,
                    x as i64 + (change_x * i) as i64,
                );

                if next_y < 0
                    || next_x < 0
                    || next_y as usize >= grid.len()
                    || next_x as usize >= grid[0].len()
                {
                    break;
                }

                if grid[next_y as usize][next_x as usize] != Cell::EMPTY {
                    break;
                }

                (new_y, new_x) = (next_y, next_x);
            }

            if new_y != y as i64 || new_x != x as i64 {
                grid[new_y as usize][new_x as usize] = Cell::ROUNDED;
                grid[y][x] = Cell::EMPTY;
            }
        }
    }

    grid
}

fn spin(input: &Vec<Vec<Cell>>, times: usize) -> Vec<Vec<Cell>> {
    let mut grid = input.clone();
    let now = Instant::now();

    for i in 0..times {
        if i % (times / 100) == 1 {
            // println!(
            //     "spinning: {}% in {}s, {}s total",
            //     i * 100 / times,
            //     now.elapsed().as_secs(),
            //     now.elapsed().as_secs() * times as u64 / i as u64
            // );
        }

        for dir in [
            Direction::NORTH,
            Direction::WEST,
            Direction::SOUTH,
            Direction::EAST,
        ] {
            grid = move_grid(&grid, dir);
        }
    }

    grid
}

fn calculate_load(grid: &Vec<Vec<Cell>>) -> u64 {
    let amount = grid.len() as u64;
    let mut res = 0;

    for (i, line) in grid.iter().enumerate() {
        res += line.iter().filter(|c| c == &&Cell::ROUNDED).count() as u64 * (amount - i as u64);
    }

    res
}

fn _print_grid(grid: &Vec<Vec<Cell>>) {
    for line in grid.iter() {
        for cell in line.iter() {
            match cell {
                Cell::EMPTY => print!("."),
                Cell::ROUNDED => print!("O"),
                Cell::CUBE => print!("#"),
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::days::day14;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day14::solve();

        assert_eq!(result.0, Solution::U64(107142));
        assert_eq!(result.1, Solution::U64(104815));
    }
}
