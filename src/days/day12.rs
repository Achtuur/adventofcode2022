use std::{path::PathBuf, collections::VecDeque, fmt::Display, time::Duration};
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use super::*;

pub struct Day12 {
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day12/test.txt"),
            InputType::Real => PathBuf::from("./data/day12/real.txt"),
        }
    }
}

impl AdventDay for Day12 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let width = input.split("\n").next().unwrap().trim().len();

        let mut map = input.lines().flat_map(|line| line.chars().map(|c| c as u8 - b'A')).collect::<Vec<u8>>();
        let startI = map.iter().position(|c| *c == b'S' - b'A').unwrap();
        let endI = map.iter().position(|c| *c == b'E' - b'A').unwrap();
        
        let path_len = lee(&mut map, startI, endI, width as i32);

        path_len.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let width = input.split("\n").next().unwrap().trim().len();

        let mut map = input.lines().flat_map(|line| line.chars().map(|c| c as u8 - b'A')).collect::<Vec<u8>>();
        let start_indices = map.iter().enumerate().filter_map(|(i, c)| (*c == b'a' - b'A').then_some(i)).collect::<Vec<usize>>();
        let endI = map.iter().position(|c| *c == b'E' - b'A').unwrap();

        let path_len = start_indices.iter().map(|startI| lee(&mut map, *startI, endI, width as i32)).min().unwrap();
        
        path_len.to_string()
    }
}



///Takes in map to perform lee algorithm on, returns length of shortest path
pub fn lee(map: &mut Vec<u8>, startI: usize, endI: usize, width: i32) -> usize {
    let mut step_map = vec![0; map.len()];
    let mut queue = VecDeque::<usize>::new();
    let (width, height) = get_dimensions(map, width as usize);
    let dir = [1, -1, width as i32, -(width as i32)];

    // print_2d_vec(&map, width);
    let old_start = map[startI];
    if map[startI] < b'a' - b'A' {
        map[startI] = b'z' - b'A'; //start with highest letter so it always succeeds
    }

    map[endI] = b'z' - b'A';
    step_map[startI] = 1;
    queue.push_front(startI);

    while !queue.is_empty() {
        let i = queue.pop_back().unwrap();
        for d in dir {
            let n_i = i as i32 + d;
            if checkBounds(i, n_i, width, height) && map[i] >= map[n_i as usize] - 1 { //valid position
                let n_i = n_i as usize;
                if n_i != startI && n_i != endI && step_map[n_i] == 0 { //if not at end and unvisited, add to queue
                    queue.push_front(n_i);
                }
                if step_map[n_i] == 0 || step_map[i] + 1 < step_map[n_i] {
                    step_map[n_i] = step_map[i] + 1;
                }
            }
        }
    }

    map[startI] = old_start; //revert start back to 'S'
    map[endI] = b'E' - b'A';

    // print_2d_vec(&step_map, width);

    step_map[endI] - 1
}

///Returns (width, height) tuple based on width of single line of input
fn get_dimensions<T>(map: &Vec<T>, width: usize) -> (usize, usize) {
    let height = map.len() / width;
    (width, height)
}

fn i_to_xy(i: usize, width: usize) -> (usize, usize) {
    let y = i / width;
    let x = i % width;
    (x, y)
}

fn xy_to_i(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

fn checkBounds(start_i: usize, new_i: i32, width: usize, height: usize) -> bool {
    same_row(start_i, new_i as usize, width) || same_col(start_i, new_i as usize, width) 
    && new_i >= 0 
    && (new_i as usize) < (height*width - 1)
}

fn same_row(i1: usize, i2: usize, width: usize) -> bool {
    let (_, y1) = i_to_xy(i1, width);
    let (_, y2) = i_to_xy(i2, width);
    y1 == y2
}

fn same_col(i1: usize, i2: usize, width: usize) -> bool {
    let (x1, _) = i_to_xy(i1, width);
    let (x2, _) = i_to_xy(i2, width);
    x1 == x2
}

fn print_2d_vec<T>(map: &Vec<T>, width: usize) where T:std::fmt::Debug {
    map.chunks(width).into_iter().for_each(|line| println!("{:?}", line));
}