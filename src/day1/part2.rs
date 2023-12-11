use regex::Regex;

use crate::utils::read_lines;

pub fn main(filename: &str) {
  let res: u32 = read_lines(filename).into_iter().map(prase_line).sum();
  println!("{}", res);
}

fn to_num(m: &str) -> u32{
  match m {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
    c => c.parse::<u32>().unwrap()
  }
}

fn prase_line(line: String) -> u32 {
  let patterns = ["one","two","three","four","five","six","seven","eight","nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

  let last: u32 = patterns.iter()
                          .filter_map(|pat| line.rfind(pat).map(|loc| (loc, to_num(pat))))
                          .max_by_key(|x| x.0)
                          .unwrap()
                          .1;

  let first = to_num(
    Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
          .unwrap()
          .find(&line)
          .unwrap()
          .as_str()
  );

  first * 10 + last
}