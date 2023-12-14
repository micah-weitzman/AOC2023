
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

  println!("{}", res); // 29213
}


fn parse_puzzle(puzzle: &Vec<String>) -> u32 {
  let rows_res = find_reflection(puzzle);

  if rows_res > 0 { return 100 * rows_res; }

  let transpose_puzz: Vec<String> = (0..puzzle[0].len()).map(|i|
    puzzle.iter()
          .map(|l| l.chars().nth(i).unwrap())
          .collect::<String>()
  ).collect();

  return find_reflection(&transpose_puzz);
}


fn find_reflection(puz: &Vec<String>) -> u32 {
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