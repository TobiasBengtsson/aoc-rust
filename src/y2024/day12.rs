use crate::utils::template::Solution;
use crate::utils::Point;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }

    fn lookup(map: &Vec<Vec<char>>, p: Point) -> Option<&char> {
        map.get(p.y).and_then(|row| row.get(p.x))
    }

    fn find_area(map: &Vec<Vec<char>>, start: Point, area: &mut HashSet<Point>) {
        if !area.insert(start) {
            return;
        }

        if Self::lookup(map, start) == Self::lookup(map, start.left()) {
            Self::find_area(map, start.left(), area);
        }
        if Self::lookup(map, start) == Self::lookup(map, start.right()) {
            Self::find_area(map, start.right(), area);
        }
        if Self::lookup(map, start) == Self::lookup(map, start.up()) {
            Self::find_area(map, start.up(), area);
        }
        if Self::lookup(map, start) == Self::lookup(map, start.down()) {
            Self::find_area(map, start.down(), area);
        }
    }

    fn calculate_cost(area: &HashSet<Point>) -> usize {
        let mut fence_count = 0;
        // Sort left to right, then top to bottom
        for &p in area
            .iter()
            .sorted_by(|p1, p2| p1.y.cmp(&p2.y).then(p1.x.cmp(&p2.x)))
        {
            if !area.contains(&p.left()) {
                fence_count += 1;
            }
            if !area.contains(&p.right()) {
                fence_count += 1;
            }
            if !area.contains(&p.up()) {
                fence_count += 1;
            }
            if !area.contains(&p.down()) {
                fence_count += 1;
            }
        }

        area.len() * fence_count
    }

    fn calculate_cost_discount(area: &HashSet<Point>) -> usize {
        let mut fence_count = 0;
        // Sort left to right, then top to bottom
        for &p in area
            .iter()
            .sorted_by(|p1, p2| p1.y.cmp(&p2.y).then(p1.x.cmp(&p2.x)))
        {
            if !area.contains(&p.left()) {
                if !(area.contains(&p.up()) && !area.contains(&p.up().left())) {
                    fence_count += 1;
                }
            }
            if !area.contains(&p.right()) {
                if !(area.contains(&p.up()) && !area.contains(&p.up().right())) {
                    fence_count += 1;
                }
            }
            if !area.contains(&p.up()) {
                if !(area.contains(&p.left()) && !area.contains(&p.left().up())) {
                    fence_count += 1;
                }
            }
            if !area.contains(&p.down()) {
                if !(area.contains(&p.left()) && !area.contains(&p.left().down())) {
                    fence_count += 1;
                }
            }
        }

        area.len() * fence_count
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut already_seen = HashSet::<Point>::new();
        let mut areas = Vec::new();

        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let p = Point::new(x, y);
                if already_seen.contains(&p) {
                    continue;
                }

                let mut area = HashSet::new();
                Self::find_area(&map, p, &mut area);
                already_seen.extend(&area);
                areas.push(area);
            }
        }

        let mut answer = 0;
        for area in areas {
            answer += Self::calculate_cost(&area);
        }
        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let mut already_seen = HashSet::<Point>::new();
        let mut areas = Vec::new();

        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let p = Point::new(x, y);
                if already_seen.contains(&p) {
                    continue;
                }

                let mut area = HashSet::new();
                Self::find_area(&map, p, &mut area);
                already_seen.extend(&area);
                areas.push(area);
            }
        }

        let mut answer = 0;
        for area in areas {
            answer += Self::calculate_cost_discount(&area);
        }
        answer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        assert_eq!("1930", Sln::new().part_1(input.to_string()));
    }

    #[test]
    fn test_part_2() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        assert_eq!("1206", Sln::new().part_2(input.to_string()));
    }
}
