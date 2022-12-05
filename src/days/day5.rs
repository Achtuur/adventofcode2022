use std::{path::PathBuf, collections::VecDeque};
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use regex::Regex;
use super::*;

pub struct Day5 {
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day5/test.txt"),
            InputType::Real => PathBuf::from("./data/day5/real.txt"),
        }
    }
}

impl AdventDay for Day5 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        #[cfg(target_os = "windows")]
        let mut input_it = input.split("\r\n\r\n"); //first entry contains initial stack, second entry contains the move commands

        #[cfg(target_os = "linux")]
        let mut input_it = input.split("\n\n"); //first entry contains initial stack, second entry contains the move commands
        
        let mut queue = read_init_state(input_it.next().unwrap());
        let num_reg = Regex::new(r"\d+").expect("invalid regex");
        input_it.next().unwrap().split('\n').for_each(|op| {
            //args: move args[0] from args[1] to args[2]
            let args = num_reg.find_iter(op).map(|i| i.as_str().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            for _ in 0..args[0] {
                let a = queue[args[1] - 1].pop_back().unwrap();
                queue[args[2] - 1].push_back(a);
            }
        });
        queue.iter().fold(String::from(""), |msg, row| format!("{}{}", msg, row[row.len() - 1]))
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        #[cfg(target_os = "windows")]
        let mut input_it = input.split("\r\n\r\n"); //first entry contains initial stack, second entry contains the move commands

        #[cfg(target_os = "linux")]
        let mut input_it = input.split("\n\n"); //first entry contains initial stack, second entry contains the move commands
        
        let mut queue = read_init_state(input_it.next().unwrap());
        let num_reg = Regex::new(r"\d+").expect("invalid regex");
        input_it.next().unwrap().split('\n').for_each(|op| {
            //args: move args[0] from args[1] to args[2]
            let args = num_reg.find_iter(op).map(|i| i.as_str().parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let mut a = Vec::<char>::new();
            for _ in 0..args[0] {
                a.push(queue[args[1] - 1].pop_back().unwrap());
            }
            for i in (0..a.len()).rev() {
                queue[args[2] - 1].push_back(a[i]);
            }
        });
        queue.iter().fold(String::from(""), |msg, row| format!("{}{}", msg, row[row.len() - 1]))
    }
}

fn read_init_state(input: &str) -> Vec<VecDeque<char>> {
    let letter_reg = Regex::new(r"[A-Z]").unwrap();
    input.split('\n').fold(Vec::<VecDeque<char>>::new(), |mut queue, line| {
        let mut idx = 0;

        format!("{} ", line).chars().collect::<Vec<char>>().iter().tuples().for_each(|(open, cargo, close, space)| {
            if queue.len() <= idx  {
                queue.push(VecDeque::<char>::new())
            }
            if letter_reg.is_match(&cargo.to_string()) {
                queue[idx].push_front(*cargo);
            }
            idx += 1;
        });
        queue
    })
}