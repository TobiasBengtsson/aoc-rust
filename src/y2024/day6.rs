use itertools::Itertools;

use crate::utils::template::Solution;
use crate::y2024::day6::Direction::{East, North, South, West};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

fn get_next_pos(pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
    Some(match *direction {
        North => (pos.0.checked_sub(1)?, pos.1),
        East => (pos.0, pos.1 + 1),
        South => (pos.0 + 1, pos.1),
        West => (pos.0, pos.1.checked_sub(1)?),
    })
}

fn check_for_loop(
    map: &Vec<Vec<char>>,
    mut guard_pos: (usize, usize),
    mut guard_direction: Direction,
) -> bool {
    let mut visited = HashSet::new();
    while guard_pos.0 < map.len() && guard_pos.1 < map[guard_pos.0].len() {
        if guard_pos.0 == 0 && guard_direction == North {
            return false;
        } else if guard_pos.1 == 0 && guard_direction == West {
            return false;
        }

        if !visited.insert((guard_pos, guard_direction)) {
            return true;
        }

        let mut next_pos = match get_next_pos(guard_pos, &guard_direction) {
            Some(pos) => pos,
            None => return false,
        };
        let mut next_pos_char = match map.get(next_pos.0).and_then(|col| col.get(next_pos.1)) {
            Some(pos) => *pos,
            None => return false,
        };
        while next_pos_char == '#' {
            guard_direction = guard_direction.rotate_clockwise();
            next_pos = match get_next_pos(guard_pos, &guard_direction) {
                Some(pos) => pos,
                None => return false,
            };
            next_pos_char = match map.get(next_pos.0).and_then(|col| col.get(next_pos.1)) {
                Some(pos) => *pos,
                None => return false,
            };
        }

        guard_pos = next_pos;
    }

    false
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut guard_pos: (usize, usize) = (0, 0);
        let mut guard_direction = North;
        for (i, row) in map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == '^' {
                    guard_pos = (i, j);
                    guard_direction = North;
                    break;
                } else if *cell == '>' {
                    guard_pos = (i, j);
                    guard_direction = East;
                    break;
                } else if *cell == 'v' {
                    guard_pos = (i, j);
                    guard_direction = South;
                    break;
                } else if *cell == '<' {
                    guard_pos = (i, j);
                    guard_direction = West;
                    break;
                }
            }
        }

        let mut answer = 1;
        map[guard_pos.0][guard_pos.1] = 'X';
        'outer: while guard_pos.0 < map.len() && guard_pos.1 < map[guard_pos.0].len() {
            if guard_pos.0 == 0 && guard_direction == North {
                break;
            } else if guard_pos.1 == 0 && guard_direction == West {
                break;
            }

            let mut next_pos = match get_next_pos(guard_pos, &guard_direction) {
                Some(pos) => pos,
                None => break,
            };
            let mut next_pos_char = match map.get(next_pos.0).and_then(|col| col.get(next_pos.1)) {
                Some(pos) => *pos,
                None => break,
            };
            while next_pos_char == '#' {
                guard_direction = guard_direction.rotate_clockwise();
                next_pos = match get_next_pos(guard_pos, &guard_direction) {
                    Some(pos) => pos,
                    None => break 'outer,
                };
                next_pos_char = match map.get(next_pos.0).and_then(|col| col.get(next_pos.1)) {
                    Some(pos) => *pos,
                    None => break 'outer,
                };
            }

            if next_pos_char == '.' {
                answer += 1;
                map[next_pos.0][next_pos.1] = 'X';
            }

            guard_pos = next_pos;
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut guard_pos: (usize, usize) = (0, 0);
        let mut guard_direction = North;
        for (i, row) in map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == '^' {
                    guard_pos = (i, j);
                    guard_direction = North;
                    break;
                } else if *cell == '>' {
                    guard_pos = (i, j);
                    guard_direction = East;
                    break;
                } else if *cell == 'v' {
                    guard_pos = (i, j);
                    guard_direction = South;
                    break;
                } else if *cell == '<' {
                    guard_pos = (i, j);
                    guard_direction = West;
                    break;
                }
            }
        }

        let guard_starting_pos = guard_pos;
        let guard_starting_dir = guard_direction;
        let possible_entries = Arc::new(Mutex::new(HashSet::new()));
        rayon::scope(|s| {
            'outer: while guard_pos.0 < map.len() && guard_pos.1 < map[guard_pos.0].len() {
                if guard_pos.0 == 0 && guard_direction == North {
                    break;
                } else if guard_pos.1 == 0 && guard_direction == West {
                    break;
                }

                let mut next_pos = match get_next_pos(guard_pos, &guard_direction) {
                    Some(pos) => pos,
                    None => break,
                };
                let mut next_pos_char =
                    match map.get(next_pos.0).and_then(|col| col.get(next_pos.1)) {
                        Some(pos) => *pos,
                        None => break,
                    };
                while next_pos_char == '#' {
                    guard_direction = guard_direction.rotate_clockwise();
                    next_pos = match get_next_pos(guard_pos, &guard_direction) {
                        Some(pos) => pos,
                        None => break 'outer,
                    };
                    next_pos_char = match map.get(next_pos.0).and_then(|col| col.get(next_pos.1)) {
                        Some(pos) => *pos,
                        None => break 'outer,
                    };
                }

                // Will correctly ignore guard starting pos.
                if map[next_pos.0][next_pos.1] == '.'
                    && !possible_entries.lock().unwrap().contains(&next_pos)
                {
                    let mut clone_map = map.clone();
                    let clone_guard_pos = guard_starting_pos;
                    let clone_guard_direction = guard_starting_dir;
                    let possible_entries_clone = possible_entries.clone();
                    s.spawn(move |_| {
                        clone_map[next_pos.0][next_pos.1] = '#';
                        // Need to re-run from the beginning, as adding the obstacle might affect previous paths
                        if check_for_loop(&clone_map, clone_guard_pos, clone_guard_direction) {
                            possible_entries_clone.lock().unwrap().insert(next_pos);
                        }
                    })
                }

                guard_pos = next_pos;
            }
        });

        let x = possible_entries.lock().unwrap().len().to_string();
        x
    }
}
