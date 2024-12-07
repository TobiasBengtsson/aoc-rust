use crate::utils::template::Solution;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

fn check_idxs(v: &Vec<Vec<char>>, i1: i32, i2: i32, i3: i32, j1: i32, j2: i32, j3: i32) -> bool {
    if i1 < 0 || i2 < 0 || i3 < 0 || j1 < 0 || j2 < 0 || j3 < 0 {
        return false;
    }
    v.get(i1.unsigned_abs() as usize)
        .and_then(|r| r.get(j1.unsigned_abs() as usize))
        == Some(&'M')
        && v.get(i2.unsigned_abs() as usize)
            .and_then(|r| r.get(j2.unsigned_abs() as usize))
            == Some(&'A')
        && v.get(i3.unsigned_abs() as usize)
            .and_then(|r| r.get(j3.unsigned_abs() as usize))
            == Some(&'S')
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let v: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
        let mut count = 0;
        for (iu, row) in v.clone().iter().enumerate() {
            for (ju, column) in row.clone().iter().enumerate() {
                let i = iu as i32;
                let j = ju as i32;
                if column == &'X' {
                    // Forwards
                    if check_idxs(&v, i, i, i, j + 1, j + 2, j + 3) {
                        count += 1;
                    }

                    // Backwards
                    if check_idxs(&v, i, i, i, j - 1, j - 2, j - 3) {
                        count += 1;
                    }

                    // Downwards
                    if check_idxs(&v, i + 1, i + 2, i + 3, j, j, j) {
                        count += 1;
                    }

                    // Upwards
                    if check_idxs(&v, i - 1, i - 2, i - 3, j, j, j) {
                        count += 1;
                    }

                    // Up-right
                    if check_idxs(&v, i - 1, i - 2, i - 3, j + 1, j + 2, j + 3) {
                        count += 1;
                    }

                    // Up-left
                    if check_idxs(&v, i - 1, i - 2, i - 3, j - 1, j - 2, j - 3) {
                        count += 1;
                    }

                    // Down-left
                    if check_idxs(&v, i + 1, i + 2, i + 3, j - 1, j - 2, j - 3) {
                        count += 1;
                    }

                    // Down-right
                    if check_idxs(&v, i + 1, i + 2, i + 3, j + 1, j + 2, j + 3) {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let v: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
        let mut count = 0;
        for (iu, row) in v.clone().iter().enumerate() {
            for (ju, column) in row.clone().iter().enumerate() {
                let i = iu as i32;
                let j = ju as i32;
                let backslash_match = check_idxs(&v, i, i + 1, i + 2, j, j + 1, j + 2)
                    || check_idxs(&v, i + 2, i + 1, i, j + 2, j + 1, j);
                let forward_slash_match = check_idxs(&v, i + 2, i + 1, i, j, j + 1, j + 2)
                    || check_idxs(&v, i, i + 1, i + 2, j + 2, j + 1, j);
                if backslash_match && forward_slash_match {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}
