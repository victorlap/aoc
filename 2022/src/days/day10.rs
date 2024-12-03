use crate::{Solution, SolutionPair};

use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day10.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: String = sol2(&parsed);

    (Solution::U64(sol1), Solution::Str(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let instructions = parse_input(&input);

    let mut register = 1;
    let mut cycle = 0;
    let mut sum = 0;

    for instruction in instructions {
        let adding = match instruction.0 {
            Instruction::Noop => 1,
            Instruction::Addx => 2,
        };

        for _ in 0..adding {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                sum += cycle * register;
            }
        }

        register += instruction.1;
    }

    sum as u64
}

fn sol2(input: &Vec<String>) -> String {
    let instructions = parse_input(&input);

    let mut register = 1;
    let mut cycle = 0;
    let mut output = String::from("\n");

    for instruction in instructions {
        let adding = match instruction.0 {
            Instruction::Noop => 1,
            Instruction::Addx => 2,
        };

        for _ in 0..adding {
            if ((cycle % 40) == register - 1)
                || ((cycle % 40) == register)
                || ((cycle % 40) == register + 1)
            {
                output += "█";
            } else {
                output += " ";
            }
            cycle += 1;

            if cycle % 40 == 0 {
                output += "\n";
            }
        }

        register += instruction.1;
    }
    output
}

///////////////////////////////////////////////////////////////////////////////

type Input = (Instruction, i32);

#[derive(Debug, PartialEq)]
enum Instruction {
    Addx,
    Noop,
}

fn parse_input(input: &Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|line| match line == &"noop".to_string() {
            true => (Instruction::Noop, 0),
            false => (
                Instruction::Addx,
                line.replace("addx ", "").parse().unwrap(),
            ),
        })
        .collect()
}
