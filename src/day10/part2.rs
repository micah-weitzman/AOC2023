
use crate::utils::read_lines;

#[derive(Debug)]
enum Dir {
  Up,
  Down,
  Left,
  Right,
}

pub fn main(filename: &str) {
  use Dir::*;
  let mut map: Vec<Vec<char>> = read_lines(filename).iter().map(|s| s.chars().collect()).collect();
  let mut is_loop: Vec<Vec<bool>> = map.iter().map(|v|vec![false; v.len()]).collect();

  let (mut s_x, mut s_y): (usize, usize) = (0, 0);

  for (y, row) in map.iter().enumerate() {
    if let Some(res) = row.iter().position(|&s| s == 'S') {
      s_x = res;
      s_y = y;
      break;
    }
  }

  let mut start_adj: Vec<Dir> = vec![];
  let mut next_moves: Vec<(Dir, Dir, usize, usize)> = vec![];

  if s_y + 1 < map.len() {
    let nxt = match map.get(s_y + 1).unwrap().get(s_x).unwrap() {
      'J' => Some((Down, Left, s_x, s_y + 1)),
      '|' => Some((Down, Down, s_x, s_y + 1)),
      'L' => Some((Down, Right, s_x, s_y + 1)),
      _ => None
    };
    if let Some(d) = nxt {
      next_moves.push(d);
      start_adj.push(Down)
    }
  }
  if s_y > 0 {
    let nxt = match map.get(s_y - 1).unwrap().get(s_x).unwrap() {
      '7' => Some((Up, Left, s_x, s_y - 1)),
      '|' => Some((Up, Up, s_x, s_y - 1)),
      'F' => Some((Up, Right, s_x, s_y - 1)),
      _ => None
    };
    if let Some(d) = nxt {
      next_moves.push(d);
      start_adj.push(Up)
    }
  }
  if s_x > 0 {
    let nxt = match map.get(s_y).unwrap().get(s_x - 1).unwrap() {
      'L' => Some((Left, Up, s_x - 1, s_y)),
      '-' => Some((Left, Left, s_x - 1, s_y)),
      'F' => Some((Left, Down, s_x - 1, s_y)),
      _ => None
    };
    if let Some(d) = nxt {
      next_moves.push(d);
      start_adj.push(Left)
    }
  }
  if s_x + 1 < map[0].len() {
    let nxt = match map.get(s_y).unwrap().get(s_x + 1).unwrap() {
      'J' => Some((Right, Up, s_x + 1, s_y)),
      '-' => Some((Right, Right, s_x + 1, s_y)),
      '7' => Some((Right, Down, s_x + 1, s_y)),
      _ => None
    };
    if let Some(d) = nxt {
      next_moves.push(d);
      start_adj.push(Right)
    }
  }
  is_loop[s_y][s_x] = true;
  let start_char = {
    let q = start_adj.pop().unwrap();
    let p = start_adj.pop().unwrap();
    match (q,p) {
      (Left, Right) | (Right, Left) => '-',
      (Up, Down) | (Down, Up) => '|',
      (Up, Right) | (Right, Up) => 'L',
      (Left, Up) | (Up, Left) => 'J',
      (Down, Left) | (Left, Down) => '7',
      (Down, Right) | (Right, Down) => 'F',
      _ => panic!("bad start")
    }
  };


  // dbg!(&next_moves);

  let (mut r_start, mut r_next, mut r_x, mut r_y) = next_moves.pop().unwrap();
  // let (mut l_start, mut l_next, mut l_x, mut l_y) = next_moves.pop().unwrap();


  while map.get(r_y).unwrap().get(r_x).unwrap() != &'S' {

    (r_x, r_y, r_next) = match (&r_start, map[r_y][r_x]) {
      (Up, 'F') => (r_x + 1, r_y, Right),
      (Up, '|') => (r_x, r_y - 1, Up),
      (Up, '7') => (r_x - 1, r_y, Left),
      (Down, '|') => (r_x, r_y + 1, Down),
      (Down, 'L') => (r_x + 1, r_y, Right),
      (Down, 'J') => (r_x - 1, r_y, Left),
      (Left, 'L') => (r_x, r_y - 1, Up),
      (Left, '-') => (r_x - 1, r_y, Left),
      (Left, 'F') => (r_x, r_y + 1, Down),
      (Right, '-') => (r_x + 1, r_y, Right),
      (Right, '7') => (r_x, r_y + 1, Down),
      (Right, 'J') => (r_x, r_y - 1, Up),
      res => {
        dbg!(res);
        panic!("Invalid direction");
      }
    };
    is_loop[r_y][r_x] = true;
    r_start = r_next;
  }

  map[s_y][s_x] = start_char;

  let res = (1..map.len()-1).fold(0,
  |acc, y| {
    let row = map.get(y).unwrap();

    let mut count = 0;

    let mut it = row.iter().enumerate();

    let mut is_open = false;
    let mut is_bar = false;
    while let Some((x, &c)) = it.next() {
      if is_loop[y][x] {
        match c {
          '|' => {is_open = !is_open; continue; },
          '7' => { is_bar = false; continue;},
          'J' => { is_open = !is_open; is_bar = false; continue;},
          'F' => { is_bar = true; continue;}
          'L' => { is_open = !is_open; is_bar = true; continue;}
          '-' => if is_bar { continue; }
          _ => panic!("something wrong")
        }
      } else if !is_bar && is_open {
        count += 1;
      }
    }
    return acc + count;
  });

  println!("{}", res); // 305
}
