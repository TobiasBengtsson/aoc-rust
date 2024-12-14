use std::collections::HashSet;

use crate::utils::{template::Solution, Point};

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }

    fn hike(map: &Map, start: Point) -> (HashSet<Point>, usize) {
        let current_elevation = match map.elevation(start) {
            Some(e) => e,
            None => return (HashSet::new(), 0),
        };

        if current_elevation >= 9 {
            return (HashSet::from([start]), 1);
        }

        let mut res = HashSet::new();
        let mut paths = 0;
        for p in vec![start.up(), start.down(), start.left(), start.right()] {
            if let Some(e) = map.elevation(p) {
                if e == current_elevation + 1 {
                    let sub = Self::hike(&map, p);
                    res.extend(sub.0);
                    paths += sub.1;
                }
            }
        }

        (res, paths)
    }
}

struct Map {
    m: Vec<Vec<u8>>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn new(m: Vec<Vec<u8>>) -> Map {
        let max_x = m.first().unwrap().len() - 1;
        let max_y = m.len() - 1;
        Map { m, max_x, max_y }
    }

    fn elevation(&self, p: Point) -> Option<u8> {
        if p.x > self.max_x || p.y > self.max_y {
            return None;
        }

        Some(self.m[p.y][p.x])
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut map_vec: Vec<Vec<u8>> = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for digit in line.chars() {
                row.push(digit.to_digit(10).unwrap() as u8);
            }
            map_vec.push(row);
        }

        let map = Map::new(map_vec);
        let mut answer = 0;
        for (y, row) in map.m.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if col == 0 {
                    answer += Self::hike(&map, Point::new(x, y)).0.len();
                }
            }
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let mut map_vec: Vec<Vec<u8>> = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for digit in line.chars() {
                row.push(digit.to_digit(10).unwrap() as u8);
            }
            map_vec.push(row);
        }

        let map = Map::new(map_vec);
        let mut answer = 0;
        for (y, row) in map.m.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if col == 0 {
                    answer += Self::hike(&map, Point::new(x, y)).1;
                }
            }
        }

        answer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!("36", Sln::new().part_1(input.to_string()));
    }

    #[test]
    fn test_part_2() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!("81", Sln::new().part_2(input.to_string()));
    }
}
