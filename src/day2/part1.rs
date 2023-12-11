use regex::Regex;

use crate::utils::read_lines;


// only 12 red cubes, 13 green cubes, and 14 blue cubes
// sum of all possible

pub fn main(filename: &str) {
  let res: u32 = read_lines(filename)
                      .into_iter()
                      .map(prase_line)
                      .sum();
  println!("{}", res);
}

fn prase_line(line: String) -> u32 {
  let caps = Regex::new(r"Game (?<game_no>[0-9]+):").unwrap().captures(&line).unwrap();
  let game_no = &caps["game_no"];

  let s = line.split(": ").last().unwrap().to_owned();
  let games: Vec<&str> = s.split("; ").collect();
  let mut okay = true;

  for g in games {
    if let Some(d) = Regex::new(r"(?<num>\d+) red").unwrap().captures(g) {
      let num = &d["num"].parse::<i32>().unwrap();
      if *num > 14 {
        okay = true;
        break;
      }
    }
    if let Some(d) = Regex::new(r"(?<num>\d+) green").unwrap().captures(g) {
      let num = &d["num"].parse::<i32>().unwrap();
      if *num > 14 {
        okay = true;
        break;
      }
    }
    if let Some(d) = Regex::new(r"(?<num>\d+) blue").unwrap().captures(g) {
      let num = &d["num"].parse::<i32>().unwrap();
      if *num > 14 {
        okay = true;
        break;
      }
    }

  }

  if okay {
    return game_no.parse::<u32>().unwrap();
  }
  0
}