
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let res: u64 = read_lines(filename).get(0)
                                     .unwrap()
                                     .split(',')
                                     .map(|s|   s.bytes()
                                     .fold( 0,|acc, c| ((acc + (c as u64)) * 17) % 256))
                                     .sum();
  println!("{}", res); // 505379
}