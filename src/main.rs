#![allow(unused)]
#![allow(dead_code)]

use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
    ("day5", "part1") => day5::part1::main(),
    ("day5", "part2") => day5::part2::main(),
    // ("day6", "part1") => day6::part1::main(),
    // ("day6", "part2") => day6::part2::main(),
    // ("day7", "part1") => day7::part1::main(),
    // ("day7", "part2") => day7::part2::main(),
    // ("day8", "part1") => day8::part1::main(),
    // ("day8", "part2") => day8::part2::main(),
    // ("day9", "part1") => day9::part1::main(),
    // ("day9", "part2") => day9::part2::main(),
    // ("day10", "part1") => day10::part1::main(),
    // ("day10", "part2") => day10::part2::main(),
    // ("day11", "part1") => day11::part1::main(),
    // ("day11", "part2") => day11::part2::main(),
    // ("day12", "part1") => day12::part1::main(),
    // ("day12", "part2") => day12::part2::main(),
    // ("day13", "part1") => day13::part1::main(),
    // ("day13", "part2") => day13::part2::main(),
    // ("day14", "part1") => day14::part1::main(),
    // ("day14", "part2") => day14::part2::main(),
    // ("day15", "part1") => day15::part1::main(),
    // ("day15", "part2") => day15::part2::main(),
    // ("day16", "part1") => day16::part1::main(),
    // ("day16", "part2") => day16::part2::main(),
    // ("day17", "part1") => day17::part1::main(),
    // ("day17", "part2") => day17::part2::main(),
    // ("day18", "part1") => day18::part1::main(),
    // ("day18", "part2") => day18::part2::main(),
    // ("day19", "part1") => day19::part1::main(),
    // ("day19", "part2") => day19::part2::main(),
    // ("day20", "part1") => day20::part1::main(),
    // ("day20", "part2") => day20::part2::main(),
    // ("day21", "part1") => day21::part1::main(),
    // ("day21", "part2") => day21::part2::main(),
    // ("day22", "part1") => day22::part1::main(),
    // ("day22", "part2") => day22::part2::main(),
    // ("day23", "part1") => day23::part1::main(),
    // ("day23", "part2") => day23::part2::main(),
    // ("day24", "part1") => day24::part1::main(),
    // ("day24", "part2") => day24::part2::main(),
    // ("day25", "part1") => day25::part1::main(),
    // ("day25", "part2") => day25::part2::main(),
    _ => panic!("Not implemented")
  }
}