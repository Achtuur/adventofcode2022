use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use super::*;

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day4/test.txt"),
            InputType::Real => PathBuf::from("./data/day4/real.txt"),
        }
    }
}

impl AdventDay for Day4 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let tot_pairs = input.trim().split('\n').fold(0_u32, |n_pairs, line| {
            //line = a-b,c-d
            let elfs = line
                .trim()
                .split(',')
                .map(|elf| { //elf = "x-y"
                    elf.split('-')
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(); //groups = [[a, b], [c, d]]

            //Check if elf0 is fully overlapped by elf1 or other way around
            if check_full_overlap(&elfs[0], &elfs[1]) || check_full_overlap(&elfs[1], &elfs[0]) {
                return n_pairs + 1;
            }
            n_pairs
        });
        tot_pairs.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        let tot_pairs = input.trim().split('\n').fold(0_u32, |n_pairs, line| {
            //line = a-b,c-d
            let elfs = line
                .trim()
                .split(',')
                .map(|elf| { //elf = "x-y"
                    elf.split('-')
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(); //groups = [[a, b], [c, d]]

            // Check if elf0 partially overlaps into elf1 or other way around
            if check_partial_overlap(&elfs[0], &elfs[1]) || check_partial_overlap(&elfs[1], &elfs[0])
            {
                return n_pairs + 1;
            }
            n_pairs
        });
        tot_pairs.to_string()
    }
}

///Returns true if elf1 is fully inside elf 2
fn check_full_overlap(elf1: &[i32], elf2: &[i32]) -> bool {
    elf1[0] <= elf2[0] && elf1[1] >= elf2[1] // || (elf2[0] <= elf1[0] && elf2[1] >= elf1[1])
}
///Returns true if elf1 is partially inside elf 2
fn check_partial_overlap(elf1: &[i32], elf2: &[i32]) -> bool {
    elf1[0] >= elf2[0] && elf1[0] <= elf2[1] || elf1[1] >= elf2[0] && elf1[1] <= elf2[1]
}
