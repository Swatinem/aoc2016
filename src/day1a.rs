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

pub fn run(input: &[u8]) -> String {
    let mut direction = N;
    let mut coord = (0, 0);
    let mut input = input;

    while let IResult::Done(_input, (turn, num)) = parse_dir(input) {
        input = _input;
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
    assert_eq!(super::run_exercise("day1a", &b"R2, L3"[..]), "5");
    assert_eq!(super::run_exercise("day1a", &b"R2, R2, R2"[..]), "2");
    assert_eq!(super::run_exercise("day1a", &b"R5, L5, R5, R3"[..]), "12");
    assert_eq!(super::run_exercise("day1a", None), "287");
}
