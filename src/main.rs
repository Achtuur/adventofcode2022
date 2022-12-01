#![allow(non_snake_case)]
mod days;
use std::time::Instant;

///Type of input file to read
use days::*;

use day1::Day1;
use day2::Day2;
use day3::Day3;

fn main() {
    // let it = InputType::Test;
    let it = InputType::Real;

    let day = Day1::new();

    let t = Instant::now();
    let ansA = day.A(&it);
    println!("ansA: {0:?}\n time: {1:?}", ansA, t.elapsed());

    let t = Instant::now();
    let ansB = day.B(&it);
    println!("ansB: {0:?}\n time: {1:?}", ansB, t.elapsed());
}
