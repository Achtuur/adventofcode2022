use super::*;
use regex::Regex;
use std::path::PathBuf;
pub struct Day1 {}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {}
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
        let inputraw = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");

        let calories = get_calories(&inputraw);

        calories.max().unwrap().to_string()
    }

    fn B(&self, it: &InputType) -> String {
        println!("Self::get_path(it): {0:?}", Self::get_path(it));
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        let calories = get_calories(&input);
        let mut cvec: Vec<u32> = calories.collect::<Vec<u32>>();
        cvec.sort_by(|a, b| b.partial_cmp(a).unwrap());
        (cvec[0] + cvec[1] + cvec[2]).to_string()
    }
}

fn get_calories(input: &String) -> impl Iterator<Item = u32> + '_ {
    input.trim().split("\r\n\r\n").map(|elf| {
        elf.split('\n')
            .map(|cookie| {
                return cookie.trim().parse::<u32>().unwrap();
            })
            .sum::<u32>()
    })
}
