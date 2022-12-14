use super::*;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::path::PathBuf;

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day11/test.txt"),
            InputType::Real => PathBuf::from("./data/day11/real.txt"),
        }
    }
}

impl AdventDay for Day11 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 {
            //arbitrary small value
            println!(
                "{}",
                "Input file empty, you probably forgot to copy the input data"
                    .bold()
                    .red()
            );
        }

        let mut monkeys = input
            .lines()
            .map(|monkey_inp| Monkey::from_input(monkey_inp))
            .collect_vec();

        for _ in 0..20 {
            //20 rounds
            for i in 0..monkeys.len() {
                for _ in 0..monkeys[i].items.len() {
                    let mut worry = monkeys[i].items.pop().unwrap();
                    monkeys[i].times_inspected += 1;
                    worry = monkeys[i].do_op(worry) / 3;
                    if worry % monkeys[i].test == 0 {
                        let t = monkeys[i].true_target;
                        monkeys[t].items.push(worry);
                    } else {
                        let t = monkeys[i].false_target;
                        monkeys[t].items.push(worry);
                    }
                }
            }
        }

        monkeys.sort_by(|a, b| b.times_inspected.partial_cmp(&a.times_inspected).unwrap());

        (monkeys[0].times_inspected * monkeys[1].times_inspected).to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it))
            .expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 {
            //arbitrary small value
            println!(
                "{}",
                "Input file empty, you probably forgot to copy the input data"
                    .bold()
                    .red()
            );
        }

        let mut monkeys = input
            .lines()
            .map(|monkey_inp| Monkey::from_input(monkey_inp))
            .collect_vec();

        let lcm: u64 = monkeys.iter().fold(1, |acc, el| lcm(el.test, acc)); //thank you Victor <3

        for _ in 0..10_000 {
            //10000 rounds
            for i in 0..monkeys.len() {
                for _ in 0..monkeys[i].items.len() {
                    let mut worry = monkeys[i].items.pop().unwrap();
                    monkeys[i].times_inspected += 1;
                    worry = monkeys[i].do_op(worry);
                    worry %= lcm;
                    if worry % monkeys[i].test == 0 {
                        let t = monkeys[i].true_target;
                        monkeys[t].items.push(worry);
                    } else {
                        let t = monkeys[i].false_target;
                        monkeys[t].items.push(worry);
                    }
                }
            }
        }

        monkeys.sort_by(|a, b| b.times_inspected.partial_cmp(&a.times_inspected).unwrap());

        (monkeys[0].times_inspected * monkeys[1].times_inspected).to_string()
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64, u64) -> u64>,
    operation_arg: u64,
    test: u64,
    true_target: usize,
    false_target: usize,
    times_inspected: usize,
}

impl Monkey {
    pub fn new() -> Self {
        Monkey {
            items: Vec::<u64>::new(),
            operation: Box::new(|a, _b| a),
            operation_arg: 0,
            test: 0,
            true_target: 0,
            false_target: 0,
            times_inspected: 0,
        }
    }

    pub fn from_input(input: &str) -> Self {
        let mut spl = input.lines();
        let (_, items, op, test, true_throw, false_throw) = spl.next_tuple().unwrap();
        let mut m = Monkey::new();
        /* Items */
        let reg = Regex::new(r"\d+").unwrap();
        m.items = reg
            .find_iter(items)
            .map(|item| item.as_str().parse::<u64>().unwrap())
            .collect_vec();

        /* operation */
        let reg = Regex::new(r"old ([\+\*]) (\d+|old)").unwrap();
        let cap = reg.captures(op).unwrap();
        match (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()) {
            ("+", "old") => m.operation = Box::new(add_self),
            ("*", "old") => m.operation = Box::new(mul_self),
            ("+", x) => {
                m.operation = Box::new(add);
                m.operation_arg = x.parse().unwrap();
            }
            ("*", x) => {
                m.operation = Box::new(mul);
                m.operation_arg = x.parse().unwrap();
            }
            _ => unreachable!(),
        }

        m.test = test.trim().split(' ').nth(3).unwrap().parse().unwrap();

        m.true_target = true_throw
            .trim()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        m.false_target = false_throw
            .trim()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();

        m
    }

    pub fn do_op(&self, arg: u64) -> u64 {
        (self.operation)(arg, self.operation_arg)
    }
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

fn add_self(a: u64, _b: u64) -> u64 {
    a + a
}

fn mul_self(a: u64, _b: u64) -> u64 {
    a * a
}
