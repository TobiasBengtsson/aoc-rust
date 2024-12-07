use crate::utils::template::Solution;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }

    fn read_into_vecs(input: String, v1: &mut Vec<u32>, v2: &mut Vec<u32>) {
        for line in input.lines() {
            let mut split = line.split_whitespace();
            let val1 = split.next().unwrap().parse::<u32>().unwrap();
            let val2 = split.next().unwrap().parse::<u32>().unwrap();
            v1.push(val1);
            v2.push(val2);
        }
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut v1: Vec<u32> = vec![];
        let mut v2: Vec<u32> = vec![];
        Self::read_into_vecs(input, &mut v1, &mut v2);

        v1.sort_unstable();
        v2.sort_unstable();

        let mut answer = 0;

        for (val1, val2) in v1.iter().zip(v2.iter()) {
            answer += val1.abs_diff(*val2);
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let mut v1: Vec<u32> = vec![];
        let mut v2: Vec<u32> = vec![];
        Self::read_into_vecs(input, &mut v1, &mut v2);

        let mut answer = 0;
        for val1 in v1 {
            for val2 in &v2 {
                if val1 == *val2 {
                    answer += val1
                }
            }
        }

        answer.to_string()
    }
}
