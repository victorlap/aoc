use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day13.txt").unwrap();
    let parsed: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

fn sol1(input: &Vec<Vec<char>>) -> u64 {
    let rooms = find_rooms(input);

    rooms.iter().map(|r| find_reflection(r, 0).unwrap()).sum()
}

fn sol2(input: &Vec<Vec<char>>) -> u64 {
    let rooms = find_rooms(input);

    rooms
        .iter()
        .map(|r| fix_smudge_and_find_reflection(r))
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

fn find_rooms(input: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut rooms = vec![];
    let mut room = vec![];

    for line in input {
        if line.len() != 0 {
            room.push(line.clone());
        } else {
            rooms.push(room);
            room = vec![];
        }
    }

    rooms.push(room);
    rooms
}

fn find_reflection(room: &Vec<Vec<char>>, skip: u64) -> Option<u64> {
    for y in 0..room.len() {
        if find_reflection_from_y(room, y) && (y + 1) as u64 * 100 != skip {
            return Some((y + 1) as u64 * 100);
        }
    }

    for x in 0..room[0].len() {
        if find_reflection_from_x(room, x) && (x + 1) as u64 != skip {
            return Some((x + 1) as u64);
        }
    }

    return None;
}

fn find_reflection_from_y(room: &Vec<Vec<char>>, y: usize) -> bool {
    let mut checked = false;

    for i in 0..=y {
        if (y as isize - i as isize) < 0 || y + 1 + i > room.len() - 1 {
            break;
        }
        if room[y - i] != room[y + 1 + i] {
            return false;
        }
        checked = true;
    }

    checked
}

fn find_reflection_from_x(room: &Vec<Vec<char>>, x: usize) -> bool {
    let mut checked = false;

    for line in room.iter() {
        for i in 0..=x {
            if (x as isize - i as isize) < 0 || x + 1 + i > line.len() - 1 {
                break;
            }
            if line[x - i] != line[x + 1 + i] {
                return false;
            }
            checked = true;
        }
    }

    checked
}

fn fix_smudge_and_find_reflection(room: &Vec<Vec<char>>) -> u64 {
    let orig = find_reflection(&room, 0).unwrap();

    for y in 0..room.len() {
        for x in 0..room[0].len() {
            let mut changed = room.clone();
            changed[y][x] = if changed[y][x] == '#' { '.' } else { '#' };

            let new_reflection = find_reflection(&changed, orig);
            if new_reflection.is_some() {
                return new_reflection.unwrap();
            }
        }
    }

    panic!("Cannot find smudge");
}

#[cfg(test)]
mod tests {
    use crate::days::day13;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day13::solve();

        assert_eq!(result.0, Solution::U64(33356));
        assert_eq!(result.1, Solution::U64(28475));
    }
}
