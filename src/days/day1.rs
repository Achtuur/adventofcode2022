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
        println!("Self::get_path(it): {0:?}", Self::get_path(it));
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        let reg = Regex::new(r"\r\n").expect("invalid regex"); //split by empty lines
        let input: Vec<&str> = reg.split(&input).into_iter().collect();

        let mut max = 0;
        let mut stack: Vec<u32> = Vec::<u32>::new();

        for i in 0..input.len() {
            let line = input[i];
            if line.is_empty() || i == input.len() - 1 {
                let x = stack.iter().sum(); // get sum of calories
                if x > max {
                    max = x;
                }
                stack.clear();
            } else {
                stack.push(line.parse::<u32>().expect("pushing NaN"));
            }
        }

        max.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        println!("Self::get_path(it): {0:?}", Self::get_path(it));
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        let reg = Regex::new(r"\r\n").expect("invalid regex"); //split by empty lines
        let input: Vec<&str> = reg.split(&input).into_iter().collect();

        let mut snacks: Vec<u32> = Vec::<u32>::new();
        let mut stack: Vec<u32> = Vec::<u32>::new();

        for i in 0..input.len() {
            let line = input[i];
            let last = i == input.len() - 1;
            if line.is_empty() || last {
                let x = if last {
                    line.parse::<u32>().expect("NaN")
                } else {
                    stack.iter().sum() // get sum of calories
                };
                
                if snacks.len() < 3 {
                    snacks.push(x)
                } else {
                    for i in 0..3 {
                        if x > snacks[i] {
                            snacks[i] = x;
                            break;
                        }
                    }
                }
                snacks.sort();
                stack.clear();
            } else {
                stack.push(line.parse::<u32>().expect("pushing NaN"));
            }
        }
        (snacks.into_iter().sum::<u32>()).to_string()
    }
}
