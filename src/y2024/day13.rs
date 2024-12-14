use regex::Regex;

use crate::utils::{template::Solution, Point};

pub struct Sln {}

impl Sln {
    pub fn new() -> Sln {
        Sln {}
    }
}

struct Game {
    btn_a: (u64, u64),
    btn_b: (u64, u64),
    prize: Point,
}

impl Solution for Sln {
    fn part_1(&self, input: String) -> String {
        let button_a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let button_b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_re = Regex::new(r"Prize: X\=(\d+), Y\=(\d+)").unwrap();

        let mut games = vec![];
        for game_text in input.split("\n\n") {
            let button_a_matches = button_a_re.captures(game_text).unwrap();
            let button_a_x = button_a_matches
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_a_y = button_a_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();

            let button_b_matches = button_b_re.captures(game_text).unwrap();
            let button_b_x = button_b_matches
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_b_y = button_b_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();

            let prize_matches = prize_re.captures(game_text).unwrap();
            let prize_x = prize_matches
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let prize_y = prize_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            games.push(Game {
                btn_a: (button_a_x, button_a_y),
                btn_b: (button_b_x, button_b_y),
                prize: Point::new(prize_x, prize_y),
            });
        }

        let mut answer = 0;
        for game in games {
            // x1*a11 + x2*a12 = b1
            // x1*a21 + x2*a22 = b2
            //
            // x2*a22 = b2 - x1*a21
            // x2 = (b2 - x1*a21)/a22
            //
            // x1*a11 + ((b2 - x1*a21)/a22)*a12 = b1
            // x1*a11 + (b2/a22 - x1*a21/a22)*a12 = b1
            // x1*a11 + a12*b2/a22 - a12*x1*a21/a22 = b1
            // x1*a11 - x1*a12*a21/a22 = b1 - a12*b2/a22
            // x1(a11 - a12*a21/a22) = b1 - a12*b2/a22
            // x1 = (b1 - a12*b2/a22) / (a11 - a12*a21/a22)
            // assume no linear dependence and see how it goes...

            let a11 = game.btn_a.0 as f64;
            let a12 = game.btn_b.0 as f64;
            let a21 = game.btn_a.1 as f64;
            let a22 = game.btn_b.1 as f64;
            let b1 = game.prize.x as f64;
            let b2 = game.prize.y as f64;

            let x1 = (b1 - a12 * b2 / a22) / (a11 - a12 * a21 / a22);
            let x2 = (b2 - x1 * a21) / a22;

            let x1_round = x1.round() as u64;
            let x2_round = x2.round() as u64;

            if x1_round * game.btn_a.0 + x2_round * game.btn_b.0 == game.prize.x as u64
                && x1_round * game.btn_a.1 + x2_round * game.btn_b.1 == game.prize.y as u64
            {
                answer += x1_round * 3 + x2_round;
            }
        }

        answer.to_string()
    }

    fn part_2(&self, input: String) -> String {
        let button_a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let button_b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let prize_re = Regex::new(r"Prize: X\=(\d+), Y\=(\d+)").unwrap();

        let mut games = vec![];
        for game_text in input.split("\n\n") {
            let button_a_matches = button_a_re.captures(game_text).unwrap();
            let button_a_x = button_a_matches
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_a_y = button_a_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();

            let button_b_matches = button_b_re.captures(game_text).unwrap();
            let button_b_x = button_b_matches
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let button_b_y = button_b_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();

            let prize_matches = prize_re.captures(game_text).unwrap();
            let prize_x = prize_matches
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let prize_y = prize_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            games.push(Game {
                btn_a: (button_a_x, button_a_y),
                btn_b: (button_b_x, button_b_y),
                prize: Point::new(prize_x + 10000000000000, prize_y + 10000000000000),
            });
        }

        let mut answer = 0u64;
        for game in games {
            // x1*a11 + x2*a12 = b1
            // x1*a21 + x2*a22 = b2
            //
            // x2*a22 = b2 - x1*a21
            // x2 = (b2 - x1*a21)/a22
            //
            // x1*a11 + ((b2 - x1*a21)/a22)*a12 = b1
            // x1*a11 + (b2/a22 - x1*a21/a22)*a12 = b1
            // x1*a11 + a12*b2/a22 - a12*x1*a21/a22 = b1
            // x1*a11 - x1*a12*a21/a22 = b1 - a12*b2/a22
            // x1(a11 - a12*a21/a22) = b1 - a12*b2/a22
            // x1 = (b1 - a12*b2/a22) / (a11 - a12*a21/a22)
            // assume no linear dependence and see how it goes...

            let a11 = game.btn_a.0 as f64;
            let a12 = game.btn_b.0 as f64;
            let a21 = game.btn_a.1 as f64;
            let a22 = game.btn_b.1 as f64;
            let b1 = game.prize.x as f64;
            let b2 = game.prize.y as f64;

            let x1 = (b1 - a12 * b2 / a22) / (a11 - a12 * a21 / a22);
            let x2 = (b2 - x1 * a21) / a22;

            let x1_round = x1.round() as u64;
            let x2_round = x2.round() as u64;

            if x1_round * game.btn_a.0 + x2_round * game.btn_b.0 == game.prize.x as u64
                && x1_round * game.btn_a.1 + x2_round * game.btn_b.1 == game.prize.y as u64
            {
                answer += x1_round * 3 + x2_round;
            }
        }

        answer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        assert_eq!("480", Sln::new().part_1(input.to_string()));
    }
}
