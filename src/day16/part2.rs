
use crate::utils::read_lines;
use std::{collections::HashSet, vec};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
  Left,
  Right,
  Up,
  Down,
}

pub fn main(filename: &str) {
  use Dir::*;
  let map: Vec<Vec<char>> = read_lines(filename).iter().map(|s| s.chars().collect()).collect();

  let mut res: u32 = 0;

  for i in 0..map.len() {
    let pwr_r = clac_power(0, i as i32, Right, &map);
    let pwr_l = clac_power(map[0].len() as i32, i as i32, Left, &map);
    res = std::cmp::max(res, std::cmp::max(pwr_l, pwr_r));
  }

  for i in 0..map[0].len() {
    let pwr_r = clac_power(i as i32, 0 as i32, Down, &map);
    let pwr_l = clac_power(i as i32, map.len() as i32, Up, &map);
    res = std::cmp::max(res, std::cmp::max(pwr_l, pwr_r));
  }


  println!("{}", res); // 8221
}


fn clac_power(x: i32, y: i32, d: Dir, map: &Vec<Vec<char>>) -> u32 {
  use Dir::*;

  let mut seen: HashSet<(i32, i32, Dir)> = HashSet::new();

  let mut active: Vec<(i32, i32, Dir)> = vec![(x, y, d)];

  while !active.is_empty() {
    let mut next_block: Vec<(i32, i32, Dir)> = vec![];

    for (i, j, dir) in active.iter() {
      if *j as usize >= map.len() || (*j as i32) < 0 || (*i as i32) < 0 || *i >= map[0].len() as i32 {
        continue;
      }

      if !seen.insert((*i, *j, *dir)) {
        continue;
      }

      let curr_block = *map.get(*j as usize).unwrap().get(*i as usize).unwrap();

      match (curr_block, *dir) {
        ('/', Right) => next_block.push((*i, *j-1, Up)),
        ('/', Left) => next_block.push((*i, *j+1, Down)),
        ('/', Up) => next_block.push((*i+1, *j, Right)),
        ('/', Down) => next_block.push((*i-1, *j, Left)),
        ('\\', Right) => next_block.push((*i, *j+1, Down)),
        ('\\', Left) => next_block.push((*i, *j-1, Up)),
        ('\\', Up) => next_block.push((*i-1, *j, Left)),
        ('\\', Down) => next_block.push((*i+1, *j, Right)),
        ('|', Right) => {
          next_block.push((*i, *j - 1, Up));
          next_block.push((*i, *j + 1, Down));
        }
        ('|', Left) => {
          next_block.push((*i, *j - 1, Up));
          next_block.push((*i, *j + 1, Down));
        }
        ('-', Up) => {
          next_block.push((*i - 1, *j, Left));
          next_block.push((*i + 1, *j, Right));
        }
        ('-', Down) => {
          next_block.push((*i - 1, *j, Left));
          next_block.push((*i + 1, *j, Right));
        }
        (_, Right) => next_block.push((*i + 1, *j, *dir)),
        (_, Left) => next_block.push((*i - 1, *j, *dir)),
        (_, Up) => next_block.push((*i, *j - 1, *dir)),
        (_, Down) => next_block.push((*i, *j + 1, *dir)),
      };
    }
    active = next_block.clone();
  }

  let mut  new_seen: HashSet<(usize, usize)> = HashSet::new();

  for (i, j, _) in seen.into_iter() {
    new_seen.insert((i as usize, j as usize));
  }

  let res = new_seen.len();
  return res as u32;
}