use super::*;
use colored::{ColoredString, Colorize};
use std::{
    collections::{HashMap, VecDeque, HashSet},
    path::PathBuf,
};

pub struct Day6 {}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day6/test.txt"),
            InputType::Real => PathBuf::from("./data/day6/real.txt"),
        }
    }
}

impl AdventDay for Day6 {
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

        let mut v: VecDeque<char> = VecDeque::new();
        let code_idx: usize = input
            .trim()
            .chars()
            .enumerate()
            .find_map(|(i, c)| {
                if v.len() < 4 {
                    v.push_front(c);
                    return None;
                }

                v.push_front(c); //add new char
                v.pop_back(); //remove oldest char

                let mut h = HashSet::<&char>::with_capacity(4);
                v.iter().for_each(|v_c| {
                    h.insert(v_c);
                });

                if h.len() == 4 {
                    return Some(i + 1);
                }

                None
            })
            .unwrap();

        code_idx.to_string()
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

        let mut v: VecDeque<char> = VecDeque::new();
        let code_idx: usize = input
            .trim()
            .chars()
            .enumerate()
            .find_map(|(i, c)| {
                if v.len() < 14 {
                    v.push_front(c);
                    return None;
                }

                v.push_front(c); //add new char
                v.pop_back(); //remove oldest char

                let mut h = HashSet::<&char>::with_capacity(14);
                v.iter().for_each(|v_c| {
                    h.insert(v_c);
                });

                if h.len() == 14 {
                    return Some(i + 1); //+1 since aoc counts first element as 1
                }

                None
            })
            .unwrap();

        code_idx.to_string()
    }
}
