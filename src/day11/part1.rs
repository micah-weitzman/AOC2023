
use crate::utils::read_lines;
use itertools::Itertools;

pub fn main(filename: &str) {
  let lines: Vec<String> = read_lines(filename);
  let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
  let mut new_map: Vec<Vec<char>> = vec![];

  for (i, line) in map.iter().enumerate() {
    if line.iter().all(|c| *c == '.') {
      new_map.push(line.clone());
    }
    new_map.push(line.clone());
  }

  for i in (0..map[0].len()) {
    let col: Vec<char> = new_map.iter().map(|line| *line.get(i as usize).unwrap()).collect();
    // dbg!(&col);
    if col.iter().all(|c| *c == '.') {
      new_map.iter_mut().for_each(|l| l.insert(i, '.'))
    }
  }

  let mut stars: Vec<(usize, usize)> = vec![];

  for (y,row) in new_map.iter().enumerate() {
    for (x, c) in row.iter().enumerate() {
      if *c == '#' {
        stars.push((x,y));
      }
    }
  }

  let mut res = 0;

  for comb in stars.into_iter().combinations(2) {
    let (x1, y1) = comb[0];
    let (x2, y2) = comb[1];
    let dx: i32 = x1 as i32 - x2 as i32;
    let dy: i32 = y1 as i32 - y2 as i32;
    dbg!(dx.abs(), dy.abs());
    res += dx.abs() + dy.abs();
  }

  println!("{}", res); // 9329143
}
