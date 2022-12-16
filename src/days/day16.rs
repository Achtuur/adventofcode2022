use std::{path::PathBuf, collections::HashMap};
use colored::{ColoredString, Colorize};
use float_ord::FloatOrd;
use itertools::Itertools;
use regex::{Regex, CaptureNames};
use super::*;

pub struct Day16 {
}

impl Day16 {
    pub fn new() -> Day16 {
        Day16{}
    }

    fn get_path(it: &InputType) -> PathBuf {
        match it {
            InputType::Test => PathBuf::from("./data/day16/test.txt"),
            InputType::Real => PathBuf::from("./data/day16/real.txt"),
        }
    }
}

impl AdventDay for Day16 {
    fn A(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        
        let mut vs = ValveSystem::new(&input);
        // println!("vs: {0:#?}", vs);
        while vs.minutes_left > 0 {
            vs.pass_minute()
        }
        println!("vs.released_flow: {0:?}", vs.released_flow);
        todo!()
    }

    fn B(&self, it: &InputType) -> String {
        let input = std::fs::read_to_string(Self::get_path(it)).expect("Reading input failed, file doesn't exist most likely");
        if input.len() < 3 { //arbitrary small value
            println!("{}", "Input file empty, you probably forgot to copy the input data".bold().red());
        }
        todo!();
    }
}
#[derive(Debug)]
struct ValveSystem {
    valves: HashMap<String, Valve>,
    opened: HashMap<String, bool>,
    released_flow: u32,
    current_valve: String,
    minutes_left: u32,
}

impl ValveSystem {
    pub fn new(input: &str) -> Self {
        let mut h = HashMap::<String, Valve>::new();
        let mut o = HashMap::<String, bool>::new();
        let reg = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ((?:[A-Z]{2}(?:,\s)?)+)").unwrap();
        input.lines().for_each(|line| {
            let cap = reg.captures(line).unwrap();
            let (name, flow_rate, lead_to) = (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str());
            let lead_to = lead_to.split(", ").map(|name| name.to_string()).collect_vec();
            let v = Valve::new(flow_rate.parse().unwrap(), lead_to);
            h.insert(name.to_string(), v);
            o.insert(name.to_string(), false);
        });

        ValveSystem {
            valves: h,
            opened: o,
            released_flow: 0,
            current_valve: String::from("AA"),
            minutes_left: 30,
        }
    }

    pub fn pass_minute(&mut self) {
        let weights = self.get_neighbour_weights();
        let (i_max, w_max) = weights.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b)).unwrap();
        println!("-----------------------------------");
        println!("current: {0:?}", self.current_valve);
        println!("neighbours: {0:?}", self.get_current_valve().lead_to);
        println!("weights: {0:?}", weights);
        println!("-----------------------------------");
        if *w_max == 0.0 { //if weight = 0 then all opened valves have been opened, wait until done
            while self.minutes_left > 0 {
                self.add_released_flow();
                self.minutes_left -= 1;
            }
        } else {
            // Move to neighbouring pipe
            self.current_valve = self.get_current_valve().lead_to[i_max].clone();
            self.minutes_left -= 1;
            // self.add_released_flow();
            
            // Open the pipe
            if self.get_current_valve().flow_rate > 0 {
                *self.opened.get_mut(&self.current_valve).unwrap() = true;
                self.minutes_left -= 1;
                self.released_flow += self.get_current_valve().flow_rate * self.minutes_left;
                println!("added flow: {0:?}", self.get_current_valve().flow_rate * self.minutes_left);
                // self.add_released_flow()
            }
        }
    }

    fn add_released_flow(&mut self) {
        self.valves.iter().for_each(|(name, v)| if *self.opened.get(name).unwrap() { self.released_flow += v.flow_rate })
    }

    ///Get weights for all neighbouring valves
    /// 
    /// Assign weight based on closeness to high flow rate
    /// formula: for each unvisited valve: flow_rate / minutes
    pub fn get_neighbour_weights(&self) -> Vec<f32> {
        let mut weights = Vec::<f32>::new();

        self.get_current_valve().lead_to.iter().for_each(|v| {
            let mut vis: Vec<String> = vec![self.current_valve.to_string()];
            // println!("start search");
            let w = self.get_weights(v, &mut vis, 1.0);
            weights.push(w);
        });
        weights
    }

    fn get_weights(&self, start_valve: &str, vis: &mut Vec<String>, depth: f32) -> f32 {
        vis.push(start_valve.to_string());
        let v = self.get_valve(start_valve);
        let mut depth_add = 1.0;
        let mut w = if !self.valve_opened(start_valve) && v.flow_rate != 0 {
            depth_add += 1.0; //add one to depth since valve will be opened
            // println!("v, depth, w: {0:?}", (start_valve, depth, v.flow_rate as f32  * 0.0_f32.max(self.minutes_left as f32 - depth - 1.0)));
            // v.flow_rate as f32  * 0.0_f32.max(self.minutes_left as f32 - 7.0*depth - 1.0)
            v.flow_rate as f32 / (depth)
        } else {
            0.0
        };
        w += v.lead_to.iter().map(|n| {
            if !vis.contains(n) { //if unvisited
                return self.get_weights(n, vis, depth + depth_add);
            }
            0.0
        }).sum::<f32>();
        w
    }

    #[inline]
    fn get_current_valve(&self) -> &Valve {
        self.get_valve(&self.current_valve)
    }

    #[inline]
    fn get_valve(&self, v: &str) -> &Valve {
        self.valves.get(v).unwrap()
    }

    #[inline]
    fn valve_opened(&self, v: &str) -> bool {
        *self.opened.get(v).unwrap()
    }
}
#[derive(Debug)]
struct Valve {
    lead_to: Vec<String>,
    flow_rate: u32,
}

impl Valve {
    pub fn new(flow_rate: u32, lead_to: Vec<String>) -> Self {
        Valve {
            lead_to,
            flow_rate,
        }
    }
}