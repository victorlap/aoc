use std::collections::HashMap;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day10.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<String>) -> u64 {
    let grid = parse_grid(input);

    let start_pos = find_start_pos(&grid);
    let main_loop = find_main_loop(&grid, (start_pos.0 + 1, start_pos.1));

    (main_loop.len() as u64) / 2
}

fn sol2(input: &Vec<String>) -> u64 {
    let grid = parse_grid(input);

    let start_pos = find_start_pos(&grid);
    let main_loop = find_main_loop(&grid, (start_pos.0 + 1, start_pos.1));

    // let mapped = check_grid_enclosed(&clean_grid(&grid, &main_loop), &main_loop);
    let mapped = check_grid_enclosed(&grid, &main_loop);

    // print_grid(&mapped);

    mapped.iter().fold(0, |acc, row| acc + row.iter().filter(|cell| **cell == 'I').count()) as u64
}

///////////////////////////////////////////////////////////////////////////////

fn parse_grid(input: &Vec<String>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn find_start_pos(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return (x, y);
            }
        }
    }

    panic!("No start position found!");
}

fn find_main_loop(grid: &Vec<Vec<char>>, start_pos: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut pos = start_pos;

    let mut main_loop = HashMap::new();
    let mut prev_dir = 'R';
    let mut steps = 1;

    loop {
        main_loop.insert(pos, steps);

        let (x, y) = pos;
        let cell = grid[y][x];

        if cell == 'S' {
            break;
        }

        let dir = find_dir(cell, prev_dir);

        match dir {
            'U' => pos.1 -= 1,
            'D' => pos.1 += 1,
            'L' => pos.0 -= 1,
            'R' => pos.0 += 1,
            _ => panic!("Invalid direction: {}", dir),
        }

        prev_dir = dir;
        steps += 1;
    }

    main_loop
}

fn find_dir(cell: char, prev_dir: char) -> char {
    match cell {
        'F' => {
            match prev_dir {
                'L' => 'D',
                'U' => 'R',
                _ => panic!("Invalid direction: {}", prev_dir),
            }
        }
        '7' => {
            match prev_dir {
                'R' => 'D',
                'U' => 'L',
                _ => panic!("Invalid direction: {}", prev_dir),
            }
        }
        'L' => {
            match prev_dir {
                'D' => 'R',
                'L' => 'U',
                _ => panic!("Invalid direction: {}", prev_dir),
            }
        }
        'J' => {
            match prev_dir {
                'R' => 'U',
                'D' => 'L',
                _ => panic!("Invalid direction: {}", prev_dir),
            }
        }
        '|' => prev_dir,
        '-' => prev_dir,
        _ => panic!("Invalid cell: {}", cell),
    }
}

fn check_grid_enclosed(input: &Vec<Vec<char>>, main_loop: &HashMap<(usize, usize), usize>) -> Vec<Vec<char>> {
    let mut grid = input.clone();
    let mut windings = 0;

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if let Some(step) = main_loop.get(&(x, y)) {
                if let Some(step_below) = main_loop.get(&(x, y + 1)) {
                    let diff = *step_below as i64 - *step as i64;
                    if diff == 1 {
                        windings += 1;
                    }
                    if diff == -1 {
                        windings -= 1;
                    }
                }
            } else {
                if windings > 0 {
                    *cell = 'I';
                }
            }
        }
    }

    grid
}

fn _clean_grid(input: &Vec<Vec<char>>, main_loop: &HashMap<(usize, usize), usize>) -> Vec<Vec<char>> {
    let mut grid = input.clone();

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let in_main = main_loop.get(&(x, y)).is_some();
            if !in_main {
                *cell = '.';
            }
        }
    }

    grid
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for line in grid.iter() {
        println!("{}", line.iter().map(|c| {
            match c {
                'F' => '┌',
                '7' => '┐',
                'L' => '└',
                'J' => '┘',
                '|' => '│',
                '-' => '─',
                _ => *c,
            }
        }).collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day10::solve();

        assert_eq!(result.0, Solution::U64(6725));
        assert_eq!(result.1, Solution::U64(383));
    }
}
