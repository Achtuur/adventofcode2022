//usage: node gen_day.js <day>
const path = require('path');
const fs = require('fs');


let args = process.argv.slice(2); //get all additional arguments
let day = args[0];
let force = args[1];

//Add new files
let source_path = path.resolve(__dirname, `src/days/day${day}.rs`); //path to src/dayx.rs
let input_path = path.resolve(__dirname, `data/day${day}/`) //path to src/input/dayx/
if (!fs.existsSync(input_path)) {
    fs.mkdirSync(input_path);
}
let fn = ['test.txt', 'real.txt'];
let input_paths = fn.map(f => path.resolve(input_path, f));


let template = `use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use super::*;

pub struct Day${day} {
}

impl Day${day} {
    pub fn new() -> Day${day} {
        Day${day}{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day${day}/test.txt"),
            InputType::Real => PathBuf::from("./data/day${day}/real.txt"),
        }
    }
}

impl AdventDay for Day${day} {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }
}`;

if(!fs.existsSync(source_path) || force == 1) { 
    fs.writeFileSync(source_path, template, (err) => console.error(err));
} else {
    console.error(`Day ${day}.rs already exists, run with force = 1 to overwrite`);
}
input_paths.forEach(file_path => {
    if (!fs.existsSync(file_path) || force == 1) {
        fs.writeFileSync(file_path, "", (err) => console.error(err))
    } else {
        console.error(`${file_path} already exists, run with force = 1 to overwrite`);
    }
});

// Edit main.rs and days.rs
let main_p = path.resolve(__dirname, "src/main.rs");
let days_p = path.resolve(__dirname, "src/days.rs");

let main_contents = fs.readFileSync(main_p, {encoding: "utf-8"});

let prev_day_reg = new RegExp(`(use day${day-1}::Day${day-1};)`);
let current_day_reg = new RegExp(`use day${day}::Day${day};`);

if (!current_day_reg.test(main_contents)) { //if 'use day${day}...' does not exist yet1
    main_contents = main_contents.replace(prev_day_reg, `$1\nuse day${day}::Day${day};`); 
    fs.writeFileSync(main_p, main_contents, (err) => console.error(err));
}

prev_day_reg = new RegExp(`(pub mod day${day-1};)`);
current_day_reg = new RegExp(`pub mod day${day};`);
let days_contents = fs.readFileSync(days_p, {encoding: "utf-8"});
if (!current_day_reg.test(days_contents)) {
    days_contents = days_contents.replace(prev_day_reg, `$1\npub mod day${day};`);
    fs.writeFileSync(days_p, days_contents, (err) => console.error(err));
}

