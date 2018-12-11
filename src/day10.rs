extern crate regex;

use std::io;
use std::io::BufRead;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
  vx: i32,
  vy: i32
}

fn calc_bbox(points: &Vec<Point>) -> ((i32, i32), (i32, i32)) {
  let mut it = points.iter();
  let first = it.next().unwrap();
  let bbox = it.fold(((first.x, first.y), (first.x, first.y)), |acc, e| {
    ( (min(e.x, (acc.0).0), min(e.y, (acc.0).1)), (max(e.x, (acc.1).0), max(e.y, (acc.1).1)) )
  });
  bbox
}

fn calc_area(bbox: &((i32, i32), (i32, i32))) -> u64 {
  ((bbox.1).0-(bbox.0).0) as u64 * ((bbox.1).1-(bbox.0).1) as u64
}

fn print_msg(points: &Vec<Point>) {
  let (min, max) = calc_bbox(points);
  let width = max.0-min.0+1;
  let height = max.1-min.1+1;
  let mut out = vec![vec![" "; width as usize]; height as usize];
  println!("w={}, h={}", width, height);
  for point in points {
    let x = (point.x-min.0) as usize;
    let y = (point.y-min.1) as usize;
    out[y][x] = "#";
  }
  for line in out {
    println!("{}", line.join(""));
  }
}

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  
  let re = regex::Regex::new(r"position=< ?(-?\d+), +(-?\d+)> velocity=< ?(-?\d+), +(-?\d+)>").unwrap();

  let mut points: Vec<_> = reader.lines()
    .filter_map(|line| line.ok())
    .map(|line| {
      let caps = re.captures(&line).unwrap();
      Point {
        x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        vx: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        vy: caps.get(4).unwrap().as_str().parse::<i32>().unwrap()
      }
    })
    .collect();
  
  let mut old = calc_area(&calc_bbox(&points));
  let mut new = old - 1;
  let mut sec = 0;
  
  while old > new {
    for point in points.iter_mut() {
      point.x += point.vx;
      point.y += point.vy;
    }
    old = new;
    new = calc_area(&calc_bbox(&points));
    sec += 1;
  }

  for point in points.iter_mut() {
    point.x -= point.vx;
    point.y -= point.vy;
  }
  
  sec -= 1;
  println!("nb seconds={}", sec);
  
  print_msg(&points);
}
