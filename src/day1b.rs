use std::collections::HashSet;
use parsers::*;

named!(sep, eat_separator!(&b", "[..]));
named!(parse_dir <(char, i64)>, delimited!(sep, tuple!(one_of!("RL"), int), sep));

enum Direction {
    N,
    E,
    S,
    W,
}
use self::Direction::*;

type Coord = (i64, i64);

fn record_line(visited: &mut HashSet<Coord>,
               mut coord: Coord,
               direction: &Direction,
               num: i64)
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

pub fn run(input: &[u8]) -> String {
    let mut direction = N;
    let mut visited = HashSet::new();
    let mut coord = (0, 0);
    let mut input = input;

    while let IResult::Done(_input, (turn, num)) = parse_dir(input) {
        input = _input;
        visited.insert(coord.clone());
        direction = match (direction, turn) {
            (N, 'R') => E,
            (N, 'L') => W,
            (E, 'R') => S,
            (E, 'L') => N,
            (S, 'R') => W,
            (S, 'L') => E,
            (W, 'R') => N,
            (W, 'L') => S,
            _ => unreachable!(),
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
    assert_eq!(super::run_exercise("day1b", &b"R8, R4, R4, R8"[..]), "4");
    assert_eq!(super::run_exercise("day1b", None), "133");
}
