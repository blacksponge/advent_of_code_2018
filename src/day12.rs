use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::cmp::max;

//const NB_GEN: usize = 20; // part 1
const NB_GEN: usize = 50_000_000_000; // part 2

fn pad_dot(state: &mut Vec<char>) -> i128 {
  let before = state.iter().take_while(|c| **c == '.').count();
  for _ in 0..(3-before) {
    state.insert(0, '.');
  }

  let after = state.iter().rev().take_while(|c| **c == '.').count();
  state.append(&mut vec!['.'; max(3-after, 0)]);
  max(0, 3-before) as i128
}

fn main() {
  let mut input_line = String::new();
  let stdin = io::stdin();

  stdin.read_line(&mut input_line).expect("unable to read line");
  
  let mut lines = stdin.lock().lines();
  lines.next();

  let rules: HashMap<String, char> = lines.filter_map(|l| l.ok()).map(|l| {
    let splitted: Vec<_> = l.split(" => ").collect();
    (String::from(splitted[0]), splitted[1].chars().next().unwrap())
  }).collect();

  let mut state: Vec<char> = input_line.trim().split(": ").nth(1).unwrap().chars().collect();
  let mut offset = pad_dot(&mut state);
  let mut old_states: HashMap<String, (usize, i128)> = HashMap::new();
  let mut gen: usize = 0;
  old_states.insert(state.iter().collect::<String>(), (gen, offset));
  while gen < NB_GEN {
    state = state.windows(5).map(|w| *rules.get(&w.into_iter().collect::<String>()).unwrap_or(&'.')).collect::<Vec<_>>();
    offset += pad_dot(&mut state) - 2;

    let state_str: String = state.iter().collect();
    
    if old_states.contains_key(&state_str) {
      let (old_gen, old_offset) = old_states[&state_str];
      let cycle_len = gen - old_gen;
      let nb_cycles = (NB_GEN - gen) / cycle_len;
      let offset_diff = offset - old_offset;
      offset += offset_diff * (nb_cycles - 1) as i128;
      gen += nb_cycles * cycle_len;
    } else {
      old_states.insert(state_str, (gen, offset));
      gen += 1;
    }

  }
  
  let res: u128 = state.iter().enumerate().filter(|e| *e.1 == '#').inspect(|e| { println!("{:?}", e); }).map(|e| (e.0 as i128 - offset) as u128).sum();
  println!("{}", res);
}
