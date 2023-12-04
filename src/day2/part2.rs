use regex::Regex;
use std::cmp;

use crate::utils::read_lines;


// only 12 red cubes, 13 green cubes, and 14 blue cubes
// sum of all possible

pub fn main() {
  let  res: i32 = read_lines("./src/day2/input.txt")
                  .iter()
                  .map(|line| prase_line(line))
                  .sum();
  println!("{}", res);
}

fn prase_line(line: &String) -> i32 {
  let caps = Regex::new(r"Game (?<game_no>[0-9]+):").unwrap().captures(line).unwrap();

  let s = line.split(": ").last().unwrap().to_owned();
  let games: Vec<&str> = s.split("; ").collect();

  let mut min_red = 0;
  let mut min_green = 0;
  let mut min_blue = 0;

  for g in games {
    if let Some(d) = Regex::new(r"(?<num>\d+) red").unwrap().captures(g) {
      let num = &d["num"].parse::<i32>().unwrap();
      min_red = cmp::max(min_red, *num);
    }
    if let Some(d) = Regex::new(r"(?<num>\d+) green").unwrap().captures(g) {
      let num = &d["num"].parse::<i32>().unwrap();
      min_green = cmp::max(min_green, *num);
    }
    if let Some(d) = Regex::new(r"(?<num>\d+) blue").unwrap().captures(g) {
      let num = &d["num"].parse::<i32>().unwrap();
      min_blue = cmp::max(min_blue, *num);
    }
  }

  min_red * min_green * min_blue
}