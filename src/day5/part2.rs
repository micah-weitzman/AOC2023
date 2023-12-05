
use crate::utils::read_lines;

pub fn main(filename: &str) {
  read_lines(filename)
                      .iter()
                      .for_each(|l| println!("{}", l));
                      // .map(|line| prase_line(line))
                      // .sum();
  // println!("{}", res);
}

fn prase_line(line: &String) -> u32 {
  0
}