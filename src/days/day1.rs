use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use super::*;

pub struct Day1 {
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day1/test.txt"),
            InputType::Real => PathBuf::from("./data/day1/real.txt"),
        }
    }
}

impl AdventDay for Day1 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }
}