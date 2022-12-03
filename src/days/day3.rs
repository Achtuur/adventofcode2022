use std::path::PathBuf;

use super::*;
use itertools::{izip, Itertools};
use substring::Substring;

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day3/test.txt"),
            InputType::Real => PathBuf::from("./data/day3/real.txt"),
        }
    }
}

impl AdventDay for Day3 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");

        let total: u32 = input.trim().split('\n').fold(0, |sum, rucksack| {
            let mid_i = rucksack.len() / 2;
            let compartment1 = rucksack.substring(0, mid_i).trim();
            let compartment2 = rucksack.substring(mid_i, rucksack.len()).trim();
            let mut both = compartment1
                .as_bytes()
                .iter()
                .filter(|item1| compartment2.as_bytes().iter().any(|item2| item2 == *item1));
            sum + get_points(both.next().unwrap()) as u32
        });

        total.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");

        let input_iter = input.trim().split('\n');

        // let total = input_iter.zip(input_iter.skip(1)).zip(input_iter.skip(1))
        let total =
            input_iter
                .tuples::<(&str, &str, &str)>()
                .fold(0, |sum, (line1, line2, line3)| {
                    let (rucksack1, rucksack2, rucksack3) =
                        (line1.trim(), line2.trim(), line3.trim());
                    let mut both12 = rucksack1
                        .trim()
                        .as_bytes()
                        .iter()
                        .filter(|item1| rucksack2.as_bytes().iter().any(|item2| item2 == *item1))
                        .collect::<Vec<&u8>>(); //get matching items in rucksacks 1 and 2
                    both12.sort();
                    both12.dedup(); //remove duplicates
                    let mut both_all = rucksack3
                        .trim()
                        .as_bytes()
                        .iter()
                        .filter(|item3| both12.iter().any(|item12| item12 == item3)); //get matching items between rucksack12 and rucksack 3
                    sum + get_points(both_all.next().unwrap()) as u32
                });

        total.to_string()
    }
}

fn get_points(char: &u8) -> u8 {
    match char {
        65..=90 => char - 65 + 27, //A-Z
        97..=122 => char - 97 + 1, //a-z
        _ => 0,
    }
}

fn to_ascii(char: &u8) -> char {
    match char {
        65..=90 => char.to_ascii_uppercase() as char,
        97..=122 => char.to_ascii_lowercase() as char,
        _ => '_',
    }
}

#[cfg(test)]
mod tests {
    use substring::Substring;

    #[test]
    fn test_substr() {
        let s = "abcdefgh";
        assert_eq!(s.substring(0, 3), "abc");
    }
}
