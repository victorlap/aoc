use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: String = read_to_string("src/input/day18.txt").unwrap();
    let parsed: Vec<Instruction> = input
        .lines()
        .map(|line| parse_instruction(line).unwrap())
        .collect();

    (Solution::I32(sol1(&parsed)), Solution::I64(sol2(&parsed)))
}

fn sol1(input: &Vec<Instruction>) -> i32 {
    let mut area = 0;
    let mut perim = 0;
    let mut pos = (0, 0);

    for edge in input {
        let prev_pos = pos.clone();

        match edge.0 {
            'R' => pos.0 += edge.1,
            'L' => pos.0 -= edge.1,
            'U' => pos.1 += edge.1,
            'D' => pos.1 -= edge.1,
            _ => panic!("Unknown direction"),
        }

        perim += edge.1;
        area += pos.0 * prev_pos.1 - prev_pos.0 * pos.1;
    }

    area.abs() / 2 + perim / 2 + 1
}

fn sol2(input: &Vec<Instruction>) -> i64 {
    let mut area = 0;
    let mut perim = 0;
    let mut pos = (0, 0);

    for edge in input {
        let prev_pos = pos.clone();
        let steps = i64::from_str_radix(&edge.2[0..5], 16).unwrap();

        match &edge.2[5..6] {
            "0" => pos.0 += steps,
            "1" => pos.1 -= steps,
            "2" => pos.0 -= steps,
            "3" => pos.1 += steps,
            _ => panic!("Unknown direction"),
        }

        perim += steps;
        area += pos.0 * prev_pos.1 - prev_pos.0 * pos.1;
    }

    area.abs() / 2 + perim / 2 + 1
}

///////////////////////////////////////////////////////////////////////////////

type Instruction = (char, i32, String);

fn parse_instruction(input: &str) -> Option<Instruction> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    Some((
        parts[0].chars().next().unwrap(),
        parts[1].to_string().parse().unwrap(),
        parts[2][2..8].to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::days::day18;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day18::solve();

        assert_eq!(result.0, Solution::I32(72821));
        assert_eq!(result.1, Solution::I64(127844509405501));
    }
}
