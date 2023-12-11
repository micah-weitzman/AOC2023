#![allow(unused)]

use std::env;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();

  let day: &str = &args[1];
  let part: &str = &args[2];

  let filename = match args.get(3) {
    Some(w) if w.eq("test") => "test.txt",
    _ => "input.txt"
  };

  env::set_current_dir(
    Path::new("./src").join(day)
  );

  match (day, part) {
    ("day1", "part1") => day1::part1::main(filename),
    ("day1", "part2") => day1::part2::main(filename),
    ("day2", "part1") => day2::part1::main(filename),
    ("day2", "part2") => day2::part2::main(filename),
    ("day3", "part1") => day3::part1::main(filename),
    ("day3", "part2") => day3::part2::main(filename),
    ("day4", "part1") => day4::part1::main(filename),
    ("day4", "part2") => day4::part2::main(filename),
    ("day5", "part1") => day5::part1::main(filename),
    ("day5", "part2") => day5::part2::main(filename),
    ("day6", "part1") => day6::part1::main(filename),
    ("day6", "part2") => day6::part2::main(filename),
    ("day7", "part1") => day7::part1::main(filename),
    ("day7", "part2") => day7::part2::main(filename),
    ("day8", "part1") => day8::part1::main(filename),
    ("day8", "part2") => day8::part2::main(filename),
    ("day9", "part1") => day9::part1::main(filename),
    ("day9", "part2") => day9::part2::main(filename),
    ("day10", "part1") => day10::part1::main(filename),
    ("day10", "part2") => day10::part2::main(filename),
    // ("day11", "part1") => day11::part1::main(filename),
    // ("day11", "part2") => day11::part2::main(filename),
    // ("day12", "part1") => day12::part1::main(filename),
    // ("day12", "part2") => day12::part2::main(filename),
    // ("day13", "part1") => day13::part1::main(filename),
    // ("day13", "part2") => day13::part2::main(filename),
    // ("day14", "part1") => day14::part1::main(filename),
    // ("day14", "part2") => day14::part2::main(filename),
    // ("day15", "part1") => day15::part1::main(filename),
    // ("day15", "part2") => day15::part2::main(filename),
    // ("day16", "part1") => day16::part1::main(filename),
    // ("day16", "part2") => day16::part2::main(filename),
    // ("day17", "part1") => day17::part1::main(filename),
    // ("day17", "part2") => day17::part2::main(filename),
    // ("day18", "part1") => day18::part1::main(filename),
    // ("day18", "part2") => day18::part2::main(filename),
    // ("day19", "part1") => day19::part1::main(filename),
    // ("day19", "part2") => day19::part2::main(filename),
    // ("day20", "part1") => day20::part1::main(filename),
    // ("day20", "part2") => day20::part2::main(filename),
    // ("day21", "part1") => day21::part1::main(filename),
    // ("day21", "part2") => day21::part2::main(filename),
    // ("day22", "part1") => day22::part1::main(filename),
    // ("day22", "part2") => day22::part2::main(filename),
    // ("day23", "part1") => day23::part1::main(filename),
    // ("day23", "part2") => day23::part2::main(filename),
    // ("day24", "part1") => day24::part1::main(filename),
    // ("day24", "part2") => day24::part2::main(filename),
    // ("day25", "part1") => day25::part1::main(filename),
    // ("day25", "part2") => day25::part2::main(filename),
    _ => panic!("Not implemented")
  }
}