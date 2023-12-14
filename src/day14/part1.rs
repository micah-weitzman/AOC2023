
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let lines: Vec<String> = read_lines(filename);

  let north_lines = make_north(lines);

  let res: u32 = calc_score(&north_lines);
  println!("{}", res); // 109939
}


fn make_north(lines: Vec<String>) -> Vec<String> {
  let mut new_map = lines.clone();

  for (i, l) in lines.iter().enumerate() {
    if i == 0 { continue; }
    let mut new_line: Vec<char> = l.chars().collect();
    let mut changed = false;

    for (j, c) in new_line.iter_mut().enumerate() {
      let mut row = i;
      if *c == 'O' {
        while (row) as i32 > 0 && new_map.get(row - 1).unwrap().chars().nth(j).unwrap() == '.' { row -= 1; }
      }
      if row != i {
        changed = true;
        *c = '.';

        let row_to_change = new_map.get_mut(row).unwrap();
        *row_to_change = row_to_change.chars().enumerate().map(|(loc, nc)| if loc == j { 'O' } else {nc}).collect::<String>();
      }
    }

    if changed {
      let line = new_map.get_mut(i).unwrap();
      *line = new_line.iter().collect::<String>();
    }
  }

  new_map
}

fn calc_score(lines: &Vec<String>) -> u32 {
  lines.into_iter()
        .rev()
        .enumerate()
        .map(|(i, s)|((i+1) * s.chars().filter(|c| *c == 'O').count()) as u32)
        .sum()
}