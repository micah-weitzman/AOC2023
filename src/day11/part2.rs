
use crate::utils::read_lines;
use itertools::Itertools;

const N: i64 = 1000000 - 1;

pub fn main(filename: &str) {
  let lines: Vec<String> = read_lines(filename);
  let map: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

  let rows: Vec<usize> = map.iter()
                            .enumerate()
                            .filter_map(|(i, line)|
                              if line.iter().all(|c| *c == '.') { Some(i) }
                              else { None }
                            )
                            .sorted()
                            .collect();

  let cols : Vec<usize> = (0..map[0].len())
                            .filter(|&i| map.iter().all(|line| *line.get(i).unwrap() == '.'))
                            .collect();

  let stars: Vec<(usize, usize)> = map.iter()
                                      .enumerate()
                                      .flat_map(
                                        |(y, row)|
                                          row.iter()
                                              .enumerate()
                                              .filter_map(move  |(x,&c)| if c == '#' { Some((x,y)) } else {None})
                                      )
                                      .collect();

  let res: i64 = stars.into_iter().combinations(2).fold(0, |acc, comb| {
    let (x1, y1): (usize, usize) = comb[0];
    let (x2, y2): (usize, usize) = comb[1];

    let min_x =std::cmp::min(x1,x2);
    let max_x = std::cmp::max(x1,x2);

    let min_y =std::cmp::min(y1,y2);
    let max_y = std::cmp::max(y1,y2);

    let x_count = (min_x..max_x).fold(0, |acc, i| if cols.contains(&i) { acc + 1} else { acc});
    let y_count = (min_y..max_y).fold(0, |acc, i| if rows.contains(&i) { acc + 1} else { acc});

    let dx: i64 = x1 as i64 - x2 as i64;
    let dy: i64 = y1 as i64 - y2 as i64;

    acc + dx.abs() + dy.abs() + (y_count * N) + (x_count * N)
  });

  println!("{}", res); // 710674907809
}
