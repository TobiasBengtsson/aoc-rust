use crate::utils::template::Solution;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub struct Sln {}

type IntType = usize;

struct ForbiddenFollowerMap {
    map: HashMap<IntType, HashSet<IntType>>,
}

impl ForbiddenFollowerMap {
    fn new() -> Self {
        ForbiddenFollowerMap {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: IntType, forbidden_follower: IntType) {
        match self.map.get_mut(&key) {
            Some(sub_map) => {
                sub_map.insert(forbidden_follower);
            }
            None => {
                self.map
                    .insert(key, HashSet::from_iter([forbidden_follower]));
            }
        };
    }

    fn check_forbidden(&self, key: IntType, maybe_forbidden: IntType) -> bool {
        match self.map.get(&key) {
            Some(sub_map) => sub_map.contains(&maybe_forbidden),
            None => false,
        }
    }

    fn check_any_forbidden(&self, key: IntType, to_check: &[IntType]) -> bool {
        match self.map.get(&key) {
            Some(sub_map) => {
                for c in to_check {
                    if sub_map.contains(c) {
                        return true;
                    }
                }

                false
            }
            None => false,
        }
    }
}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }

    fn check_ok(forbidden_followers: &ForbiddenFollowerMap, u: &Vec<usize>) -> bool {
        // let mut forbidden = HashSet::new();
        for (i, entry) in u.iter().enumerate() {
            if forbidden_followers.check_any_forbidden(*entry, &u[i..]) {
                return false;
            }
        }

        true
    }

    fn check_ok_ref(rules: &Vec<(usize, usize)>, u: &Vec<&usize>) -> bool {
        let mut ok = true;
        for rule in rules {
            let i1 = u.iter().position(|&x| *x == rule.0).unwrap_or(0);
            let i2 = u.iter().position(|&x| *x == rule.1).unwrap_or(usize::MAX);
            if i1 > i2 {
                ok = false;
            }
        }
        ok
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut lines = input.lines();
        let mut forbidden_followers = ForbiddenFollowerMap::new();
        let mut updates: Vec<Vec<usize>> = Vec::new();
        let mut proc_rules = true;
        while let Some(line) = lines.next() {
            if proc_rules && line == "" {
                proc_rules = false;
                continue;
            }

            if proc_rules {
                let mut spl = line.split('|');
                let u1: usize = spl.next().unwrap().parse::<usize>().unwrap();
                let u2: usize = spl.next().unwrap().parse::<usize>().unwrap();
                forbidden_followers.insert(u2, u1);
            } else {
                let mut spl = line.split(',');
                let mut update = Vec::new();
                for s in spl {
                    update.push(s.parse().unwrap());
                }
                updates.push(update);
            }
        }

        let mut correct_updates = Vec::new();
        for u in updates {
            let ok = Self::check_ok(&forbidden_followers, &u);

            if ok {
                correct_updates.push(u);
            }
        }

        let mut answer = 0;
        for u in correct_updates {
            let c = u.iter().count();
            answer += u[c / 2];
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let mut lines = input.lines();
        let mut forbidden_followers = ForbiddenFollowerMap::new();
        let mut updates: Vec<Vec<usize>> = Vec::new();
        let mut proc_rules = true;
        while let Some(line) = lines.next() {
            if proc_rules && line == "" {
                proc_rules = false;
                continue;
            }

            if proc_rules {
                let mut spl = line.split('|');
                let u1: usize = spl.next().unwrap().parse::<usize>().unwrap();
                let u2: usize = spl.next().unwrap().parse::<usize>().unwrap();
                forbidden_followers.insert(u2, u1);
            } else {
                let mut spl = line.split(',');
                let mut update = Vec::new();
                for s in spl {
                    update.push(s.parse().unwrap());
                }
                updates.push(update);
            }
        }

        let mut incorrect_updates = Vec::new();
        for u in updates {
            let ok = Self::check_ok(&forbidden_followers, &u);

            if !ok {
                incorrect_updates.push(u);
            }
        }

        let mut corrected_updates = Vec::new();
        for u in incorrect_updates {
            let mut v = u.clone();
            v.sort_by(|a, b| {
                if forbidden_followers.check_forbidden(*a, *b) {
                    Ordering::Greater
                } else if forbidden_followers.check_forbidden(*b, *a) {
                    Ordering::Less
                } else {
                    a.cmp(b)
                }
            });

            corrected_updates.push(v);
        }

        let mut answer = 0;
        for u in corrected_updates {
            let c = u.iter().count();
            answer += u[c / 2];
        }

        answer.to_string()
    }
}
