mod day1a;
mod day1b;
mod day2a;
mod day2b;

use std::io::prelude::*;
use std::fs::File;

macro_rules! run {
    ($ex:ident($input:ident) for $($f: ident,)*) => {
        match ($ex, $input) {
            $((stringify!($f), input) => $f::run(input),)*
            (name, _) => panic!("Exercise {} not implemented", name),
        }
    }
}

pub fn run_exercise<'a, I>(exercise: &str, input: I) -> String
    where I: Into<Option<&'a str>>
{
    let mut buffer = String::with_capacity(4096);
    let input = input
        .into()
        .unwrap_or_else(|| {
                            let path = format!("./input/{}.txt", exercise);
                            let mut file = File::open(&path).unwrap();
                            file.read_to_string(&mut buffer).unwrap();
                            &buffer
                        });

    run!(exercise(input) for
         day1a,
         day1b,
         day2a,
         day2b,
    )
}

fn main() {
    // TODO: ability to run exercise via command line, with either a file or stdin input
}
