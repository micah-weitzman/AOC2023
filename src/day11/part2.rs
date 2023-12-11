
use crate::utils::read_lines;
use itertools::Itertools;

pub fn main(filename: &str) {
  let lines: Vec<String> = read_lines(filename);
  let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

  let mut rows : Vec<usize> = vec![];
  let mut cols : Vec<usize> = vec![];

  for (i, line) in map.iter().enumerate() {
    if line.iter().all(|c| *c == '.') {
      rows.push(i);
    }
  }

  for i in (0..map[0].len()) {
    let col: Vec<char> = map.iter().map(|line| *line.get(i).unwrap()).collect();
    if col.iter().all(|c| *c == '.') {
      cols.push(i);
    }
  }

  rows.sort();
  cols.sort();

  let mut stars: Vec<(usize, usize)> = vec![];

  for (y,row) in map.iter().enumerate() {
    for (x, c) in row.iter().enumerate() {
      if *c == '#' {
        stars.push((x,y));
      }
    }
  }

  const N: i64 = 1000000 - 1;

  let res: i64 = stars.into_iter().combinations(2).fold(0, |acc, comb| {
    let (x1, y1): (usize, usize) = comb[0];
    let (x2, y2): (usize, usize) = comb[1];

    let mut x_count = 0;
    for i in std::cmp::min(x1,x2)..std::cmp::max(x1,x2) {
      if cols.contains(&i) {
        x_count += 1;
      }
    }
    let mut y_count = 0;
    for j in std::cmp::min(y1,y2)..std::cmp::max(y1,y2) {
      if rows.contains(&j) {
        y_count += 1;
      }
    }

    let dx: i64 = x1 as i64 - x2 as i64;
    let dy: i64 = y1 as i64 - y2 as i64;

    return acc + dx.abs() + dy.abs() + (y_count * N) + (x_count * N);
  });

  println!("{}", res); // 710674907809
}
