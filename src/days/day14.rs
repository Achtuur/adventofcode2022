use super::*;
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use std::{path::PathBuf, collections::HashSet, hash::Hash};

pub struct Day14 {}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 {}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day14/test.txt"),
            InputType::Real => PathBuf::from("./data/day14/real.txt"),
        }
    }
}

impl AdventDay for Day14 {
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
        let mut cave = Cave::new(&input);
        while !cave.is_done {
            cave.step(Part::A);
        }
        (cave.sands.len() - 1).to_string()
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
        let mut cave = Cave::new(&input);
        while !cave.is_done {
            cave.step(Part::B);
        }
        (cave.sands.len()).to_string()
    }
}

#[derive(Debug)]
struct Cave {
    rocks: Vec<Rocks>,
    rocks_set: HashSet<(i32, i32)>,
    floor: i32,
    sands: Vec<Sand>,
    sands_finalpos: HashSet<(i32, i32)>,
    is_done: bool,
}

impl Cave {
    pub fn new(full_input: &str) -> Self {
        let rocks = full_input.lines().map(Rocks::new).collect_vec();
        //Combine every rock's position hashset into a single one
        let rocks_set = rocks.iter().fold(HashSet::new(), |mut h, r| { h.extend(&r.hash_pos); h});
        let floor = rocks.iter().flat_map(|r| r.ranges.iter().map(|range| range.1)).max().unwrap() + 2;
        Cave {
            rocks,
            rocks_set,
            sands: Vec::<Sand>::new(),
            floor,
            is_done: false,
            sands_finalpos: HashSet::new(),
        }
    }

    pub fn step(&mut self, part: Part) {
        if self.sands.is_empty() || self.sands[self.sands.len() - 1].landed {
            self.sands.push(Sand::new());
            // println!("self.sands.len(): {0:?}", self.sands.len());
        }
        let i = self.sands.len() - 1;
        let mut moved: bool = false;

        for new_pos in self.sands[i].get_new_pos() { //try and move either down, left down or right down
            if !self.pos_is_blocked(&new_pos, &part) {
                self.sands[i].pos = new_pos;
                moved = true;
                break;
            }
        }

        if !moved { //if not moved, then sand has landed
            self.sands[i].landed = true;
            self.sands_finalpos.insert(self.sands[i].pos);
        }
        //Stop condition
        match part {
            Part::A => {
                if self.sands[i].pos.1 > self.floor {
                    self.is_done = true;
                }
            },
            Part::B => {
                if self.sands[i].landed && self.sands[i].pos == (500, 0) {
                    self.is_done = true;
                }
            }
        }
        
    }

    pub fn pos_is_blocked_old(&self, new_pos: &(i32, i32), part: &Part) -> bool {
        match part {
            Part::A => {
                self.rocks.iter().any(|rock| rock.blocks_sand(new_pos)) || //rock blocks path
                self.sands.iter().rev().any(|sand| sand.pos == *new_pos) //sand already there
            }
            Part::B => {
                new_pos.1 >= self.floor || 
                self.rocks.iter().any(|rock| rock.blocks_sand(new_pos)) || //rock blocks path
                self.sands.iter().rev().any(|sand| sand.pos == *new_pos) //sand already there
            }
        }
    }

    pub fn pos_is_blocked(&self, new_pos: &(i32, i32), part: &Part) -> bool {
        match part {
            Part::A => {
                // self.rocks.iter().any(|rock| rock.blocks_sand(new_pos)) || //old
                self.rocks_set.contains(new_pos) || 
                self.sands_finalpos.contains(new_pos)
            }
            Part::B => {
                new_pos.1 >= self.floor ||
                // self.rocks.iter().any(|rock| rock.blocks_sand(new_pos)) || //old
                self.rocks_set.contains(new_pos) || 
                self.sands_finalpos.contains(new_pos)
            }
        }
    }
}

#[derive(Debug)]
struct Rocks {
    ranges: Vec<(i32, i32)>,
    hash_pos: HashSet<(i32, i32)>
}

impl Rocks {
    pub fn new(input_line: &str) -> Self {
        let ranges = Self::parse_line(input_line);
        let hash_pos = Self::gen_hashset(&ranges);
        Self {
            ranges,
            hash_pos,
        }
    }

    fn parse_line(input_line: &str) -> Vec<(i32, i32)> {
        input_line
            .split(" -> ")
            .flat_map(|coords| {
                coords
                    .split(',')
                    .tuples()
                    .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            })
            .collect()
    }

    ///Create hashset containing all coordinates of input paths
    fn gen_hashset(ranges: &[(i32, i32)]) -> HashSet<(i32, i32)>{
        let mut h = HashSet::<(i32, i32)>::new();
        ranges.windows(2).map(|w| (w[0], w[1])).for_each(|(pos0, pos1)| {
            let high = (std::cmp::max(pos0.0, pos1.0), std::cmp::max(pos0.1, pos1.1)); //get high x and y
            let low = (std::cmp::min(pos0.0, pos1.0), std::cmp::min(pos0.1, pos1.1)); //get low x and y
            (low.0..=high.0).for_each(|x| { //add horizontal path
                h.insert((x, low.1));
            });

            (low.1..=high.1).for_each(|y| { //add vertical path
                h.insert((low.0, y));
            });
        });
        h
    }

    ///Check if this rock path blocks a position
    fn blocks_sand_old(&self, sand_trypos: (i32, i32)) -> bool {
        self.ranges.windows(2).map(|w| (w[0], w[1])).any(|(pos0, pos1)| {
            // get high/low x and y so that later comparisons can just compare like `low.x <= sand.x <= high.x`
            let high = (std::cmp::max(pos0.0, pos1.0), std::cmp::max(pos0.1, pos1.1)); //get high x and y
            let low = (std::cmp::min(pos0.0, pos1.0), std::cmp::min(pos0.1, pos1.1)); //get low x and y
            if low.0 <= sand_trypos.0  && sand_trypos.0 <= high.0 && sand_trypos.1 == low.1 || 
               low.1 <= sand_trypos.1  && sand_trypos.1 <= high.1 && sand_trypos.0 == low.0 {
                return true;
            }
            false
        })
    }

    fn blocks_sand(&self, sand_trypos: &(i32, i32)) -> bool {
        self.hash_pos.contains(sand_trypos)
    }

}

#[derive(Debug)]
struct Sand {
    pos: (i32, i32),
    landed: bool,
}

impl Sand {
    pub fn new() -> Self {
        Sand { pos: (500, 0), landed: false }
    }

    pub fn get_new_pos(&self) -> [(i32, i32); 3] {
        [
            (self.pos.0, self.pos.1 + 1), //straight down
            (self.pos.0 - 1, self.pos.1 + 1), //left down
            (self.pos.0 + 1, self.pos.1 + 1), //right down
        ]
    }
}
