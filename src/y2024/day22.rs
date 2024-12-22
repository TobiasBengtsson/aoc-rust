use std::{collections::HashMap, i8, u8};

use crate::utils::template::Solution;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

struct SecretNumber {
    n: u64,
    price: u8,
    diff: i8,
}

impl SecretNumber {
    fn next(&self) -> SecretNumber {
        let a = self.n * 64;
        let b = self.n ^ a;
        let c = b % 16777216;
        let d = c / 32;
        let e = c ^ d;
        let f = e % 16777216;
        let g = f * 2048;
        let h = f ^ g;
        let i = h % 16777216;
        let new_price = (i % 10) as u8;
        SecretNumber {
            n: i,
            price: new_price,
            diff: new_price as i8 - self.price as i8,
        }
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut answer = 0;
        for line in input.lines() {
            let line_int = line.parse().unwrap();
            let mut sn = SecretNumber {
                n: line_int,
                price: (line_int % 10) as u8,
                diff: 0,
            };
            for _ in 0..2000 {
                sn = sn.next();
            }
            answer += sn.n;
        }
        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let mut answers: HashMap<[i8; 4], u64> = HashMap::new();
        let mut valid_seq: Vec<[i8; 4]> = Vec::new();
        for a in -9..9 {
            for b in -9..9 {
                let a_b = a + b;
                if a_b < -9 || a_b > 9 {
                    continue;
                }
                for c in -9..9 {
                    let a_b_c = a_b + c;
                    let b_c = b + c;
                    if a_b_c < -9 || a_b_c > 9 {
                        continue;
                    }
                    if b_c < -9 || b_c > 10 {
                        continue;
                    }
                    for d in -9..9 {
                        let a_b_c_d = a_b_c + d;
                        if a_b_c_d < -9 || a_b_c_d > 9 {
                            continue;
                        }
                        let b_c_d = b_c + d;
                        if b_c_d < -9 || b_c_d > 9 {
                            continue;
                        }
                        let c_d = c + d;
                        if c_d < -9 || c_d > 9 {
                            continue;
                        }

                        valid_seq.push([a, b, c, d]);
                    }
                }
            }
        }

        let mut sn_all: Vec<Vec<(u8, i8)>> = Vec::new();
        for line in input.lines() {
            let mut line_sn_all = Vec::new();
            let line_int = line.parse().unwrap();
            let mut sn = SecretNumber {
                n: line_int,
                price: (line_int % 10) as u8,
                diff: 0,
            };
            for _ in 0..2000 {
                sn = sn.next();
                line_sn_all.push((sn.price, sn.diff));
            }

            sn_all.push(line_sn_all);
        }

        for seq in valid_seq {
            for sn in sn_all.iter() {
                let (mut p0, mut p1, mut p2, mut p3) = (i8::MAX, i8::MAX, i8::MAX, i8::MAX);
                for entry in sn {
                    p3 = p2;
                    p2 = p1;
                    p1 = p0;
                    p0 = entry.1;

                    if seq[0] == p0 && seq[1] == p1 && seq[2] == p2 && seq[3] == p3 {
                        answers
                            .entry(seq)
                            .and_modify(|bananas| *bananas = *bananas + entry.0 as u64)
                            .or_insert(entry.0 as u64);
                        break;
                    }
                }
            }
        }

        let mut best = 0;
        for (_, bananas) in answers {
            if bananas > best {
                best = bananas;
            }
        }

        best.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"1
10
100
2024"#;
        assert_eq!("37327623", Sln::new().part_1(input.to_string()));
    }

    #[test]
    fn test_part_2() {
        let input = r#"1
2
3
2024"#;
        assert_eq!("23", Sln::new().part_2(input.to_string()));
    }
}
