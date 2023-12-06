
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let lines = read_lines(filename);
  let data: Vec<Vec<u32>> = lines.iter()
                                  .map(|line|
                                    line.split(":")
                                        .last()
                                        .unwrap()
                                        .trim()
                                        .split(" ")
                                        .filter_map(|n| n.trim().parse::<u32>().ok())
                                        .collect()
                              ).collect();
  let (times, distances) = (data.first().unwrap().to_owned(), data.last().unwrap().to_owned());
  let races: Vec<(u32, u32)> = times.into_iter().zip(distances.into_iter()).collect();

  let res: u32 = races.into_iter()
                      .fold(1, |acc, (time, dist)|
                        ((1..time).filter(|i| (time - i) * i > dist).count() as u32 ) * acc
                      );

  println!("{}", res); // 2449062
}
