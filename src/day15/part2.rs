
use crate::utils::read_lines;

#[derive(Debug, Clone)]
struct Slot {
  label: String,
  num: u32,
}

impl Slot {
  fn new(label: &str, num: u32) -> Self {
    Self { label: label.to_string(), num }
  }
}

pub fn main(filename: &str) {
  let instrs: Vec<String> = read_lines(filename).get(0)
                                                .unwrap()
                                                .split(',')
                                                .map(|s| s.to_string())
                                                .collect();

  let mut boxes: Vec<Vec<Slot>> = vec![Vec::<Slot>::new(); 256];

  for inst in instrs {
    if inst.contains('=') {
      let mut it = inst.split('=');
      let label = it.next().unwrap().to_string();
      let num = it.next().unwrap().parse::<u32>().unwrap();

      let _box = boxes.get_mut(hash_val(&label)).unwrap();

      let mut found = false;

      for slot in _box.iter_mut() {
        if slot.label == label {
          slot.num = num;
          found = true;
          break;
        }
      }
      if !found {
        _box.push(Slot::new(&label, num));
      }
    } else {
      let label = inst.split('-').next().unwrap().to_string();

      let _box = boxes.get_mut(hash_val(&label)).unwrap();

      for (i, slot) in _box.iter().enumerate() {
        if slot.label == label {
          _box.remove(i);
          break;
        }
      }
    }
  }


  let res: u32 = boxes.into_iter()
                      .enumerate()
                      .map(|(i, v)|
                        v.into_iter()
                         .enumerate()
                         .map(|(j, slt)| slt.num * (i as u32 + 1) * (j as u32 + 1))
                         .sum::<u32>()
                      )
                      .sum();

  println!("{}", res); // 263211
}

fn hash_val(s: &String) -> usize {
  s.chars().fold( 0,|acc, c| ((acc + c as usize) * 17) % 256)
}