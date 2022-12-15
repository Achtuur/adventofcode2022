use std::{path::PathBuf, collections::{HashMap, HashSet, BTreeSet}, time::Instant};
use colored::{ColoredString, Colorize};
use itertools::Itertools;
use regex::Regex;
use super::*;

pub struct Day15 {
}

impl Day15 {
    pub fn new() -> Day15 {
        Day15{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day15/test.txt"),
            InputType::Real => PathBuf::from("./data/day15/real.txt"),
        }
    }
}

impl AdventDay for Day15 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        let row = match it {
            InputType::Test => 10,
            InputType::Real => 2000000,
        };
        let mut map = Map::new(&input);
        map.get_blocked_positions(&row).to_string()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        let max_coord = match it {
            InputType::Test => 20,
            InputType::Real => 4000000,
        };
        let mut map = Map::new(&input);
        let p = map.get_unblocked_position(&max_coord);
        (p.0 * 4000000 + p.1).to_string()
    }
}

struct Map {
    sensors: Vec<Sensor>,
    // Beacons: HashSet<(i64, i64)>,
    blocked_positions: HashSet<i64>
}

impl Map {
    pub fn new(input: &str) -> Self {
        let sensors = input.lines().map(Sensor::new).collect_vec();
        Map {
            sensors,
            blocked_positions: HashSet::new(),
        }
    }

    pub fn get_blocked_positions(&mut self, row: &i64) -> usize {
        let mut reserved_pos = HashSet::<i64>::new();
        self.sensors.iter().for_each(|s| {
            if s.pos.1 == *row {
                reserved_pos.insert(s.pos.0);
            }
            if s.beacon_pos.1 == *row {
                reserved_pos.insert(s.beacon_pos.0);
            }
            if let Some(r) = s.block_on_row(row, &-i64::MAX, &i64::MAX) {
                self.blocked_positions.extend(r)
            }
        });
        self.blocked_positions.difference(&reserved_pos).count()
    }

    pub fn get_unblocked_position(&mut self, max_coord: &i64) -> (i64, i64) {
        for s in self.sensors.clone() {
            //for each position 1 outside sensor
                //check if mh distance to all other sensor larger than block distance
                    //if true, position found
            let v = s.get_pos_around_block(&0, max_coord);
            let p = v.iter().filter(|(x, y)| {
                self.sensors.iter().all(|s2| {
                    let mh = (s2.pos.0 - x).abs() + (s2.pos.1 - y).abs();
                    mh > s2.block_dis as i64
                })
            }).collect_vec();

            if !p.is_empty() {
                return *p[0]
            }
        };
        (0, 0)
    }
}


#[derive(Debug, Clone)]
struct Sensor {
    pos: (i64, i64),
    block_dis: i64,
    beacon_pos: (i64, i64),
}

impl Sensor {
    pub fn new(inputline: &str) -> Self {
        let x_reg = Regex::new(r"x=(?P<x>-?\d+)").unwrap();
        let y_reg = Regex::new(r"y=(?P<y>-?\d+)").unwrap();
        let x_cap = x_reg.captures_iter(inputline.trim());
        let y_cap = y_reg.captures_iter(inputline.trim());

        let (x_pos, x_beacon): (i64, i64) = x_cap.map(|cap| cap.name("x").unwrap().as_str().parse().unwrap()).collect_tuple().unwrap();
        let (y_pos, y_beacon): (i64, i64) = y_cap.map(|cap| cap.name("y").unwrap().as_str().parse().unwrap()).collect_tuple().unwrap();
        
        Sensor {
            pos: (x_pos, y_pos),
            beacon_pos: (x_beacon, y_beacon),
            block_dis: (x_pos - x_beacon).abs() + (y_pos - y_beacon).abs()
        }
    }

    pub fn block_on_row(&self, row: &i64, min_coord: &i64, max_coord: &i64) -> Option<std::ops::RangeInclusive<i64>> {
        let md = (self.pos.1 - row).abs(); //manhattan distance to row
        let mut blocked = 0;
        if md < self.block_dis {
            blocked = 2* (self.block_dis - md);
            let min = std::cmp::max(*min_coord, self.pos.0 - blocked/2);
            let max = std::cmp::min(*max_coord, self.pos.0 + blocked/2);
            return Some(min..=max);
        }
        None
    }

    pub fn get_pos_around_block(&self, min_coord: &i64, max_coord: &i64) -> Vec<(i64, i64)> {
        //start at the top of diamond
        let mut y = 1 + self.pos.1 + self.block_dis as i64;
        let mut x = self.pos.0;
        let mut ydir = -1;
        let mut xdir = 1;
        let mut v = Vec::<(i64, i64)>::new();
        //while not at last position (1 pos diagonally left under top)
        while !(y == self.pos.1 + self.block_dis as i64 && x == self.pos.0 - 1) {
            if (*min_coord..=*max_coord).contains(&y) && (*min_coord..=*max_coord).contains(&x) {
                v.push((x, y));
            }
            y += ydir;
            x += xdir;
            let dx = (self.pos.0 - x).abs();
            let dy = (self.pos.1 - y).abs();
            if dx == self.block_dis + 1 {
                xdir *= -1;
            }
            if dy == self.block_dis + 1 {
                ydir *= -1;
            }
        }
        if (*min_coord..=*max_coord).contains(&y) && (*min_coord..=*max_coord).contains(&x) {
            v.push((x, y));
        }
        v
    }
}

#[cfg(test)]
mod tests{
    use super::Sensor;

    #[test]
    fn test_get_pos_around() {
        let s = Sensor { pos: (4,4), block_dis: 2, beacon_pos: (3, 0) };
        let v = s.get_pos_around_block(&0, &20);
        println!("v: {0:?}", v);
    }
}

