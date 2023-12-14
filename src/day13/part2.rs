
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let mut puzzles: Vec<Vec<String>> = vec![];

  let mut curr_vec: Vec<String> = vec![];
  for l in read_lines(filename).into_iter() {
    if l.is_empty() {
      puzzles.push(curr_vec.to_owned());
      curr_vec = Vec::new();
    } else {
      curr_vec.push(l);
    }
  }
  puzzles.push(curr_vec.to_owned());


  let res: u32 = puzzles.iter().map(parse_puzzle).sum();

  println!("{}", res); // 37453
}



fn parse_puzzle(puzzle: &Vec<String>) -> u32 {
  let (prev, is_row) = find_reflection(puzzle);

  for (i,row) in puzzle.iter().enumerate() {
    for (j, c) in row.chars().enumerate() {
      let mut new_puz: Vec<String> = puzzle.clone();
      let line = new_puz.get_mut(i).unwrap();
      *line = {
        let mut rc: Vec<char> = row.clone().chars().collect();
        let tmp = rc.get_mut(j).unwrap();
        *tmp = if c == '.' {'#'} else { '.' };
        rc.into_iter().collect::<String>()
      };

      let new_ref = find_reflection_new(&new_puz, prev, is_row);

      if new_ref != prev && new_ref > 0 {
        return new_ref;
      }
    }
  }

  prev
}

fn find_reflection(puzzle: &Vec<String>) -> (u32, bool) {
  fn parse_rows(puz: &Vec<String>) -> u32 {
    let row_num = puz.len();
    for i in 0..(row_num-1) {
      let mut top: Vec<&String> = puz[0..=i].iter().collect();
      top.reverse();
      let bottom = puz[i+1..].iter();

      if top.iter().zip(bottom).all(|(l,r)| l.eq(&r)) {
        return (i + 1) as u32;
      }
    }
    0
  }

  let rows_res = parse_rows(puzzle);

  if rows_res > 0 { return (100 * rows_res, true); }

  let transpose_puzz: Vec<String> = (0..puzzle[0].len()).map(|i|
    puzzle.iter()
          .map(|l| l.chars().nth(i).unwrap())
          .collect::<String>()
  ).collect();

  return (parse_rows(&transpose_puzz), false);
}

fn find_reflection_new(puzzle: &Vec<String>, prev: u32, was_row: bool) -> u32 {
  let rows_res = parse_rows_new(puzzle, prev, true, was_row);

  if rows_res > 0 { return 100 * rows_res; }

  let transpose_puzz: Vec<String> = (0..puzzle[0].len()).map(|i|
    puzzle.iter()
          .map(|l| l.chars().nth(i).unwrap())
          .collect::<String>()
  ).collect();

  return parse_rows_new(&transpose_puzz, prev, false, was_row);
}


fn parse_rows_new(puz: &Vec<String>, prev: u32, is_row: bool, was_row: bool) -> u32 {
  let row_num = puz.len();
  for i in 0..(row_num-1) {
    let mut top: Vec<&String> = puz[0..=i].iter().collect();
    top.reverse();
    let bottom = puz[i+1..].iter();

    if top.iter().zip(bottom).all(|(l,r)| l.eq(&r)) {
      if (was_row && is_row && (i + 1) as u32 * 100 == prev) || (!was_row && !is_row && (i + 1) as u32 == prev) {
        continue;
      }
      return (i + 1) as u32;
    }
  }
  0
}
