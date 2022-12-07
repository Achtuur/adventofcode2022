use std::{path::PathBuf, collections::HashMap, rc::Rc, ops::{Deref, DerefMut}, cell::RefCell, thread::current};
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use super::*;

pub struct Day7 {
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day7/test.txt"),
            InputType::Real => PathBuf::from("./data/day7/real.txt"),
        }
    }
}

impl AdventDay for Day7 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let dir = get_dir_structure(&input);
        println!("dir: {0:?}", dir);


        todo!();
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }
}
#[derive(Clone, Debug)]
struct Directory {
    name: String,
    children: Box<Vec<Directory>>,
    files: Box<Vec<File>>,
    size: u32,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: String::from(name),
            children: Box::new(Vec::<Directory>::new()),
            files: Box::new(Vec::<File>::new()),
            size: 0,
        }
    }

    fn add_dir_with_name(&mut self, name: &str) {
        let d = Directory::new(name);
        self.children.push(d);
    }

    fn add_dir(&mut self, dir: Directory) {
        self.children.push(dir);
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push((name, size));
    }

    //Go to child, returns current directory as parent dir
    fn go_to_child(&mut self, child_name: String) -> Option<Directory> {
        let c = self.children.iter().find(|child| child.name == child_name);
        if let Some(child) = c {
            return Some(child.copy())
        }
        None
    }

    fn copy(&self) -> Directory {
        let mut cp = Directory::new(&self.name);
        cp.children = Box::new(self.children.to_vec());
        cp.files = Box::new(self.files.to_vec());
        cp.size = self.size.clone();
        cp
    }
}

type File = (String, u32);

fn get_dir_structure(input: &String) -> Directory {
    let mut root = Directory::new("/");
    let mut current_dir = Box::new(&mut root);
    let mut parent: Option<Box<&mut Directory>> = None;


    input.trim().split('\n').for_each(|line| {
        let mut spl = line.trim().split(' ');
        if line.starts_with('$') { //command
            if spl.nth(1).unwrap() == "cd" {
                let dir_name = spl.next().unwrap();
                if dir_name == "/" {
                    current_dir = Box::new(&mut root);
                } else if parent.is_some() && dir_name == ".." {
                    current_dir = parent.unwrap();
                } else if current_dir.children.iter().any(|child| child.name == dir_name) { //dir already exists as child
                    let new_dir = current_dir.children.iter().find(|child| child.name == dir_name).unwrap();
                    current_dir = Box::new(&mut new_dir);
                } else { //dir doesnt exist
                    println!("line: {0:?}", line);
                    println!("dir_name: {0:?}", dir_name);
                    println!("current_dir: {0:?}", current_dir);
                    panic!("cd to dir that doesnt exist");
                }
            }
        } else {
            let s = spl.next().unwrap();
            if s.starts_with("dir") {
                let new_dir_name = spl.next().unwrap();
                let mut new_dir = Directory::new(new_dir_name);
                let mut old_dir = current_dir.clone();
                parent = Some(Box::new(&mut old_dir));
                current_dir.children.push(new_dir);
            } else {
                let size = s.parse::<u32>().unwrap();
                let name = String::from(spl.next().unwrap());
                current_dir.files.push((name, size))
            }
        }
    });
    root
}

fn get_dir_size(dir: Directory) -> u32 {
    todo!();
    // dir.iter().fold(0_u32, |size, current_path| {
    //     match current_path {
    //         PathType::Dir(this_dir) => size + get_dir_size(this_dir),
    //         PathType::File(file_size) => size + file_size,
    //     }
    // })
}