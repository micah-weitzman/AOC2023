// #![allow(unused)]
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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
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

fn help_msg() {
  println!("AOC2023 Solutions in rust by Micah");
  println!(" ");
  println!("Usage: cargo run <day> <part> (test)?");
  println!(" ");
  println!("Notes:");
  println!("    <day> must be in the format 'dayXX' where XX is a number. (ie: day1 or day20)");
  println!("    <part> must either be 'part1' or 'part2'");
  println!("    'test' at then end is optional and will run on test input");
}


fn main() {
  let args: Vec<String> = env::args().collect();

  if args.iter().any(|a| a == "-h" || a == "--help") {
    help_msg();
    return;
  }

  match args.len() {
    1 => {println!("[\x1b[31mError\x1b[0m] Please provide a <day> and <part>"); return;},
    2 => {println!("[\x1b[31mError\x1b[0m] Please provide a <part>"); return;},
    _ => (),
  }

  let day_str: &str = args.get(1).unwrap();
  let part: &str = args.get(2).unwrap();

  if !day_str.starts_with("day") || day_str.split("day").last().unwrap().parse::<usize>().map(|v| v > 25 && v >= 1).unwrap_or(true) {
    println!("[\x1b[31mError\x1b[0m] <day> must be in the format 'dayXX' where XX is day number and 1 <= x <= 25");
    return;
  }

  if !part.eq("part1") && !part.eq("part2") {
    println!("[\x1b[31mError\x1b[0m] <part> must either be 'part1' or 'part2'");
    return;
  }

  let day_num: usize = day_str.split("day").last().unwrap().parse::<usize>().unwrap();

  let filename = match args.get(3) {
    Some(w) if w.eq("test") => "test.txt",
    _ => "input.txt"
  };

  env::set_current_dir(
    Path::new("./src").join(format!("day{}", day_num))
  ).unwrap();

  match (day_num, part) {
    (1, "part1") => day1::part1::main(filename),
    (1, "part2") => day1::part2::main(filename),
    (2, "part1") => day2::part1::main(filename),
    (2, "part2") => day2::part2::main(filename),
    (3, "part1") => day3::part1::main(filename),
    (3, "part2") => day3::part2::main(filename),
    (4, "part1") => day4::part1::main(filename),
    (4, "part2") => day4::part2::main(filename),
    (5, "part1") => day5::part1::main(filename),
    (5, "part2") => day5::part2::main(filename),
    (6, "part1") => day6::part1::main(filename),
    (6, "part2") => day6::part2::main(filename),
    (7, "part1") => day7::part1::main(filename),
    (7, "part2") => day7::part2::main(filename),
    (8, "part1") => day8::part1::main(filename),
    (8, "part2") => day8::part2::main(filename),
    (9, "part1") => day9::part1::main(filename),
    (9, "part2") => day9::part2::main(filename),
    (10, "part1") => day10::part1::main(filename),
    (10, "part2") => day10::part2::main(filename),
    (11, "part1") => day11::part1::main(filename),
    (11, "part2") => day11::part2::main(filename),
    (12, "part1") => day12::part1::main(filename),
    (12, "part2") => day12::part2::main(filename),
    (13, "part1") => day13::part1::main(filename),
    (13, "part2") => day13::part2::main(filename),
    (14, "part1") => day14::part1::main(filename),
    (14, "part2") => day14::part2::main(filename),
    (15, "part1") => day15::part1::main(filename),
    (15, "part2") => day15::part2::main(filename),
    // (16, "part1") => day16::part1::main(filename),
    // (16, "part2") => day16::part2::main(filename),
    // (17, "part1") => day17::part1::main(filename),
    // (17, "part2") => day17::part2::main(filename),
    // (18, "part1") => day18::part1::main(filename),
    // (18, "part2") => day18::part2::main(filename),
    // (19, "part1") => day19::part1::main(filename),
    // (19, "part2") => day19::part2::main(filename),
    // (20, "part1") => day20::part1::main(filename),
    // (20, "part2") => day20::part2::main(filename),
    // (21, "part1") => day21::part1::main(filename),
    // (21, "part2") => day21::part2::main(filename),
    // (22, "part1") => day22::part1::main(filename),
    // (22, "part2") => day22::part2::main(filename),
    // (23, "part1") => day23::part1::main(filename),
    // (23, "part2") => day23::part2::main(filename),
    // (24, "part1") => day24::part1::main(filename),
    // (24, "part2") => day24::part2::main(filename),
    // (25, "part1") => day25::part1::main(filename),
    // (25, "part2") => day25::part2::main(filename),
    _ => println!("[\x1b[33mWarning\x1b[0m] Day not implemented")

  }
}