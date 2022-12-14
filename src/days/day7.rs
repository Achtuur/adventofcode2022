use super::*;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    i64::MAX,
    ops::{Deref, DerefMut},
    path::PathBuf,
    rc::Rc,
    thread::current,
};

pub struct Day7 {}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {}
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

        let mut dir = get_dir_structure(&input).take();
        let mut totsize: u32 = 0;
        let s = get_dir_size(&mut dir, &mut totsize);
        if s < 100_000 {
            //this will probably never happen, but it might
            totsize += s;
        }
        totsize.to_string()
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

        let mut dir = get_dir_structure(&input).take();
        let unused_space = 70_000_000 - get_dir_size(&mut dir, &mut 0);
        let delete_size = 30_000_000 - unused_space;
        let smallest = get_min_dir_size(&mut dir, delete_size);

        smallest.to_string()
    }
}
#[derive(Clone, Debug, Default)]
struct Directory {
    name: String,
    children: Vec<Directory>,
    files: Vec<File>,
    size: u32,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: String::from(name),
            children: Vec::<Directory>::new(),
            files: Vec::<File>::new(),
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
fn get_dir_structure(input: &str) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new("/")));
    let mut current_dir = Rc::clone(&root);
    let mut parents: Vec<Rc<RefCell<Directory>>> = vec![];

    input.trim().split('\n').skip(1).for_each(|line| {
        let mut spl = line.trim().split(' ');
        if line.starts_with('$') {
            //command
            if spl.nth(1).unwrap() == "cd" {
                let dir_name = spl.next().unwrap();
                if dir_name == ".." {
                    parents
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .add_dir(current_dir.borrow_mut().clone());
                    current_dir = parents.pop().unwrap();
                } else {
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

    while !parents.is_empty() {
        parents
            .last()
            .unwrap()
            .borrow_mut()
            .add_dir(current_dir.borrow_mut().clone());
        current_dir = parents.pop().unwrap();
    }
    root
}

///Gets size for each directory, also adds all sizes under `100.000` and puts them in `totsize`
fn get_dir_size(dir: &mut Directory, totsize: &mut u32) -> u32 {
    let mut size = dir
        .children
        .iter_mut()
        .map(|d| {
            let s = get_dir_size(d, totsize);
            if s < 100_000 {
                *totsize += s;
            }
            s
        })
        .sum();
    size += dir.files.iter().map(|file| file.1).sum::<u32>();
    dir.size = size;
    size
}

///Get minimum dir size for deletion
fn get_min_dir_size(dir: &mut Directory, delete_size: u32) -> u32 {
    match dir
        .children
        .iter_mut()
        .map(|child| {
            if child.size >= delete_size {
                let m = get_min_dir_size(child, delete_size);
                return std::cmp::min(child.size, m);
            }
            u32::MAX
        })
        .min()
    {
        Some(x) => x,
        None => u32::MAX,
    }
}
