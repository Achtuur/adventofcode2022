use std::path::PathBuf;

use crate::{AdventDay, InputType};

pub struct Day2 {
}

impl Day2 {
    pub fn new() -> Day2 {
        Day1{}
    }

    fn get_path(it: InputType) -> PathBuf {
        match it {
            Test => PathBuf::from("./data/day2/test.txt"),
            A => PathBuf::from("./data/day2/A.txt"),
            B => PathBuf::from("./data/day2/B.txt"),
        }
    }
}

impl AdventDay for Day2 {
    fn A(&self, it: InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        todo!();
    }

    fn B(&self, it: InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        todo!();
    }
}