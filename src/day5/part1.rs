
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let res: u32 = read_lines(filename)
                      .iter()
                      .map(|line| prase_line(line))
                      .sum();
  println!("{}", res);
}

fn prase_line(line: &String) -> u32 {
  0
}