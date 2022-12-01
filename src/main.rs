mod day1;


use day1::Day1;
use std::path::PathBuf;
use regex::Regex;

///Type of input file to read
pub enum InputType {
    Test,
    A,
    B,
}

pub trait AdventDay {
    fn A(&self, it: InputType) -> String;
    fn B(&self, it: InputType) -> String;
}

fn main() {
    let it = InputType::Test;

    let day = Day1::new();
    let ansA = day.A(it);
    println!("ansA: {0:?}", ansA);

    // let ansB = day.B(it);
    // println!("ansB: {0:?}", ansB);
}
