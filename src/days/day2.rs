use std::{path::PathBuf, collections::HashMap};

use super::*;

pub struct Day2 {}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day2/test.txt"),
            InputType::Real => PathBuf::from("./data/day2/real.txt"),
        }
    }
}

impl AdventDay for Day2 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
            
        let shape = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
        let win: HashMap<&str, i32> = HashMap::from([
            ("AX", 3), ("AY", 6), ("AZ", 0),
            ("BX", 0), ("BY", 3), ("BZ", 6),
            ("CX", 6), ("CY", 0), ("CZ", 3),
        ]);

        let points: i32 = input.trim().split('\n').map(|game| {
            let hands: Vec<&str> = game.split(' ').into_iter().map(|x| x.trim()).collect();
            shape[hands[1]] + win[concat(hands[0], hands[1]).as_str()]
        }).sum();
        points.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");

        let shape = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
        let win: HashMap<&str, i32> = HashMap::from([
            ("AX", 0 + shape["Z"]), ("AY", 3 + shape["X"]), ("AZ", 6 + shape["Y"]),
            ("BX", 0 + shape["X"]), ("BY", 3 + shape["Y"]), ("BZ", 6 + shape["Z"]),
            ("CX", 0 + shape["Y"]), ("CY", 3 + shape["Z"]), ("CZ", 6 + shape["X"]),
        ]);

        let points: i32 = input.trim().split('\n').map(|game| {
            let hands: Vec<&str> = game.split(' ').into_iter().map(|x| x.trim()).collect();
            win[concat(hands[0], hands[1]).as_str()]
        }).sum();
        points.to_string()
    }
}

fn concat(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}