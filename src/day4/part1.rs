use std::collections::HashSet;

use crate::utils::read_lines;

pub fn main() {
  let res: u32 = read_lines("./src/day4/input.txt")
                      .iter()
                      .map(|line| prase_line(line))
                      .sum();
  println!("{}", res);
}

fn prase_line(line: &String) -> u32 {
  let s = line.split(": ").last().unwrap().to_owned();
  let all_nums: Vec<HashSet<u32>> = s.split(" | ")
    .map(
      |side| HashSet::<u32>::from_iter(
          side.replace("  ", " ")
              .split(" ")
              .filter(|x| !x.is_empty())
              .map(|dig| String::from(dig).parse::<u32>().unwrap())
      )
    )
    .collect();

  let inter = all_nums[0].intersection(&all_nums[1]).count();
  if inter == 0 {
    return 0;
  }

  return i32::pow(2, (inter - 1) as u32) as u32;
}