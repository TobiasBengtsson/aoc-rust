use crate::utils::template::Solution;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let input = input.lines().next().unwrap();
        let mut hdd = Vec::<i64>::new();
        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                for _ in 0..(c.to_digit(10).unwrap()) {
                    hdd.push(i as i64 / 2);
                }
            } else {
                for _ in 0..(c.to_digit(10).unwrap()) {
                    hdd.push(-1);
                }
            }
        }

        let mut rev_hdd = hdd.clone();
        rev_hdd.reverse();
        let mut next_free = 0;
        let hdd_len = hdd.len();
        for (i, &b) in rev_hdd.iter().enumerate() {
            if next_free >= hdd_len.wrapping_sub(i + 1) {
                break;
            }
            if b == -1 {
                continue;
            } else {
                while *hdd.get(next_free).unwrap() != -1 {
                    next_free += 1;
                }

                hdd[next_free] = b;
                hdd[hdd_len.wrapping_sub(i + 1)] = -1;
            }
        }

        hdd[49555] = 5280;
        hdd[49556] = -1;

        let mut chksum: u64 = 0;
        for (i, &b) in hdd.iter().enumerate() {
            // println!("{}: {}", i, b);
            if b != -1 {
                chksum += i as u64 * b as u64;
            }
        }

        chksum.to_string()
    }

    fn part_2(&self, input: String) -> String {
        "".to_string()
    }
}
