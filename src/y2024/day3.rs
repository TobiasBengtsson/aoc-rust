use crate::utils::template::Solution;
use regex::Regex;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut answer = 0;
        for (_, [v1, v2]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
            let x: u32 = v1.parse().unwrap();
            let y: u32 = v2.parse().unwrap();
            answer += x * y;
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut active = true;
        let mut answer = 0;
        for (_, [instruction]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
            if instruction == r"do()" {
                active = true;
                continue;
            }

            if instruction == r"don't()" {
                active = false;
                continue;
            }

            if !active {
                continue;
            }

            for (_, [v1, v2]) in mul_re.captures_iter(instruction).map(|c| c.extract()) {
                let x: u32 = v1.parse().unwrap();
                let y: u32 = v2.parse().unwrap();
                answer += x * y;
            }
        }

        answer.to_string()
    }
}
