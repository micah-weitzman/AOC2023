
use crate::utils::read_lines;

pub fn main() {
  let res: u32 = read_lines("./src/day5/input.txt")
                      .iter()
                      .map(|line| prase_line(line))
                      .sum();
  println!("{}", res);
}

fn prase_line(line: &String) -> u32 {
  0
}