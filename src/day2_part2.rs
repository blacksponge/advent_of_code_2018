use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();

  let input: Vec<_> =  reader.lines().map(|line| line.unwrap().chars().enumerate().collect::<HashSet<_>>() ).collect();
  for id in input.iter() {
    let candidat = input.iter().find_map(|other_id| {
      let diff: HashSet<_> = id.difference(other_id).collect();
      if diff.len() == 1 {
        Some(other_id)
      } else {
        None
      }
    });
    if let Some(other_id) = candidat {
      let mut letters: Vec<_> = id.intersection(other_id).collect();
      letters.sort_by_key(|&(i, _v)| i);
      println!("{}", letters.iter().map(|&(_i, v)| v).collect::<String>());
      break;
    }
  }

}
