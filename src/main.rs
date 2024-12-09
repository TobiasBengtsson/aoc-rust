#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

mod utils;
mod y2024;

use crate::utils::template::Solution;
use chrono::Datelike;
use clap;
use clap::Parser;
use figment::providers::{Env, Format, Toml};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    pub session_token: String,
    pub input_file_dir: PathBuf,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: Option<i32>,
    #[arg(short, long)]
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let config: Config = figment::Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("AOC_RUST_"))
        .extract()
        .unwrap();
    if !&config.input_file_dir.is_dir() {
        panic!("input_file_dir is not a directory");
    }
    let now = chrono::Utc::now();
    let year = args.year.unwrap_or(now.year());
    let day = args.day.unwrap_or(now.day());
    let input = get_input(year, day, &config);
    let solution = get_solution(year, day);
    println!("Part 1: {}", solution.part_1(input.clone()));
    println!("Part 2: {}", solution.part_2(input.clone()))
}

fn get_aoc_download_url(year: i32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

fn get_input(year: i32, day: u32, config: &Config) -> String {
    std::fs::create_dir_all(&config.input_file_dir.join(format!("{}", year))).unwrap();
    let path = &config
        .input_file_dir
        .join(format!("{}/day{}.txt", year, day));
    if path.exists() {
        return std::fs::read_to_string(path).unwrap();
    }
    let url = get_aoc_download_url(year, day);
    let session_cookie = format!("session={}", config.session_token);
    let res = ureq::get(&url)
        .set("Cookie", session_cookie.as_str())
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    std::fs::write(path, res.clone()).unwrap();

    res
}

fn get_solution(year: i32, day: u32) -> Box<dyn Solution> {
    match year {
        2024 => match day {
            1 => Box::new(y2024::day1::Sln::new()),
            2 => Box::new(y2024::day2::Sln::new()),
            3 => Box::new(y2024::day3::Sln::new()),
            4 => Box::new(y2024::day4::Sln::new()),
            5 => Box::new(y2024::day5::Sln::new()),
            6 => Box::new(y2024::day6::Sln::new()),
            7 => Box::new(y2024::day7::Sln::new()),
            8 => Box::new(y2024::day8::Sln::new()),
            9 => Box::new(y2024::day9::Sln::new()),
            10 => Box::new(y2024::day10::Sln::new()),
            11 => Box::new(y2024::day11::Sln::new()),
            12 => Box::new(y2024::day12::Sln::new()),
            13 => Box::new(y2024::day13::Sln::new()),
            14 => Box::new(y2024::day14::Sln::new()),
            15 => Box::new(y2024::day15::Sln::new()),
            16 => Box::new(y2024::day16::Sln::new()),
            17 => Box::new(y2024::day17::Sln::new()),
            18 => Box::new(y2024::day18::Sln::new()),
            19 => Box::new(y2024::day19::Sln::new()),
            20 => Box::new(y2024::day20::Sln::new()),
            21 => Box::new(y2024::day21::Sln::new()),
            22 => Box::new(y2024::day22::Sln::new()),
            23 => Box::new(y2024::day23::Sln::new()),
            24 => Box::new(y2024::day24::Sln::new()),
            25 => Box::new(y2024::day25::Sln::new()),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
