use self::Direction::*;
use self::Turn::*;

pub enum Direction {
    N,
    E,
    S,
    W,
}

pub enum Turn {
    R,
    L,
}

pub fn parse_dir(s: &str) -> (Turn, i32) {
    let (turn, num) = s.split_at(1);
    let turn = match turn {
        "R" => R,
        "L" => L,
        _ => unreachable!(),
    };
    (turn, num.parse().unwrap())
}

pub fn run(input: &str) -> String {
    let mut direction = N;
    let mut coord = (0, 0);

    let input = input.split(",").map(str::trim).map(parse_dir);
    for (turn, num) in input {
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
        match direction {
            N => coord.0 += num,
            E => coord.1 += num,
            S => coord.0 -= num,
            W => coord.1 -= num,
        }
    }

    format!("{}", coord.0.abs() + coord.1.abs())
}

#[test]
fn test() {
    assert_eq!(super::run_exercise("day1a", "R2, L3"), "5");
    assert_eq!(super::run_exercise("day1a", "R2, R2, R2"), "2");
    assert_eq!(super::run_exercise("day1a", "R5, L5, R5, R3"), "12");
    assert_eq!(super::run_exercise("day1a", None), "287");
}
