#[macro_use] extern crate itertools;
extern crate rayon;

use rayon::prelude::*;
use std::io;
use std::collections::HashMap;

fn calc_powerlevel(serial: i32, i: i32, j: i32) -> i32 {
  let rack_id = i+1+10;
  let mut power_level = rack_id * (j+1) + serial;
  power_level *= rack_id;
  power_level /= 100;
  power_level %= 10;
  power_level -= 5;
  power_level
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

  let mut mem: HashMap<(usize, usize, usize), i32> = iproduct!(0..GRID_SIZE, 0..GRID_SIZE).map(|e| ((e.0, e.1, 1_usize), grid[e.0][e.1])).collect();

  for size in 2..(GRID_SIZE+1) {
    println!("SIZE {}", size);
    
    for i in 0..(GRID_SIZE-size+1) {
      for j in 0..(GRID_SIZE-size+1) { 
        let power_init = mem[&(i, j, size-1)];
        let mut power = power_init;
        power += grid[i+size-1][j..(j+size)].into_iter().sum::<i32>();
        power += grid[i..(i+size-1)].into_iter().map(|l| l[j+size-1]).sum::<i32>();
        mem.insert((i, j, size), power);
      }
    }
  }

  let max = mem.par_iter().max_by_key(|e| e.1).unwrap();
  println!("{:?}", max);
}
