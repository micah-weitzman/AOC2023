
use crate::utils::read_lines;

pub fn main(filename: &str) {
  let lines = read_lines(filename);

  let mut it = lines.iter().peekable();

  let mut seeds_vec: Vec<i64> = vec![];

  let mut seeds_soil: Vec<(i64, i64, i64)> = vec![];
  let mut soil_fert: Vec<(i64, i64, i64)> = vec![];
  let mut fert_water: Vec<(i64, i64, i64)> = vec![];
  let mut water_light: Vec<(i64, i64, i64)> = vec![];
  let mut light_temp: Vec<(i64, i64, i64)> = vec![];
  let mut temp_hum: Vec<(i64, i64, i64)> = vec![];
  let mut hum_loc: Vec<(i64, i64, i64)> = vec![];


  while it.peek().is_some() {
    let line = it.next().unwrap();

    if line.starts_with("seeds: ") {
      seeds_vec = line.split(": ")
                      .last()
                      .unwrap()
                      .split(" ")
                      .map(|n| n.parse::<i64>().unwrap())
                      .collect();
    }
    if line.ends_with("map:") {
      let vec = match line.split(" ").next().unwrap() {
        "seed-to-soil" => &mut seeds_soil,
        "soil-to-fertilizer" => &mut soil_fert,
        "fertilizer-to-water" => &mut fert_water,
        "water-to-light" => &mut water_light,
        "light-to-temperature" => &mut light_temp,
        "temperature-to-humidity" => &mut temp_hum,
        "humidity-to-location" => &mut hum_loc,
        _ => panic!("map not found")
      };

      while it.peek().is_some() && !it.peek().unwrap().is_empty() {
        let nums: Vec<i64> = it.next()
                               .unwrap()
                               .split(" ")
                               .map(|n| n.parse::<i64>().unwrap())
                               .collect();
        let start = nums[1];
        let stop = start + nums[2] - 1;
        let inc = nums[0] - nums[1];
        vec.push((start, stop, inc));
      }
    }
  }

  let res: i64 = seeds_vec.into_iter()
  .map(|seed|
    [&seeds_soil,&soil_fert,&fert_water,&water_light,&light_temp,&temp_hum,&hum_loc]
      .iter()
      .fold(seed, |acc, x| {
        for (start, end, inc) in *x {
          if acc >= *start && acc <= *end {
            return acc + inc;
          }
        }
        acc
      })
  )
  .min()
  .unwrap();

  println!("{}", res);

}
