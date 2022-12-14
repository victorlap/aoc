use crate::{Solution, SolutionPair};

use std::{collections::HashMap, fs::read_to_string, ops::RangeInclusive};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("src/input/day14.txt").unwrap();
    let parsed: Vec<String> = input.lines().map(|line| line.parse().unwrap()).collect();

    let sol1: u64 = sol1(&parsed);
    let sol2: u64 = sol2(&parsed);

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn sol1(input: &Vec<String>) -> u64 {
    let mut field = parse_field(&input);
    let bounds = get_bounds(&field);

    loop {
        let result = spawn_sand(&mut field, &bounds);

        if !result {
            break;
        }
    }

    print_field(&field, &bounds);

    field
        .into_iter()
        .map(|(_, tile)| if tile == Tile::Sand { 1 } else { 0 })
        .sum()
}

fn sol2(input: &Vec<String>) -> u64 {
    let mut field = parse_field(&input);
    let mut bounds = get_bounds(&field);
    add_floor(&mut field, &mut bounds);

    loop {
        let result = spawn_sand(&mut field, &bounds);

        if !result {
            break;
        }
    }

    print_field(&field, &bounds);

    field
        .into_iter()
        .map(|(_, tile)| if tile == Tile::Sand { 1 } else { 0 })
        .sum()
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
enum Tile {
    Rock,
    Air,
    Sand,
    SandSpawn,
}

type Coordinate = (i32, i32);
type Field = HashMap<Coordinate, Tile>;
type Bounds = (i32, i32, i32, i32);

fn parse_field(input: &Vec<String>) -> Field {
    let mut field: Field = HashMap::new();
    field.insert((500, 0), Tile::SandSpawn);

    for line in input {
        let parts: Vec<Coordinate> = line
            .split(" -> ")
            .map(|part| string_to_coordinate(part))
            .collect();

        for coordinates in parts.windows(2) {
            for i in tile_range(coordinates[0].0, coordinates[1].0) {
                for j in tile_range(coordinates[0].1, coordinates[1].1) {
                    field.insert((i, j), Tile::Rock);
                }
            }
        }
    }
    field
}

fn get_bounds(field: &Field) -> Bounds {
    let mut smallest_x = i32::MAX;
    let mut smallest_y = i32::MAX;
    let mut biggest_x = 0;
    let mut biggest_y = 0;

    for (coordinate, _) in field {
        if coordinate.0 < smallest_x {
            smallest_x = coordinate.0;
        }
        if coordinate.0 > biggest_x {
            biggest_x = coordinate.0;
        }
        if coordinate.1 < smallest_y {
            smallest_y = coordinate.1;
        }
        if coordinate.1 > biggest_y {
            biggest_y = coordinate.1;
        }
    }
    (smallest_x, biggest_x, smallest_y, biggest_y)
}

fn string_to_coordinate(input: &str) -> Coordinate {
    let parts: Vec<i32> = input.split(',').map(|line| line.parse().unwrap()).collect();
    (parts[0], parts[1])
}

fn print_field(field: &Field, bounds: &Bounds) {
    for i in bounds.2..=bounds.3 {
        for j in bounds.0..=bounds.1 {
            print!(
                "{}",
                tile_to_string(field.get(&(j, i)).or(Some(&Tile::Air)).unwrap())
            );
        }
        println!()
    }
    println!()
}

fn spawn_sand(field: &mut Field, bounds: &Bounds) -> bool {
    let mut coord = (500, 0);

    loop {
        if coord.0 < bounds.0 || coord.0 > bounds.1 || coord.1 > bounds.3 {
            // Out of bounds!
            return false;
        } else if field.get(&(coord.0, coord.1 + 1)) == None {
            // No tile, fall down!
            coord.1 += 1;
        } else if field.get(&(coord.0 - 1, coord.1 + 1)) == None {
            // No tile, fall down left!
            coord.0 -= 1;
            coord.1 += 1;
        } else if field.get(&(coord.0 + 1, coord.1 + 1)) == None {
            // No tile, fall down right!
            coord.0 += 1;
            coord.1 += 1;
        } else if field.get(&(coord.0, coord.1)) != None {
            // We are full
            field.insert(coord, Tile::Sand);
            return false;
        } else {
            break;
        }
    }

    field.insert(coord, Tile::Sand);
    true
}

fn add_floor(field: &mut Field, bounds: &mut Bounds) {
    bounds.0 -= 200;
    bounds.1 += 200;
    bounds.3 += 2;
    for i in bounds.0..=bounds.1 {
        field.insert((i, bounds.3), Tile::Rock);
    }
}

fn tile_range(from: i32, to: i32) -> RangeInclusive<i32> {
    if from < to {
        from..=to
    } else {
        to..=from
    }
}

fn tile_to_string(tile: &Tile) -> &'static str {
    match tile {
        Tile::Rock => "#",
        Tile::Air => ".",
        Tile::Sand => "o",
        Tile::SandSpawn => "+",
    }
}
