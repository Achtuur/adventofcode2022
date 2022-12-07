use std::{path::PathBuf, collections::HashMap, rc::Rc, ops::{Deref, DerefMut}, cell::{RefCell, Ref}, thread::current};
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
            InputType::Real => PathBuf::from("./data/day7/real2.txt"),
        }
    }
}

impl AdventDay for Day7 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }

        let mut dir = get_dir_structure(&input).take();
        let mut totsize: u32 = 0;
        let s = get_dir_size(&mut dir, &mut totsize);
        if s < 100_000 { //this will probably never happen, but it might
            totsize += s;
        }
        // println!("dir: {0:#?}", dir);
        totsize.to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }
}
#[derive(Clone, Debug, Default)]
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

    fn add_dir(&mut self, dir: Directory) {
        self.children.push(dir);
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push((name, size));
    }
}

type File = (String, u32);

#[allow(clippy::collapsible_else_if)]
fn get_dir_structure(input: &String) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new("/")));
    let mut current_dir = Rc::clone(&root);
    let mut parents = vec![Rc::clone(&root)];

    input.trim().split('\n').skip(1).for_each(|line| {
        let mut spl = line.trim().split(' ');
        if line.starts_with('$'){ //command
            if spl.nth(1).unwrap() == "cd" {
                let dir_name = spl.next().unwrap();
                if dir_name == ".." {
                    parents.last().unwrap().borrow_mut().add_dir(current_dir.borrow_mut().clone());
                    current_dir = parents.pop().unwrap();
                } else {
                    // let target_dir = current_dir.borrow_mut().get_child(dir_name.to_owned()).unwrap().clone();
                    let target_dir = Directory::new(dir_name);
                    parents.push(Rc::clone(&current_dir));
                    current_dir = Rc::new(RefCell::new(target_dir));
                }
            }
        } else {
            if !line.starts_with("dir") {
                let size = spl.next().unwrap().parse::<u32>().unwrap();
                let name = spl.next().unwrap();
                current_dir.borrow_mut().add_file(name.to_owned(), size);
            }
        }
    });
    root.borrow_mut().add_dir(current_dir.borrow_mut().clone());
    root
}

fn get_dir_size(dir: &mut Directory, totsize: &mut u32) -> u32 {
    let mut size = dir.children.iter_mut().map(|d| {
        let s = get_dir_size(d, totsize);
        if s < 100_000 {
            println!("s: {0:?}", s);
            *totsize += s;
        }
        s
    }).sum();
    size += dir.files.iter().map(|file| file.1).sum::<u32>();
    dir.size = size;
    size
}