use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum Block {
  Void,
  HorizontalTrack,
  VerticalTrack,
  CornerTLBR, // east => north ; west => south ; north => east ; south => west
  CornerBLTR, // east => south ; west => north ; north => west ; south => east
  Intersection,
}

#[derive(Debug)]
struct Cart {
  direction: (i32, i32),
  coord: (i32, i32),
  intersection_nb: usize
}


fn direction_at_intersection(cart: &Cart) -> (i32, i32) {
  match cart.intersection_nb % 3 {
    0 => if cart.direction.0 == 0 {
      (cart.direction.1, 0)
    } else {
      (0, -cart.direction.0)
    },
    1 => cart.direction,
    2 | _ => if cart.direction.0 == 0 {
      (-cart.direction.1, 0)
    } else {
      (0, cart.direction.0)
    } 
  }
}

fn display_map(map: &Vec<Vec<Block>>) {
  map.iter().map(|l| l.iter().map(|e| match e {
    Block::HorizontalTrack => '-',
    Block::VerticalTrack => '|', 
    Block::CornerTLBR => '/',
    Block::CornerBLTR => '\\',
    _ => ' '
  }).collect::<String>())
  .for_each(|l| println!("{}", l))
}
fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  let mut carts: Vec<Cart> = Vec::new();
  
  // map[y][x]
  let map: Vec<Vec<Block>> = reader.lines()
    .filter_map(|l| l.ok())
    .enumerate()
    .map(|(y, line)|
      line.trim_matches('\n')
        .char_indices()
        .inspect(|(x, c)|
          match c {
            '^' | '>' | 'v' | '<' => {
              carts.push(Cart {
                direction: match c {
                  '^' => (0, -1),
                  '>' => (1, 0),
                  'v' => (0, 1),
                  '<' | _ => (-1, 0)
                },
                coord: (*x as i32, y as i32),
                intersection_nb: 0
              });
            }
            _ => ()
        })
        .map(|(_x, c)| match c {
          '|' | '^' | 'v' => Block::VerticalTrack,
          '-' | '>' | '<' => Block::HorizontalTrack,
          '+' => Block::Intersection,
          '/' => Block::CornerTLBR,
          '\\' => Block::CornerBLTR,
          _ => Block::Void
        })
        .collect()
    ).collect();
  
  //display_map(&map); 
  
  'outer: loop {
    for cart in carts.iter_mut() {
      //println!("{:?}", cart);
      //println!("{:?}", cart);
      //println!("{:?}", map[cart.coord.1 as usize][cart.coord.0 as usize]);
      
      (*cart).coord.0 += cart.direction.0;
      (*cart).coord.1 += cart.direction.1;
      
      //println!("{:?}", cart);
      
      match map[cart.coord.1 as usize][cart.coord.0 as usize] {
        Block::CornerTLBR => (*cart).direction = (-cart.direction.1, -cart.direction.0),
        Block::CornerBLTR => (*cart).direction = (cart.direction.1, cart.direction.0),
        Block::Intersection => {
          (*cart).direction = direction_at_intersection(&cart);
          (*cart).intersection_nb += 1;
        }
        _ => ()
      }
      //println!("{:?}", cart);
    }
    for cart in &carts {
      if carts.iter().filter(|c| c.coord == cart.coord).count() > 1 {
        println!("{},{}", cart.coord.0, cart.coord.1);
        break 'outer;
      }
    }
    carts.sort_by_key(|cart| cart.coord);
  }
}
