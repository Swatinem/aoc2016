use std::fmt::Write;

use super::day2a::{Direction, parse_direction};

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
fn advance(num: u8, direction: Direction) -> u8 {
    use self::Direction::*;
    match direction {
        Up => {
            if num == 3 {
                1
            } else if num >= 6 && num <= 12 && num != 9 {
                num - 4
            } else if num == 13 {
                11
            } else {
                num
            }
        }
        Right => {
            if num == 1 || num == 4 || num == 9 || num == 12 || num == 13 {
                num
            } else {
                num + 1
            }
        }
        Down => {
            if num == 11 {
                13
            } else if num >= 2 && num <= 8 && num != 5 {
                num + 4
            } else if num == 1 {
                3
            } else {
                num
            }
        }
        Left => {
            if num == 1 || num == 2 || num == 5 || num == 10 || num == 13 {
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
            write!(code, "{:X}", num).unwrap();
        } else {
            num = advance(num, parse_direction(c));
        }
    }

    code
}

#[test]
fn test() {
    assert_eq!(super::run_exercise("day2b", "ULL\nRRDDD\nLURDL\nUUUUD\n"),
               "5DB3");
    assert_eq!(super::run_exercise("day2b", None), "B3C27");
}
