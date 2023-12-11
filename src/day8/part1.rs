
use std::collections::HashMap;

use crate::utils::read_lines;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Neighbor {
  left: String,
  right: String,
}

impl Neighbor {
  fn new(left: &str, right: &str) -> Self {
    Self { left: left.to_string(), right: right.to_string() }
  }
}

pub fn main(filename: &str) {
  let lines = read_lines(filename);
  let mut map: HashMap<String, Neighbor> = HashMap::new();

  let mut it = lines.iter();
  let dirs = it.next().unwrap().chars();
  it.next();

  it.for_each(|line| {
    let mut split = line.split(" = ");
    let start = split.next().unwrap();
    let neigh: Vec<String> = split.next()
                                  .unwrap()
                                  .replace(['(', ')', ' '], "")
                                  .split(',')
                                  .map(String::from)
                                  .collect();
    let neighbors = Neighbor::new(&neigh[0], &neigh[1]);
    map.insert(start.to_string(), neighbors);
  });

  let mut res: u64 = 0;
  let mut dirs_it = dirs.cycle();

  let mut s = "AAA";
  while !s.eq("ZZZ") {
    match dirs_it.next().unwrap() {
      'L' => s = &map.get(s).unwrap().left,
      'R' => s = &map.get(s).unwrap().right,
      _ => panic!("Unknown direction")
    };
    res += 1;
  }

  println!("{}", res); // 17141
}