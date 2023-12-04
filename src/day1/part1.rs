use crate::utils::read_lines;

pub fn main() {
  let res: u32 = read_lines("./src/day1/input.txt").iter().map(|line| prase_line(line)).sum();
  println!("{}", res);
}

fn prase_line(line: &String) -> u32 {
  let l: Vec<u32> = line.chars()
                          .filter(|c| c.is_ascii_digit())
                          .map(|c| c.to_digit(10).unwrap())
                          .collect();
  let first = *l.first().unwrap();
  let last = *l.last().unwrap();
  first * 10 + last
}