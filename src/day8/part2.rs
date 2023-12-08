
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

fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 { return a; }
  return gcd(b, a % b);
}
fn lcm(nums: &[u64]) -> u64 {
  nums.iter().fold(1, |acc, &x|
    ((( (x * acc) as i64) / (gcd(x, acc) as i64)) as u64)
  )
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
                                  .replace("(", "")
                                  .replace(")", "")
                                  .replace(" ", "")
                                  .split(",")
                                  .map(String::from)
                                  .collect();
    let neighbors = Neighbor::new(&neigh[0], &neigh[1]);
    map.insert(start.to_string(), neighbors);
  });

  let mut step: u64 = 0;
  let mut dirs_it = dirs.cycle();

  let mut starting: Vec<&String> = map.keys().filter(|s| s.ends_with('A')).collect();
  // dbg!(&starting);
  let mut ending_steps: Vec<u64> =  vec![];
  let done_str = "DONE".to_string();

  while !starting.iter().all(|&s| s.eq(&done_str)) {
    let next_dir = dirs_it.next().unwrap();
    starting.iter_mut().for_each(|s| {
      if (*s).eq("DONE") { return; }
      if !s.ends_with('Z') {
        match next_dir {
          'L' => *s = &map.get(*s).unwrap().left,
          'R' => *s = &map.get(*s).unwrap().right,
          _ => panic!("Unknown direction")
        }
      } else {
        *s = &done_str;
        ending_steps.push(step);
      }
    }
    );
    step += 1;
  }

  let res = lcm(&ending_steps);
  println!("{}", res); // 10818234074807
}