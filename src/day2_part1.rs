use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  
  let mut nb_2 = 0;
  let mut nb_3 = 0;

  for line in reader.lines() {
    let input = line.unwrap();
    let mut letters: HashMap<char, i32> = HashMap::new();
    for letter in input.chars() {
      letters.entry(letter).and_modify(|i| *i += 1).or_insert(1);
    }
    let values: HashSet<_> = letters.values().collect();
    if values.contains(&2) {
      nb_2 += 1;
    }
    if values.contains(&3) {
      nb_3 += 1;
    }
  }

  println!("{}", nb_2 * nb_3);
}
