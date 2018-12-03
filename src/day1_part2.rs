use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  let numbers: Vec<_> = reader.lines().filter_map(|line| line.unwrap().parse::<i32>().ok()).collect();
  
  let mut cycle_numbers = numbers.iter().cycle();
  let mut seen: HashMap<i32, i32> = HashMap::new();
  let mut tot = 0;
  seen.insert(0, 1);
  println!("Searching...");
  loop {
    let n = cycle_numbers.next().unwrap();
    tot += n;

    seen.entry(tot).and_modify(|i| *i += 1).or_insert(1);

    let first_twice = seen.iter().find(|a| *a.1 == 2);
    
    if let Some(res) = first_twice {
      println!("{}", res.0);
      break;
    }
  }
}
