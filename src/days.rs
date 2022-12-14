pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[allow(dead_code)]
pub enum InputType {
    Test,
    Real,
}

pub trait AdventDay {
    fn A(&self, it: &InputType) -> String;
    fn B(&self, it: &InputType) -> String;
}

pub enum Part {
    A,
    B,
}
