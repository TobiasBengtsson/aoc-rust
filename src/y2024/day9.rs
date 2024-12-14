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

        let mut a = 0;
        let mut z = hdd.len() - 1;
        while a < z {
            if hdd[a] != -1 {
                a += 1;
                continue;
            }

            if hdd[z] == -1 {
                z -= 1;
                continue;
            }

            hdd[a] = hdd[z];
            hdd[z] = -1;
        }

        let mut chksum: u64 = 0;
        for (i, &b) in hdd.iter().enumerate() {
            if b != -1 {
                chksum += i as u64 * b as u64;
            }
        }

        chksum.to_string()
    }

    fn part_2(&self, input: String) -> String {
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

        let mut z = hdd.len() - 1;
        'outer: while z > 0 {
            if hdd[z] == -1 {
                z -= 1;
                continue;
            }

            let id = hdd[z];
            let mut len = 0;
            while hdd[z] == id {
                len += 1;
                if z == 0 {
                    break 'outer;
                }
                z -= 1;
            }

            let mut a = 0;
            while a < z {
                if hdd[a] != -1 {
                    a += 1;
                    continue;
                }

                let mut free_len = 0;
                let start_a = a;
                while hdd[a] == -1 {
                    a += 1;
                    free_len += 1;
                }

                if free_len >= len {
                    for i in (z + 1)..(z + len + 1) {
                        hdd[i] = -1;
                    }

                    for i in start_a..(start_a + len) {
                        hdd[i] = id;
                    }

                    continue 'outer;
                }
            }
        }

        let mut chksum: u64 = 0;
        for (i, &b) in hdd.iter().enumerate() {
            if b != -1 {
                chksum += i as u64 * b as u64;
            }
        }

        chksum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"2333133121414131402";
        assert_eq!("1928", Sln::new().part_1(input.to_string()))
    }

    #[test]
    fn test_part_2() {
        let input = r"2333133121414131402";
        assert_eq!("2858", Sln::new().part_2(input.to_string()))
    }
}
