use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::str::FromStr;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day02.txt").unwrap();
    let parsed: Vec<Game> = input.lines().map(|line| line.parse().unwrap()).collect();

    (Solution::U64(sol1(&parsed)), Solution::U64(sol2(&parsed)))
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Game {
    id: u64,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    draw: Vec<Set>,
}

type Set = (u64, Color);

#[derive(Debug, PartialEq)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let semicolon = input.find(": ").unwrap();
        let id: u64 = input[5..semicolon].parse().unwrap();
        let draws = parse_draws(&input[semicolon + 2..input.len()]);

        Ok(Game { id, draws })
    }
}

impl Game {
    fn find_max(&self) -> (u64, u64, u64) {
        (
            self.find_max_color(Color::RED),
            self.find_max_color(Color::GREEN),
            self.find_max_color(Color::BLUE),
        )
    }

    fn find_max_color(&self, color: Color) -> u64 {
        self.draws.iter().fold(0, |res, draw| {
            let max = draw.find_max_color(&color);
            if max > res {
                max
            } else {
                res
            }
        })
    }
}

impl Draw {
    fn find_max_color(&self, color: &Color) -> u64 {
        self.draw.iter().fold(
            0,
            |res, set| if set.1 == *color { res + set.0 } else { res },
        )
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::RED),
            "green" => Ok(Color::GREEN),
            "blue" => Ok(Color::BLUE),
            _ => {
                println!("not found {}", input);
                Err(())
            }
        }
    }
}

fn parse_draws(input: &str) -> Vec<Draw> {
    input.split("; ").map(|draw| parse_draw(draw)).collect()
}

fn parse_draw(input: &str) -> Draw {
    let draw = input.split(", ").map(|draw| parse_set(draw)).collect();

    Draw { draw }
}

fn parse_set(input: &str) -> Set {
    let parts: Vec<&str> = input.split(" ").collect();
    let number: u64 = parts[0].parse().unwrap();
    let color: Color = parts[1].parse().unwrap();

    (number, color)
}

fn sol1(input: &Vec<Game>) -> u64 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    input
        .into_iter()
        .filter(|game| {
            let max = game.find_max();
            // println!("max: game {}, {} {} {}", game.id, max.0, max.1, max.2);
            max.0 <= 12 && max.1 <= 13 && max.2 <= 14
        })
        .fold(0, |res, g| {
            // println!("possible: {}", g.id);
            res + g.id
        })
}

fn sol2(input: &Vec<Game>) -> u64 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    input.into_iter().fold(0, |res, game| {
        let max: (u64, u64, u64) = game.find_max();
        res + (max.0 * max.1 * max.2)
    })
}

#[cfg(test)]
mod tests {
    use crate::day02;
    use crate::Solution;

    #[test]
    fn solve() {
        let result = day02::solve();

        assert_eq!(result.0, Solution::U64(2551));
        assert_eq!(result.1, Solution::U64(62811));
    }
}
