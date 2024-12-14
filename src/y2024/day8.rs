use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::utils::{template::Solution, Point};

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }

    fn read_clusters(input: String) -> (HashMap<char, AntennaCluster>, usize, usize) {
        let mut clusters: HashMap<char, AntennaCluster> = HashMap::new();
        let mut y_max = 0;
        let mut x_max = 0;
        for (y, row) in input.lines().enumerate() {
            y_max = y;
            for (x, cell) in row.chars().enumerate() {
                x_max = x;
                if cell != '.' {
                    if !clusters.contains_key(&cell) {
                        clusters.insert(cell, AntennaCluster::new());
                    }
                    clusters.get_mut(&cell).unwrap().insert(Point::new(x, y));
                }
            }
        }

        (clusters, x_max, y_max)
    }
}

struct AntennaCluster {
    locations: HashSet<Point>,
}

impl AntennaCluster {
    fn new() -> AntennaCluster {
        AntennaCluster {
            locations: HashSet::new(),
        }
    }

    fn insert(&mut self, p: Point) {
        self.locations.insert(p);
    }

    fn get_antinodes(
        &self,
        resonant_harmonics: bool,
        x_max: usize,
        y_max: usize,
    ) -> HashSet<Point> {
        let mut antinodes = HashSet::new();
        for (i, &p1) in self.locations.iter().enumerate() {
            for &p2 in self.locations.iter().skip(i + 1) {
                if resonant_harmonics {
                    antinodes.insert(p1);
                    antinodes.insert(p2);
                }
                let direction = p1 - p2;
                let mut an1 = p1 + direction;
                let mut an2 = p2 - direction;
                while an1.x <= x_max && an1.y <= y_max {
                    antinodes.insert(an1);
                    if !resonant_harmonics {
                        break;
                    }
                    an1 = an1 + direction;
                }

                while an2.x <= x_max && an2.y <= y_max {
                    antinodes.insert(an2);
                    if !resonant_harmonics {
                        break;
                    }
                    an2 = an2 - direction;
                }
            }
        }

        antinodes
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let (clusters, x_max, y_max) = Sln::read_clusters(input);

        let antinodes = Arc::new(Mutex::new(HashSet::new()));
        rayon::scope(|s| {
            for antenna_cluster in clusters.values() {
                let antinodes_clone = antinodes.clone();
                s.spawn(move |_| {
                    let cluster_antinodes = antenna_cluster.get_antinodes(false, x_max, y_max);
                    antinodes_clone.lock().unwrap().extend(cluster_antinodes);
                });
            }
        });

        let x = antinodes.lock().unwrap().len().to_string();
        x
    }

    fn part_2(&self, input: String) -> String {
        let (clusters, x_max, y_max) = Sln::read_clusters(input);

        let antinodes = Arc::new(Mutex::new(HashSet::new()));
        rayon::scope(|s| {
            for antenna_cluster in clusters.values() {
                let antinodes_clone = antinodes.clone();
                s.spawn(move |_| {
                    let cluster_antinodes = antenna_cluster.get_antinodes(true, x_max, y_max);
                    antinodes_clone.lock().unwrap().extend(cluster_antinodes);
                });
            }
        });

        let x = antinodes.lock().unwrap().len().to_string();
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sln_part1() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let answer = Sln::new().part_1(input.to_string());
        assert_eq!("14", answer);
    }

    #[test]
    fn test_sln_part2() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let answer = Sln::new().part_2(input.to_string());
        assert_eq!("34", answer);
    }
}
