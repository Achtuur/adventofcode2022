use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use super::*;

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day4/test.txt"),
            InputType::Real => PathBuf::from("./data/day4/real.txt"),
        }
    }
}

impl AdventDay for Day4 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let pairs = input.trim().split('\n')
        .flat_map(|line| line.trim().split(',')) //[a-b, c-d]
        .flat_map(|elf| elf.split('-')) //[a, b, c, d] (as strings)
        .map(|i| i.parse::<i32>().unwrap())
        .tuples()
        .filter(|(a, b, c, d)| a <= c && b >= d || c <= a && d >= b)
        .count();

        pairs.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");

        let pairs = input.trim().split('\n')
            .flat_map(|line| line.trim().split(',')) //[a-b, c-d]
            .flat_map(|elf| elf.split('-')) //[a, b, c, d] (as strings)
            .map(|i| i.parse::<i32>().unwrap())
            .tuples()
            .filter(|(a, b, c, d)| check_partial_overlap(a, b, c, d) || check_partial_overlap(c, d, a, b))
            .count();
        pairs.to_string()
    }
}


///Returns true if elf1 (a,b) is partially inside elf2 (c,d)
fn check_partial_overlap(a: &i32, b: &i32, c: &i32, d: &i32) -> bool {
    a >= c && a <= d || b >= c && b <= d
}
