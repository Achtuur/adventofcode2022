use super::*;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use std::{collections::HashSet, path::PathBuf};
use substring::Substring;

pub struct Day9 {}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day9/test.txt"),
            InputType::Real => PathBuf::from("./data/day9/real.txt"),
        }
    }
}

impl AdventDay for Day9 {
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

        let mut head = Rope::new();
        let mut tail = Rope::new();

        input.lines().for_each(|cmd| {
            let (dir, n) = cmd.split(' ').collect_tuple().unwrap();
            let n = n.parse::<i32>().unwrap();
            (0..n).for_each(|_| {
                head.do_command(dir);
                tail.follow_other(head.pos);
                tail.add_unique_pos();
            })
        });

        tail.visited.len().to_string()
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
        let mut head = vec![Rope::new(); 9];
        let mut tail = Rope::new();

        input.lines().for_each(|cmd| {
            let (dir, n) = cmd.split(' ').collect_tuple().unwrap();
            let n = n.parse::<i32>().unwrap();
            (0..n).for_each(|_| {
                head[0].do_command(dir);
                for i in 0..8 {
                    let p = head[i].pos;
                    head[i + 1].follow_other(p);
                }
                tail.follow_other(head[8].pos);
                tail.add_unique_pos();
            })
        });

        tail.visited.len().to_string()
    }
}

#[derive(Clone, Debug)]
struct Rope {
    pos: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            pos: (0, 0),
            visited: HashSet::<(i32, i32)>::new(),
        }
    }

    pub fn do_command(&mut self, dir: &str) {
        match dir {
            "U" => self.pos.1 += 1,
            "R" => self.pos.0 += 1,
            "D" => self.pos.1 -= 1,
            "L" => self.pos.0 -= 1,
            _ => unreachable!(),
        }
    }

    pub fn follow_other(&mut self, other_pos: (i32, i32)) {
        let dx = self.pos.0 - other_pos.0;
        let dy = self.pos.1 - other_pos.1;
        let dy_abs = dy.abs();
        let dx_abs = dx.abs();
        if is_same_column(dx, dy) && dy_abs == 2 {
            self.pos.1 -= dy / dy_abs; //move 1 in the right direction
        } else if is_same_row(dx, dy) && dx_abs == 2 {
            self.pos.0 -= dx / dx_abs;
        } else if dx_abs > 1 || dy_abs > 1 {
            self.pos.0 -= dx / dx_abs;
            self.pos.1 -= dy / dy_abs;
        }
    }
    pub fn add_unique_pos(&mut self) {
        self.visited.insert(self.pos);
    }
}

fn is_same_column(dx: i32, dy: i32) -> bool {
    dx == 0 && dy != 0
}

fn is_same_row(dx: i32, dy: i32) -> bool {
    dy == 0 && dx != 0
}
