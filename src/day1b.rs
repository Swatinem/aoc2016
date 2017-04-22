use std::collections::HashSet;

use super::day1a::{Direction, Turn, parse_dir};
use self::Direction::*;
use self::Turn::*;

type Coord = (i32, i32);

fn record_line(visited: &mut HashSet<Coord>,
               mut coord: Coord,
               direction: &Direction,
               num: i32)
               -> Result<Coord, Coord> {
    for _ in 0..num {
        match *direction {
            N => coord.0 += 1,
            E => coord.1 += 1,
            S => coord.0 -= 1,
            W => coord.1 -= 1,
        }
        if visited.contains(&coord) {
            return Err(coord);
        }
        visited.insert(coord.clone());
    }
    Ok(coord)
}

pub fn run(input: &str) -> String {
    let mut direction = N;
    let mut visited = HashSet::new();
    let mut coord = (0, 0);
    let mut input = input;

    let input = input.split(",").map(str::trim).map(parse_dir);
    for (turn, num) in input {
        visited.insert(coord.clone());
        direction = match (direction, turn) {
            (N, R) => E,
            (N, L) => W,
            (E, R) => S,
            (E, L) => N,
            (S, R) => W,
            (S, L) => E,
            (W, R) => N,
            (W, L) => S,
        };
        coord = match record_line(&mut visited, coord, &direction, num) {
            Ok(coord) => coord,
            Err(coord) => return format!("{}", coord.0.abs() + coord.1.abs()),
        }
    }

    format!("{}", coord.0.abs() + coord.1.abs())
}

#[test]
fn test() {
    assert_eq!(super::run_exercise("day1b", "R8, R4, R4, R8"), "4");
    assert_eq!(super::run_exercise("day1b", None), "133");
}
