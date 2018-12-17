#[macro_use] extern crate itertools;
extern crate rayon;

use rayon::prelude::*;
use std::io;

fn calc_powerlevel(serial: i32, i: i32, j: i32) -> i32 {
  let rack_id = i+1+10;
  let mut power_level = rack_id * (j+1) + serial;
  power_level *= rack_id;
  power_level /= 100;
  power_level %= 10;
  power_level -= 5;
  power_level
}


fn calc_region_power(grid: &Vec<Vec<i32>>, coord: (usize, usize), size: usize) -> i32 {
  let mut tot = 0;
  for x in 0..size {
    for y in 0..size {
      tot += grid[x+coord.0][y+coord.1];
    }
  }
  tot
}

const GRID_SIZE:usize = 300;

fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();

  stdin.read_line(&mut input_text).expect("failed to read line");
  let serial = input_text.trim().parse::<i32>().unwrap();
  
  let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
  calc_powerlevel(serial, 2, 4);

  for i in 0..GRID_SIZE {
    for j in 0..GRID_SIZE {
      grid[i][j] = calc_powerlevel(serial, i as i32, j as i32);
    }
  }
  
  let mut global_max_coord = (0, 0);
  let mut global_max_power = calc_region_power(&grid, global_max_coord, GRID_SIZE);
  let mut max_size = 300;
  println!("{}", global_max_power);

  for size in (0..300).rev() {
    println!("SIZE {}", size);
    let (max_coord, max_power) = iproduct!(0..(300-size), 0..(300-size))
      .collect::<Vec<(usize, usize)>>()
      .par_iter()
      .map(|coord| (*coord, calc_region_power(&grid, *coord, size)))
      .max_by_key(|x| x.1).unwrap();
    
    if max_power > global_max_power {
      global_max_power = max_power;
      global_max_coord = max_coord;
      max_size = size;
      println!("MAX {:?} -> {}", global_max_coord, global_max_power);
    }
  }

  println!("coord=({},{}) power={} size={}", global_max_coord.0 + 1, global_max_coord.1 + 1, global_max_power, max_size);
}
