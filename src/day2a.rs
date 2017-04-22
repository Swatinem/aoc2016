use std::fmt::Write;

use self::Direction::*;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn parse_direction(i: char) -> Direction {
    println!("{:#?}", i);
    match i {
        'U' => Up,
        'R' => Right,
        'D' => Down,
        'L' => Left,
        _ => unreachable!(),
    }
}

// 1 2 3
// 4 5 6
// 7 8 9
fn advance(num: u8, direction: Direction) -> u8 {
    match direction {
        Up => {
            if num == 1 || num == 2 || num == 3 {
                num
            } else {
                num - 3
            }
        }
        Right => {
            if num == 3 || num == 6 || num == 9 {
                num
            } else {
                num + 1
            }
        }
        Down => {
            if num == 7 || num == 8 || num == 9 {
                num
            } else {
                num + 3
            }
        }
        Left => {
            if num == 1 || num == 4 || num == 7 {
                num
            } else {
                num - 1
            }
        }
    }
}

pub fn run(input: &str) -> String {
    let mut num = 5;
    let mut code = String::with_capacity(8);

    for c in input.chars() {
        if c == '\n' {
            write!(code, "{}", num).unwrap();
        } else {
            num = advance(num, parse_direction(c));
        }
    }

    code
}

#[test]
fn test() {
    assert_eq!(super::run_exercise("day2a", "ULL\nRRDDD\nLURDL\nUUUUD\n"),
               "1985");
    assert_eq!(super::run_exercise("day2a", None), "56855");
}
