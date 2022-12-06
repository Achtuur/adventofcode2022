pub mod day1;

pub enum InputType {
    Test,
    Real,
}

pub trait AdventDay {
    fn A(&self, it: &InputType) -> String;
    fn B(&self, it: &InputType) -> String;
}
