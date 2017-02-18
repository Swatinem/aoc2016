use std::str;
use std::str::FromStr;
use nom::digit;

pub use nom::IResult;

named!(pub int <i64>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
