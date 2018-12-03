extern crate regex;

use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

fn get_parsed_match(caps: &regex::Captures, i: usize) -> i32 {
  caps.get(i).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap()
}

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  let mut fabric: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();

  let re = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
  for line in reader.lines() {
    let input = line.unwrap();
    let caps = re.captures(&input).unwrap();
    let id = get_parsed_match(&caps, 1);
    let (corner_x, corner_y) = (get_parsed_match(&caps, 2), get_parsed_match(&caps, 3));
    let (w, h) = (get_parsed_match(&caps, 4), get_parsed_match(&caps, 5));
    for i in corner_x..(corner_x + w) {
      for j in corner_y..(corner_y + h) {
        let s = fabric.entry((i, j)).or_insert(HashSet::new());
        s.insert(id);
      }
    }
  }
  let overlaps_count = fabric.values().filter(|s| s.len() > 1).count();
  println!("{:?}", overlaps_count);
}
