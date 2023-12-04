#![allow(unused)]
#![allow(dead_code)]

use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();

  let day: &str = &args[1];
  let part: &str = &args[2];

  match (day, part) {
    ("day1", "part1") => day1::part1::main(),
    ("day1", "part2") => day1::part2::main(),
    ("day2", "part1") => day2::part1::main(),
    ("day2", "part2") => day2::part2::main(),
    ("day3", "part1") => day3::part1::main(),
    ("day3", "part2") => day3::part2::main(),
    ("day4", "part1") => day4::part1::main(),
    ("day4", "part2") => day4::part2::main(),
    _ => panic!("Not implemented")
  }
}