use std::collections::HashSet;

use crate::utils::read_lines;

pub fn main(filename: &str) {
  let mut res: u32 = 0;
  let lines = read_lines(filename);

  let mut symbols = Vec::<(usize, usize)>::new();

  for (row, line) in lines.iter().enumerate() {
    for (col, char) in line.char_indices() {
      if char == '*' {
        symbols.push((row, col));
      }
    }
  }

  let dirs: [(i32, i32); 8] = [(1, 1), (1, 0), (-1, -1), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];

  for (x,y) in symbols.iter() {
    let mut seen = HashSet::<(usize, usize)>::new();
    let mut seen_nums = HashSet::<u32>::new();

    for (i, j) in dirs.iter() {

      if (*x as i32 + *i) < 0 || (*y as i32 + *j) < 0  { continue; }
      let row = (*x as i32 + *i) as usize;
      let col = (*y as i32 + *j) as usize;

      if row >= lines.len() || col >= lines[0].len() { continue; }

      if seen.contains(&(row, col)) { continue; }
      if !seen.insert((row, col)) {
        continue;
      }

      if (lines[row].as_bytes()[col] as char).is_ascii_digit() {

        let ref_row: Vec<char> = lines[row].chars().collect();
        let mut l = col;
        let mut r = col;
        while l > 0 && ref_row[l - 1].is_ascii_digit(){
          l -= 1;
          seen.insert((row, l));
        }
        while r + 1 < ref_row.len() && ref_row[r + 1].is_ascii_digit() {
          r += 1;
          seen.insert((row, r));
        }

        let num_str: String = ref_row[l..=r].iter().collect();
        let num = num_str.parse::<u32>().unwrap();
        seen_nums.insert(num);
      }
    }
    if seen_nums.len() == 2 {
      let mut seen_iter = seen_nums.into_iter();
      let num1 = seen_iter.next().unwrap();
      let num2 = seen_iter.next().unwrap();
      res += num1 * num2;
    }
  }
  println!("{}", res);
}
