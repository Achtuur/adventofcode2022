use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use super::*;

pub struct Day10 {
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day10/test.txt"),
            InputType::Real => PathBuf::from("./data/day10/real.txt"),
        }
    }
}

impl AdventDay for Day10 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let mut elfpc = ElfPc::new();
        let sum: i32 = input.lines().map(|line| {
            elfpc.cycle(line, None);
            elfpc.check_signal_strength();
            elfpc.signal_strength
        }).sum();

        sum.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        let mut elfpc = ElfPc::new();
        let mut crt = Crt::new();

        input.lines().for_each(|line| {
            elfpc.cycle(line, Some(&mut crt));
            crt.cycle(elfpc.x);
        });

        format!("\n{}", crt.screen_to_string())
    }
}


struct ElfPc {
    pc: usize,
    x: i32,
    signal_strength: i32,
    signal_checked: bool,
}

impl ElfPc {
    pub fn new() -> ElfPc {
        ElfPc { pc: 1, x: 1, signal_strength: 0, signal_checked: false}
    }

    pub fn cycle(&mut self, instr: &str, crt: Option<&mut Crt>) {
        self.signal_checked = false;
        let mut spl = instr.split(' ');
        let op = spl.next().unwrap();
        match op {
            "noop" => self.pc += 1,
            "addx" => {
                self.pc += 1;
                self.check_signal_strength();
                if let Some(c) = crt {
                    c.cycle(self.x)
                }
                self.x += spl.next().unwrap().parse::<i32>().unwrap();
                self.pc += 1;
            },
            _ => (),
        };
    }

    pub fn check_signal_strength(&mut self) {
        if self.pc >= 20 && (self.pc - 20) % 40 == 0 {
            self.signal_strength = self.x * self.pc as i32;
            self.signal_checked = true;
        } else if !self.signal_checked {
            self.signal_strength = 0;
        }
    }
}

struct Crt {
    pc: usize,
    screen: Vec<Vec<char>>,
}

impl Crt {
    pub fn new() -> Self {
        Crt { pc: 1, screen: vec![vec!['.'; 40]; 6] }
    }

    pub fn cycle(&mut self, reg_x: i32) {
        let x_pos = self.pc as i32 % 40;
        let y = self.pc / 40;
        if x_pos == reg_x || x_pos == reg_x - 1 || x_pos == reg_x + 1 {
            self.screen[y][x_pos as usize] = '#'
        }
        self.pc += 1;
    }
    
    pub fn screen_to_string(&self) -> String {
        self.screen.iter().map(|line| line.iter().collect::<String>() + "\n").collect()
    }
}