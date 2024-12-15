use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use crate::utils::{template::Solution, Point};

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Movement {
    fn apply(&self, p: Point) -> Point {
        match self {
            Movement::Up => p.up(),
            Movement::Down => p.down(),
            Movement::Left => p.left(),
            Movement::Right => p.right(),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum MapTile {
    Robot,
    Object,
    LargeObjectLeft,
    LargeObjectRight,
    Wall,
    Empty,
}

impl TryFrom<char> for MapTile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::Object,
            '.' => Self::Empty,
            '[' => Self::LargeObjectLeft,
            ']' => Self::LargeObjectRight,
            _ => return Err(()),
        })
    }
}

impl From<MapTile> for char {
    fn from(value: MapTile) -> Self {
        match value {
            MapTile::Robot => '@',
            MapTile::Object => 'O',
            MapTile::Wall => '#',
            MapTile::Empty => '.',
            MapTile::LargeObjectLeft => '[',
            MapTile::LargeObjectRight => ']',
        }
    }
}

struct Map {
    m: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn new(m: Vec<Vec<char>>) -> Map {
        let max_x = m.first().unwrap().len();
        let max_y = m.len();
        Map { m, max_x, max_y }
    }

    fn get_tile(&self, p: Point) -> MapTile {
        self.m
            .get(p.y)
            .and_then(|r| r.get(p.x))
            .map(|c| MapTile::try_from(*c).expect("Unrecognized map tile"))
            .expect("Index out of range")
    }

    fn write(&mut self, p: Point, t: MapTile) {
        let p = self.m.get_mut(p.y).and_then(|r| r.get_mut(p.x)).unwrap();
        *p = t.into();
    }

    fn can_push(&self, p: Point, m: Movement) -> bool {
        if self.get_tile(p) == MapTile::Empty {
            return true;
        } else if self.get_tile(p) == MapTile::Wall {
            return false;
        } else if self.get_tile(p) == MapTile::LargeObjectLeft
            && (m == Movement::Up || m == Movement::Down)
        {
            return self.can_push(m.apply(p), m) && self.can_push(m.apply(p.right()), m);
        } else if self.get_tile(p) == MapTile::LargeObjectRight
            && (m == Movement::Up || m == Movement::Down)
        {
            return self.can_push(m.apply(p), m) && self.can_push(m.apply(p.left()), m);
        } else {
            return self.can_push(m.apply(p), m);
        }
    }

    fn push(&mut self, t: MapTile, p: Point, m: Movement) {
        let mut already_pushed = HashSet::new();
        let mut push_queue = VecDeque::from([(t, p)]);
        while !push_queue.is_empty() {
            let (t, p) = push_queue.pop_front().unwrap();
            if !already_pushed.insert(p) {
                continue;
            }
            let curr_t = self.get_tile(p);
            self.write(p, t.into());
            if curr_t == MapTile::Empty {
                continue;
            }
            if m == Movement::Up || m == Movement::Down {
                if curr_t == MapTile::LargeObjectLeft {
                    push_queue.push_back((MapTile::Empty, p.right()));
                } else if curr_t == MapTile::LargeObjectRight {
                    push_queue.push_back((MapTile::Empty, p.left()));
                }
            }
            push_queue.push_back((curr_t, m.apply(p)));
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.m.iter() {
            for c in r {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl TryFrom<char> for Movement {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Movement::Up,
            'v' => Movement::Down,
            '<' => Movement::Left,
            '>' => Movement::Right,
            _ => return Err(()),
        })
    }
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let mut split = input.split("\n\n");
        let mut map: Map = Map::new(
            split
                .next()
                .unwrap()
                .lines()
                .map(|l| l.chars().collect())
                .collect(),
        );
        let movements: Vec<Movement> = split
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect();

        let mut robot_pos = Point::new(0, 0);
        for (y, r) in map.m.iter().enumerate() {
            for (x, &c) in r.iter().enumerate() {
                if c == '@' {
                    robot_pos = Point::new(x, y);
                }
            }
        }

        for movement in movements {
            if map.can_push(robot_pos, movement) {
                map.push(MapTile::Empty, robot_pos, movement);
                robot_pos = movement.apply(robot_pos);
            }
        }

        let mut answer = 0;
        for (y, r) in map.m.iter().enumerate() {
            for (x, &c) in r.iter().enumerate() {
                if c == 'O' {
                    answer += x + 100 * y;
                }
            }
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let mut split = input.split("\n\n");
        let mut map_vec: Vec<Vec<char>> = vec![];
        for line in split.next().unwrap().lines() {
            let mut row: Vec<char> = vec![];
            for col in line.chars() {
                let (c1, c2) = match col {
                    '#' => ('#', '#'),
                    '@' => ('@', '.'),
                    'O' => ('[', ']'),
                    '.' => ('.', '.'),
                    _ => panic!(),
                };
                row.push(c1);
                row.push(c2);
            }
            map_vec.push(row);
        }
        let mut map: Map = Map::new(map_vec);
        let movements: Vec<Movement> = split
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect();

        let mut robot_pos = Point::new(0, 0);
        for (y, r) in map.m.iter().enumerate() {
            for (x, &c) in r.iter().enumerate() {
                if c == '@' {
                    robot_pos = Point::new(x, y);
                }
            }
        }

        for movement in movements {
            if map.can_push(robot_pos, movement) {
                map.push(MapTile::Empty, robot_pos, movement);
                robot_pos = movement.apply(robot_pos);
            }
        }

        let mut answer = 0;
        for (y, r) in map.m.iter().enumerate() {
            for (x, &c) in r.iter().enumerate() {
                if c == '[' {
                    answer += x + 100 * y;
                }
            }
        }

        answer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_small() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        assert_eq!("2028", Sln::new().part_1(input.to_string()));
    }

    #[test]
    fn test_part_1() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!("10092", Sln::new().part_1(input.to_string()));
    }

    #[test]
    fn test_part_2() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!("9021", Sln::new().part_2(input.to_string()));
    }
}
