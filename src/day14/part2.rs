
use std::collections::HashMap;
use itertools::Itertools;

use crate::utils::read_lines;

const TOTAL_ITERS: u64 = 1000000000;

pub fn main(filename: &str) {
  let mut lines: Vec<String> = read_lines(filename);

  let mut hash_map: HashMap<String, u64> = HashMap::new();

  hash_map.insert(lines_to_string(&lines), 0);

  let mut stop_iter = 0;

  for i in 1..TOTAL_ITERS {
    make_north(&mut lines);
    make_west(&mut lines);
    make_south(&mut lines);
    make_east(&mut lines);

    if let Some(old) = hash_map.insert(lines_to_string(&lines), i) {
      stop_iter = old - 1 + ((TOTAL_ITERS - old + 1) % (i - old));
      break;
    }
  }

  let mut final_lines: Vec<String> = vec![];
  for (k,v) in hash_map.into_iter() {
    if v == stop_iter {
      final_lines = k.chars()
                     .chunks(lines[0].len())
                     .into_iter()
                     .map(|chunk| chunk.collect::<String>())
                     .collect::<Vec<String>>();
      break;
    }
  }

  let res: u32 = calc_score(&final_lines);
  println!("{}", res); // 101010
}


fn make_north(lines: &mut Vec<String>) {
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

  *lines = new_map;
}


fn make_south(lines: &mut Vec<String>) {
  let mut new_map = lines.clone();

  for (i, l) in lines.iter().enumerate().rev() {
    if i == lines.len() { continue; }
    let mut new_line: Vec<char> = l.chars().collect();
    let mut changed = false;

    for (j, c) in new_line.iter_mut().enumerate() {
      let mut row = i;
      if *c == 'O' {
        while row + 1 < lines.len() && new_map.get(row + 1).unwrap().chars().nth(j).unwrap() == '.' { row += 1; }
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

  *lines = new_map;
}

fn make_east(lines: &mut Vec<String>) {
  for l in lines.iter_mut() {
    if l.find('O').is_none() { continue }

    let old_line: Vec<char> = l.chars().collect();
    let mut new_line: Vec<char> = l.chars().collect();
    let mut changed = false;

    for (j, c) in old_line.iter().enumerate().rev() {
      if *c == 'O' {
        let mut col = j;
        while col < new_line.len() - 1 && new_line.get(col + 1).unwrap() == &'.' { col += 1; }

        if col != j {
          changed = true;

          let curr_char = new_line.get_mut(j).unwrap();
          *curr_char = '.';

          let char_to_change = new_line.get_mut(col).unwrap();
          *char_to_change = 'O';
        }
      }
    }

    if changed {
      *l = new_line.iter().collect::<String>();
    }
  }
}

fn make_west(lines: &mut Vec<String>) {
  for l in lines.iter_mut() {
    if l.find('O').is_none() { continue }

    let old_line: Vec<char> = l.chars().collect();
    let mut new_line: Vec<char> = l.chars().collect();
    let mut changed = false;

    for (j, c) in old_line.iter().enumerate() {
      if *c == 'O' {
        let mut col = j;
        while col as i32 > 0 && new_line.get(col - 1).unwrap() == &'.' { col -= 1; }

        if col != j {
          changed = true;

          let curr_char = new_line.get_mut(j).unwrap();
          *curr_char = '.';

          let char_to_change = new_line.get_mut(col).unwrap();
          *char_to_change = 'O';
        }
      }
    }

    if changed {
      *l = new_line.iter().collect::<String>();
    }
  }
}

fn calc_score(lines: &Vec<String>) -> u32 {
  lines.into_iter()
        .rev()
        .enumerate()
        .map(|(i, s)|((i+1) * s.chars().filter(|c| *c == 'O').count()) as u32)
        .sum()
}

fn lines_to_string(lines: &Vec<String>) -> String {
  lines.join("")
}