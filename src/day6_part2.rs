use std::io;
use std::io::BufRead;
use std::cmp::{ min, max };
use std::collections::HashSet;

const MAX_WEIGHT: i32 = 10_000;


fn pass_weight(points: &Vec<(i32, i32)>, my_point: (i32, i32)) -> bool {
  return points.iter().fold(0, |acc, c| acc + (c.0 - my_point.0).abs() + (c.1 - my_point.1).abs()) < MAX_WEIGHT;
}

fn main() {
  let stdin  = io::stdin();
  let reader  = stdin.lock();
  let coord: Vec<_> = reader.lines()
    .filter_map(|line| line.ok())
    .map(|line| {
      let v = line.split(", ")
        .filter_map(|e| e.parse::<usize>().ok())
        .collect::<Vec<_>>();
      (v[0] as i32, v[1] as i32)
    })
    .collect();
  
  let bbox = coord.iter().skip(1).fold((coord[0], coord[0]), |bbox, point| {
    ((min(point.0, (bbox.0).0), min(point.1, (bbox.0).1)), (max(point.0, (bbox.1).0), max(point.1, (bbox.1).1)))
  });

  let (width, height) = (((bbox.1).0 - (bbox.0).0 + 1), ((bbox.1).1 - (bbox.0).1 + 1));
  
  let mut zone: HashSet<(i32, i32)> = HashSet::new();
  let mut stack: Vec<(i32, i32)> = Vec::new();

  'outer: for x in 0..width {
    for y in 0..height {
      if pass_weight(&coord, (x, y)) {
        stack.push((x, y));
        break 'outer;
      }
    }
  }

  println!("{:?}", stack);

  while let Some(cur) = stack.pop() {
    if zone.contains(&cur) {
      continue;
    }

    let weight = coord.iter().fold(0, |acc, c| acc + (c.0 - cur.0).abs() + (c.1 - cur.1).abs());
    if weight < MAX_WEIGHT {
      zone.insert(cur);
      
      stack.push((cur.0 + 1, cur.1));
      stack.push((cur.0, cur.1 + 1));
      stack.push((cur.0 - 1, cur.1));
      stack.push((cur.0, cur.1 - 1));
    }
  }
  
  println!("{}", zone.len());
}
