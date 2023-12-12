

use std::collections::HashMap;

use crate::utils::read_lines;

#[derive(Debug, Clone, Copy)]
struct Res {
  group: usize,
  amount: u32,
  perms: u32
}

impl Res {
  fn new(group: usize, amount: u32, perms: u32) -> Self {
    Self { group, amount, perms}
  }
}

pub fn main(filename: &str) {
  let lines: Vec<String> = read_lines(filename);
  let res: u32 = lines.into_iter().map(parse_line).sum();
  println!("{}", res); // 7916
}


fn parse_line(line: String) -> u32 {
  let l: Vec<&str> = line.split(' ').collect();

  let springs: String = l[0].to_string();
  let nums: Vec<u32> = l[1].to_owned()
                           .split(',')
                           .map(|n| n.parse::<u32>().unwrap())
                           .collect();

  if !springs.contains('?') {
    return 1;
  }


  let mut res_vec= vec![Res::new(0, 0, 1)];

  for i in 0..springs.len() {
    let c = springs.chars().nth(i).unwrap();

    if c == '?' {
      let mut new_res: Vec<Res> = vec![];
      for v in res_vec.iter_mut() {
        let mut v_clone = v.clone();
        v_clone.amount += 1;
        new_res.push(v_clone);

        if v.amount > 0 && v.amount == nums[v.group] {
          v.amount = 0;
          v.group += 1;
          new_res.push(*v);
        } else if v.amount == 0 {
          new_res.push(*v);
        }
      }
      res_vec = new_res;
    }
    if c == '#' {
      for v in res_vec.iter_mut() {
        v.amount += 1;
      }
    }
    if c == '.' {
      let mut new_res: Vec<Res> = vec![];
      for v in res_vec.iter_mut() {
        if v.amount > 0 && v.amount == nums[v.group] {
          v.amount = 0;
          v.group += 1;
          new_res.push(*v);
        } else if v.amount == 0 {
          new_res.push(*v);
        }
      }
      res_vec = new_res;
    }



    res_vec = res_vec.into_iter()
                     .filter(|res| res.amount <= *nums.get(res.group).unwrap_or(&0))
                     .fold(HashMap::<(usize, u32), Res>::new(), |mut map, val| {
                       if let Some(r) = map.get_mut(&(val.group, val.amount)) {
                         r.perms += val.perms;
                       } else {
                         map.insert((val.group, val.amount), val);
                       }
                       map
                     })
                     .into_values()
                     .collect::<Vec<Res>>();
  }

  return res_vec.iter_mut()
                .map(|v| {
                  if v.amount == *nums.get(v.group).unwrap_or(&0) {
                    v.group += 1;
                  }
                  v
                })
                .filter(|v| (v.group as usize >= nums.len()))
                .map(|v| v.perms)
                .sum();
}