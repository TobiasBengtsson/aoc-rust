use crate::utils::template::Solution;
use std::collections::HashMap;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }

    fn solve(&self, input: String, iterations: usize) -> String {
        let mut star_multiset: HashMap<u64, usize> = HashMap::new();
        for line in input.lines() {
            for star in line.split_whitespace() {
                star_multiset
                    .entry(star.parse::<u64>().unwrap())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        for _ in 0..iterations {
            let mut new_set = HashMap::new();
            for (&number, &count) in star_multiset.iter() {
                if number == 0 {
                    new_set
                        .entry(1)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                    continue;
                }

                let num_digits = number.ilog10() + 1;
                if num_digits % 2 == 0 {
                    let upper = number / 10u64.pow(num_digits / 2);
                    new_set
                        .entry(upper)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                    let lower = number % 10u64.pow(num_digits / 2);
                    new_set
                        .entry(lower)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                    continue;
                }

                new_set
                    .entry(number * 2024)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }

            star_multiset = new_set;
        }

        let mut answer = 0;
        for (&number, &count) in star_multiset.iter() {
            answer += count;
        }

        answer.to_string()
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        self.solve(input, 25).to_string()
    }

    fn part_2(&self, input: String) -> String {
        self.solve(input, 75).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"125 17"#;
        assert_eq!("55312", Sln::new().part_1(input.to_string()));
    }
}
