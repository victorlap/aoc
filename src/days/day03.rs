use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day03.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

///////////////////////////////////////////////////////////////////////////////

type GearLoc = (isize, isize);

fn sol1(input: &Vec<String>) -> u64 {
    let mut prevline;
    let mut line = &String::new();
    let mut nextline = &String::new();

    let mut i = 0;

    let mut total = 0;

    while i < input.len() {
        prevline = line;
        line = input.get(i).unwrap();
        if i + 1 < input.len() {
            nextline = input.get(i + 1).unwrap();
        }

        let mut number = String::new();

        let mut in_number = false;
        let mut discovered_char = false;

        for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                in_number = true;
                number.push(char);

                if !discovered_char && surrounding_contains_char(index, prevline, line, nextline) {
                    discovered_char = true;
                }
            } else {
                if in_number {
                    if discovered_char {
                        total += number.parse::<u64>().unwrap();
                    }
                    in_number = false;
                    discovered_char = false;
                    number = String::new();
                }
            }
        }

        // check for last number in line
        if in_number && discovered_char {
            total += number.parse::<u64>().unwrap();
        }

        i += 1;
    }

    total
}

fn sol2(input: &Vec<String>) -> u64 {
    let mut prevline;
    let mut line = &String::new();
    let mut nextline = &String::new();

    let mut i = 0;

    let mut gears: HashMap<GearLoc, Vec<u64>> = HashMap::new();

    while i < input.len() {
        prevline = line;
        line = input.get(i).unwrap();
        if i + 1 < input.len() {
            nextline = input.get(i + 1).unwrap();
        }

        let mut number = String::new();

        let mut in_number = false;
        let mut discovered_gear: Option<GearLoc> = None;

        for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                in_number = true;
                number.push(char);

                if discovered_gear.is_none() {
                    if let Some(new_gear) = get_gear_loc(index, prevline, line, nextline) {
                        discovered_gear =
                            Some((new_gear.0 + i as isize, new_gear.1 + index as isize));
                    }
                }
            } else {
                if in_number {
                    if let Some(gear) = discovered_gear {
                        let new_number = number.parse::<u64>().unwrap();

                        if let Some(old) = gears.get_mut(&gear) {
                            old.push(new_number);
                        } else {
                            gears.insert(gear, vec![new_number]);
                        }
                    }
                    in_number = false;
                    discovered_gear = None;
                    number = String::new();
                }
            }
        }

        // check for last number in line
        if in_number {
            if let Some(gear) = discovered_gear {
                let new_number = number.parse::<u64>().unwrap();

                if let Some(old) = gears.get_mut(&gear) {
                    old.push(new_number);
                } else {
                    gears.insert(gear, vec![new_number]);
                }
            }
        }

        i += 1;
    }

    gears.into_iter().fold(0, |acc, item| {
        if item.1.len() == 2 {
            return acc + item.1[0] * item.1[1];
        }
        acc
    })
}

fn surrounding_contains_char(
    index: usize,
    prevline: &String,
    line: &String,
    nextline: &String,
) -> bool {
    // left
    if index > 0 {
        if (prevline.len() >= index && is_char(&prevline[index - 1..index]))
            || (line.len() >= index && is_char(&line[index - 1..index]))
            || (nextline.len() >= index && is_char(&nextline[index - 1..index]))
        {
            return true;
        }
    }

    // center
    if (prevline.len() > index && is_char(&prevline[index..index + 1]))
        || (nextline.len() > index && is_char(&nextline[index..index + 1]))
    {
        return true;
    }

    // right
    if (prevline.len() > index + 1 && is_char(&prevline[index + 1..index + 2]))
        || (line.len() > index + 1 && is_char(&line[index + 1..index + 2]))
        || (nextline.len() > index + 1 && is_char(&nextline[index + 1..index + 2]))
    {
        return true;
    }

    return false;
}

fn is_char(input: &str) -> bool {
    let char = input.chars().next().unwrap();
    !char.is_numeric() && char != '.'
}

fn get_gear_loc(
    index: usize,
    prevline: &String,
    line: &String,
    nextline: &String,
) -> Option<GearLoc> {
    // left
    if index > 0 {
        if prevline.len() >= index && is_gear(&prevline[index - 1..index]) {
            return Some((-1, -1));
        }
        if line.len() >= index && is_gear(&line[index - 1..index]) {
            return Some((0, -1));
        }
        if nextline.len() >= index && is_gear(&nextline[index - 1..index]) {
            return Some((1, -1));
        }
    }

    // center
    if prevline.len() > index && is_gear(&prevline[index..index + 1]) {
        return Some((-1, 0));
    }
    if nextline.len() > index && is_gear(&nextline[index..index + 1]) {
        return Some((1, 0));
    }

    // right
    if prevline.len() > index + 1 && is_gear(&prevline[index + 1..index + 2]) {
        return Some((-1, 1));
    }
    if line.len() > index + 1 && is_gear(&line[index + 1..index + 2]) {
        return Some((0, 1));
    }
    if nextline.len() > index + 1 && is_gear(&nextline[index + 1..index + 2]) {
        return Some((1, 1));
    }

    return None;
}

fn is_gear(input: &str) -> bool {
    input == "*"
}

#[cfg(test)]
mod tests {
    use crate::day03;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day03::solve();

        assert_eq!(result.0, Solution::U64(514969));
        assert_eq!(result.1, Solution::U64(78915902));
    }
}
