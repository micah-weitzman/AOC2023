
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let lines = read_lines(filename);
  let data: Vec<u64> = lines.iter()
                            .map(|line|
                              line.split(":")
                                  .last()
                                  .unwrap()
                                  .trim()
                                  .replace(" ", "")
                                  .parse::<u64>()
                                  .unwrap()
                            ).collect();

  let (time, dist) = (data[0], data[1]);
  let res: u64 = (1..time).filter(|i| (time - i) * i > dist)
                          .count() as u64;

  println!("{}", res); // 33149631
}