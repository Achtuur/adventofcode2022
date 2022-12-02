use std::path::PathBuf;

use super::*;

pub struct Day3 {
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day1/test.txt"),
            InputType::Real => PathBuf::from("./data/day1/real.txt"),
        }
    }
}

impl AdventDay for Day3 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        todo!();
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        todo!();
    }
}