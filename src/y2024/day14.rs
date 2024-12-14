use std::collections::HashMap;

use regex::Regex;

use crate::utils::{template::Solution, Point};

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

#[derive(Debug)]
struct State {
    position: Point,
    velocity: (isize, isize),
}

impl State {
    fn update(&self) -> State {
        State {
            position: Point::new(
                (self.position.x as isize + self.velocity.0).rem_euclid(101) as usize,
                (self.position.y as isize + self.velocity.1).rem_euclid(103) as usize,
            ),
            velocity: self.velocity,
        }
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut states: Vec<State> = vec![];
        for line in input.lines() {
            let r = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
            let c = r.captures(line).unwrap();
            let state = State {
                position: Point::new(
                    c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ),
                velocity: (
                    c.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                    c.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                ),
            };
            states.push(state);
        }

        for _ in 0..100 {
            states = states.iter().map(|s| s.update()).collect();
        }

        let mut qs = HashMap::new();
        for state in states {
            let p = state.position;
            if p.x == 50 || p.y == 51 {
                continue;
            }

            let q = match p.x {
                x if x < 50 => {
                    if p.y < 51 {
                        1
                    } else {
                        3
                    }
                }
                x if x > 50 => {
                    if p.y < 51 {
                        2
                    } else {
                        4
                    }
                }
                _ => 0,
            };

            qs.entry(q).and_modify(|c| *c += 1).or_insert(1);
        }

        (qs[&1] * qs[&2] * qs[&3] * qs[&4]).to_string()
    }

    fn part_2(&self, input: String) -> String {
        let mut states: Vec<State> = vec![];
        for line in input.lines() {
            let r = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
            let c = r.captures(line).unwrap();
            let state = State {
                position: Point::new(
                    c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    c.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ),
                velocity: (
                    c.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                    c.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                ),
            };
            states.push(state);
        }

        for i in 0..10000 {
            states = states.iter().map(|s| s.update()).collect();
            let mut sum_x = 0.0;
            let mut sum_x2 = 0.0;
            let mut sum_y = 0.0;
            let mut sum_y2 = 0.0;
            for state in states.iter() {
                let p = state.position;
                sum_x += p.x as f64;
                sum_x2 += (p.x as f64).powf(2.0);
                sum_y += p.y as f64;
                sum_y2 += (p.y as f64).powf(2.0);
            }

            let n = states.len() as f64;
            let mean_x = sum_x / n;
            let mean_x2 = sum_x2 / n;
            let mean_y = sum_y / n;
            let mean_y2 = sum_y2 / n;
            let var_x = mean_x2 - mean_x.powf(2.0);
            let var_y = mean_y2 - mean_y.powf(2.0);
            if var_x < 600.0 && var_y < 600.0 {
                return (i + 1).to_string();
            }
        }

        "X-Mas tree not found - adjust variance target if necessary".to_string()
    }
}
