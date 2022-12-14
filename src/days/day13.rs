use super::*;
use colored::{ColoredString, Colorize};
use eval::eval;
use itertools::{FoldWhile, Itertools};
use std::path::PathBuf;
use substring::Substring;

pub struct Day13 {}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day13/test.txt"),
            InputType::Real => PathBuf::from("./data/day13/real.txt"),
        }
    }
}

impl AdventDay for Day13 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 {
            //arbitrary small value
            println!(
                "{}",
                "Input file empty, you probably forgot to copy the input data"
                    .bold()
                    .red()
            );
        }

        #[cfg(target_os = "windows")]
        let input_it = input.split("\r\n\r\n");

        #[cfg(target_os = "linux")]
        let input_it = input.split("\n\n");

        todo!();
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 {
            //arbitrary small value
            println!(
                "{}",
                "Input file empty, you probably forgot to copy the input data"
                    .bold()
                    .red()
            );
        }
        todo!();
    }
}

fn compare(line1: &str, line2: &str) -> bool {
    todo!()
}

fn indexof_closing_bracket(str: &str, start_index: usize) -> usize {
    let str = str.substring(start_index, str.len() - 1);
    let open_bracket = str.chars().next().unwrap();
    let close_bracket = get_close_bracket(&open_bracket);
    let mut v = Vec::<char>::new();
    str.chars()
        .into_iter()
        .fold_while(0, |i, c| {
            if c == open_bracket {
                v.push(c);
            } else if c == close_bracket {
                v.pop();
            }

            if v.is_empty() {
                return FoldWhile::Done(i);
            }
            FoldWhile::Continue(i + 1)
        })
        .into_inner() as usize
}

fn get_close_bracket(c: &char) -> char {
    match c {
        '[' => ']',
        '{' => '}',
        '(' => ')',
        '<' => '>',
        _ => '\0',
    }
}
