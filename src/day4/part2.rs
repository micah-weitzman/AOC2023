use std::collections::{HashSet, HashMap, VecDeque};

use crate::utils::read_lines;

pub fn main() {
  let raw_cards: Vec<Vec<u32>> = read_lines("./src/day4/input.txt")
                      .iter()
                      .enumerate()
                      .map(|(i, line)| prase_line(line, i))
    .collect();

  let cards_len: usize = raw_cards.len();

  let mut cards_orig: HashMap<u32, VecDeque<u32>> = HashMap::new();

  for (i, cards) in raw_cards.into_iter().enumerate() {
      cards_orig.insert(i as u32 + 1, VecDeque::from(cards));
  }

  let mut res: u32 = 0;
  let mut num_each_chard: HashMap<u32, u32> = HashMap::new();

  for k in (1..=cards_len).rev() {
    num_each_chard.insert(k as u32, 1);

    let mut values: VecDeque<u32> = cards_orig.get(&(k as u32)).unwrap().clone();
    while values.len() != 0 {
      let card: u32 = values.pop_front().unwrap();
      let val: &mut u32 = num_each_chard.get_mut(&card).unwrap();
      *val += 1;
      values.append(&mut cards_orig.get(&card).unwrap().clone());
    }
  }

  res = num_each_chard.values().sum();

  println!("{}", res);
}

fn prase_line(line: &String, i: usize) -> Vec<u32> {
  let s = line.split(": ").last().unwrap().to_owned();
  let all_nums: Vec<HashSet<u32>> = s.split(" | ")
    .map(
      |side| HashSet::<u32>::from_iter(
          side.replace("  ", " ")
              .split(" ")
              .filter(|x| !x.is_empty())
              .map(|dig| String::from(dig).parse::<u32>().unwrap())
      )
    )
    .collect();
  let count = all_nums[0].intersection(&all_nums[1]).count();
  if count == 0 {
    return vec![];
  }

  let begin: u32 = i as u32 + 2;
  let end: u32 = i as u32 + 2 + count as u32;
  return Vec::from_iter(begin..end);
}