pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub enum InputType {
    Test,
    Real,
}

pub trait AdventDay {
    fn A(&self, it: &InputType) -> String;
    fn B(&self, it: &InputType) -> String;
}