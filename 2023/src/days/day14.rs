use crate::{Solution, SolutionPair};
use itertools::Either;
use std::fs::read_to_string;

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
    let mut grid = parse_grid(input);

    let (cycle, start) = detect_cycle(grid.clone());
    let point_in_cycle = (1_000_000_000 - start) % cycle;

    for _ in 0..start + point_in_cycle {
        grid = spin(grid);
    }

    calculate_load(&grid)
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

fn spin(input: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut grid = input.clone();

    for dir in [
        Direction::NORTH,
        Direction::WEST,
        Direction::SOUTH,
        Direction::EAST,
    ] {
        grid = move_grid(&grid, dir);
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

// https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
fn detect_cycle(start: Vec<Vec<Cell>>) -> (usize, usize) {
    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = start.clone();

    let mut hare = spin(start.clone()); //  f(x0) is the element/node next to x0.
    while tortoise != hare {
        if power == lam {
            // time to start a new power of two?
            tortoise = hare.clone();
            power *= 2;
            lam = 0;
        }
        hare = spin(hare.clone());
        lam += 1
    }

    // Find the position of the first repetition of length λ
    tortoise = start.clone();
    hare = start.clone();
    for _ in 0..lam {
        hare = spin(hare)
    }

    // The distance between the hare and tortoise is now λ.

    // Next, the hare and tortoise move at same speed until they agree
    let mut mu = 0;
    while tortoise != hare {
        tortoise = spin(tortoise);
        hare = spin(hare);
        mu += 1;
    }

    return (lam, mu);
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
