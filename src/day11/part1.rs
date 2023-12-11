
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

  for i in (0..map[0].len()).rev() {
    let col: Vec<char> = new_map.iter()
                                .map(|line| *line.get(i).unwrap())
                                .collect();
    if col.iter().all(|c| *c == '.') {
      new_map.iter_mut().for_each(|l| l.insert(i, '.'))
    }
  }


  let stars: Vec<(usize, usize)> = new_map.iter()
                                          .enumerate()
                                          .flat_map(
                                            |(y, row)|
                                              row.iter()
                                                  .enumerate()
                                                  .filter_map(move  |(x,&c)| if c == '#' { Some((x,y)) } else {None})
                                          )
                                          .collect();

  let res = stars.into_iter()
                      .combinations(2)
                      .fold(0, |acc, comb| {
                        let (x1, y1) = comb[0];
                        let (x2, y2) = comb[1];
                        let dx: i32 = x1 as i32 - x2 as i32;
                        let dy: i32 = y1 as i32 - y2 as i32;
                        acc + dx.abs() + dy.abs()
                      });

  println!("{}", res); // 9329143
}
