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

        let points: i32 = input.trim().split('\n').map(|game| {
            let hands: Vec<&str> = game.split(' ').into_iter().map(|x| x.trim()).collect();
            shape_points(hands[1]) + match_points_A(hands[0], hands[1])
        }).sum();
        points.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");

        let points: i32 = input.trim().split('\n').map(|game| {
            let hands: Vec<&str> = game.split(' ').into_iter().map(|x| x.trim()).collect();
            match_points_B(hands[0], hands[1])
        }).sum();
        points.to_string()
    }
}

fn shape_points(played: &str) -> i32 {
    match played {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn match_points_A(opponent: &str, played: &str) -> i32 {
    match (opponent, played) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,

        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,

        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => 0,
    }
}

fn match_points_B(opponent: &str, played: &str) -> i32 {
    match (opponent, played) {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,
        _ => 0,
    }
}

