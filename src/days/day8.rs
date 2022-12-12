use std::path::PathBuf;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use super::*;

pub struct Day8 {
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day8/test.txt"),
            InputType::Real => PathBuf::from("./data/day8/real.txt"),
        }
    }
}

impl AdventDay for Day8 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        #[cfg(target_os = "windows")]
        let input_it = input.trim().split("\r\n");

        #[cfg(target_os = "linux")]
        let input_it = input.trim().split("\n");

        let width = input.split('\n').next().unwrap().trim().len();

        //Get 2d vector of map
        let tree_map: Vec<u8> = input_it.flat_map(|line| line.chars().into_iter().map(|c| c as u8 - b'0')).collect_vec();
        let (width, height) = get_dimensions(&tree_map, width);

        let mut sum = 2*height + 2*width - 4; // -4 since edges are counted twice
        //loop over map without including outer edge
        for y in 1..height-1 {
            for x in 1..width-1 {
                let start_i = xy_to_i(x, y, width);
                if is_visible(&tree_map[..], start_i, width, height) {
                    sum += 1;
                }
            }
        }

        sum.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        #[cfg(target_os = "windows")]
        let input_it = input.trim().split("\r\n");

        #[cfg(target_os = "linux")]
        let input_it = input.trim().split("\n");

        let width = input.split('\n').next().unwrap().trim().len();

        //Get 2d vector of map
        let tree_map: Vec<u8> = input_it.flat_map(|line| line.chars().into_iter().map(|c| c as u8 - b'0')).collect_vec();
        let (width, height) = get_dimensions(&tree_map, width);

        let mut max = 0; // -4 since edges are counted twice
        //loop over map without including outer edge
        for y in 1..height-1 {
            for x in 1..width-1 {
                let start_i = xy_to_i(x, y, width);
                let p = get_scenic_score(&tree_map[..], start_i, width, height);
                if p > max {
                    max = p;
                }
            }
        }

        max.to_string()
    }
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

fn checkBounds(start_i: usize, new_i: usize, width: usize, height: usize) -> bool {
    same_row(start_i, new_i, width) || same_col(start_i, new_i, width) && new_i > 0 && new_i < (height*width - 1)
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

fn is_visible(tree_map: &[u8], start_i: usize, width: usize, height: usize) -> bool {
    let dir = [1, -1, width as i32, -(width as i32)];
    dir.iter().any(|d| {
        let mut c = 0;
        let mut new_i = (start_i as i32 + *d as i32) as usize;
        while checkBounds(start_i, new_i, width, height) {
            if tree_map[new_i] >= tree_map[start_i] {
                return false
            }
            c += 1;
            new_i = (start_i as i32 + c**d as i32) as usize;
        }
        true
    })
}

///What happens inside with `c` is a bit magical but it works
fn get_scenic_score(tree_map: &[u8], start_i: usize, width: usize, height: usize) -> u32 {
    let dir = [1, -1, width as i32, -(width as i32)];
    dir.iter().map(|d| {
        let mut c = 0;
        let mut new_i = (start_i as i32 + (c+1)**d as i32) as usize;
        while checkBounds(start_i, new_i, width, height) {
            if tree_map[new_i] >= tree_map[start_i] { //if blocked
                c+=1;
                break;
            }
            c += 1;
            new_i = (start_i as i32 + (c+1)**d as i32) as usize;
        }
        c as u32
    }).product()
}