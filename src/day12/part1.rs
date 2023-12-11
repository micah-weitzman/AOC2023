
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let lines: Vec<String> = read_lines(filename);

  let res = lines.len();
  println!("{}", res);
}
