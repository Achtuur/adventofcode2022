#![allow(non_snake_case, unused_imports, dead_code)]
mod days;
use colored::{ColoredString, Colorize};
use std::time::{Duration, Instant};

///Type of input file to read
use days::*;

use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;

fn main() {
    let it = InputType::Test;
    // let it = InputType::Real;

    let day = Day16::new();

    let t = Instant::now();
    let ansA = day.A(&it);
    printansA(&ansA, t.elapsed());

    let t = Instant::now();
    let ansB = day.B(&it);
    printansB(&ansB, t.elapsed());
}

fn printansA(ansA: &String, elapsed: Duration) {
    println!(
        "ansA: {0}\t time: {1}",
        ansA.bold().green(),
        format!("{:?}", elapsed).italic().yellow()
    )
}

fn printansB(ansB: &String, elapsed: Duration) {
    println!(
        "ansB: {0}\t time: {1}",
        ansB.bold().purple(),
        format!("{:?}", elapsed).italic().yellow()
    )
}
