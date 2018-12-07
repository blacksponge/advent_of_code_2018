use std::io;
use std::io::BufRead;
use std::cmp::{ min, max };
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Cell {
  Value(char),
  Point(char),
  Several,
  Empty
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
      (v[0], v[1])
    })
    .collect();
  
  let bbox = coord.iter().skip(1).fold((coord[0], coord[0]), |bbox, point| {
    ((min(point.0, (bbox.0).0), min(point.1, (bbox.0).1)), (max(point.0, (bbox.1).0), max(point.1, (bbox.1).1)))
  });
  
  let (width, height) = (((bbox.1).0 - (bbox.0).0 + 1), ((bbox.1).1 - (bbox.0).1 + 1));
  let mut grid = vec![vec![Cell::Empty; height]; width];
  let (x0, y0) = bbox.0;
  
  //println!("{:?}", bbox); 
  
  let named_coord:Vec<_> = coord.iter()
    //.inspect(|e| println!("#########\n{:?}", e))
    .enumerate()
    .map(|(i, val)| ((val.0-x0, val.1-y0), (97 + (i as u8)) as char))
    //.inspect(|e| println!("{:?}", e.0))
    .collect();
  
  //println!("{:?}", named_coord);
  //return;
  
  for ((x, y), name) in named_coord.iter() {
    grid[*x][*y] = Cell::Point(*name);
  }

  let mut edgy_names: HashSet<char> = HashSet::new();

  for x in 0..width {
    for y in 0..height {
      if let Cell::Empty = grid[x][y] {
        let v = named_coord.iter()
          .map(|(coord, name)| ((coord.0 as i32 - x as i32).abs() + (coord.1 as i32 - y as i32).abs(), name));
        let m = v.clone().min_by_key(|e| e.0).unwrap();
        let nb_m = v.filter(|e| e.0 == m.0).count();
        
        grid[x][y] = if nb_m == 1 {
          if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
            edgy_names.insert(*m.1);
          }
          Cell::Value(*m.1)
        } else {
          Cell::Several
        };
      }
    }
  }

  let names: HashSet<_> = named_coord.iter().map(|(_coord, name)| *name).collect();
  let inner_names = names.difference(&edgy_names);
  let flat_grid = grid.iter().flatten().collect::<Vec<_>>();
  
  let zone_size = inner_names.map(|name|
    flat_grid.iter()
      .filter(|e| match e {
        Cell::Point(e) if name == e => true,
        Cell::Value(e) if name == e => true,
        _ => false
      })
      .count()
    )
    .max().unwrap();
  println!("{}", zone_size);

/*
  for y in 0..height {
    for x in 0..width {
      match grid[x][y] {
        Cell::Point(name) => print!("{}", name.to_uppercase().next().unwrap()),
        Cell::Value(name) => print!("{}", name),
        Cell::Several => print!("."),
        Cell::Empty => print!(" ")
      };
    }
    println!("");
  }
*/

}
