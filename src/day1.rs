use std::path::PathBuf;
use regex::Regex;
use crate::{AdventDay, InputType};

pub struct Day1 {
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day1/test.txt"),
            InputType::A => PathBuf::from("./data/day1/A.txt"),
            InputType::B => PathBuf::from("./data/day1/B.txt"),
            _ => PathBuf::from("../data/day1/test.txt"),
        }
    }
}

impl AdventDay for Day1 {
    fn A(&self, it: InputType) -> String {
        println!("Self::get_path(it): {0:?}", Self::get_path(&it));
        let input = std::fs::read_to_string(Self::get_path(&it)).expect("Reading input failed, file doesn't exist most likely");
        let reg = Regex::new(r"\r\n").expect("invalid regex"); //split by empty lines
        let input: Vec<&str> = reg.split(&input).into_iter().collect();

        println!("input: {0:?}", input);

        let max = 0;
        let stack: Vec<u32> = Vec::<u32>::new();

        for line in input.iter() {
            if line.len() == 0 {
                
            }
        }

        todo!();
    }

    fn B(&self, it: InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(&it)).expect("Reading input failed, file doesn't exist most likely");
        todo!();
    }
}