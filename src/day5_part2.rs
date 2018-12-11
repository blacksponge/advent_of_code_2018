use std::io;
use std::collections::HashSet;

fn step_react(input: String) -> String {
  let char_idx = input.char_indices().collect::<Vec<_>>();
  let to_remove = char_idx
    .windows(2)
    .find(|t| t[0].1 != t[1].1 && t[0].1.to_uppercase().to_string() == t[1].1.to_uppercase().to_string());
  if let Some(chosen_chars) = to_remove {
    //println!("{:?}", chosen_chars);
    let mut chars = input.chars().collect::<Vec<_>>();
    chars.remove(chosen_chars[1].0);
    chars.remove(chosen_chars[0].0);
    let res = chars.iter().collect::<String>();
    //println!("{:?}", res);
    res
  } else {
    input
  }
}

fn react(input: String) -> String {
  let mut length = input.len();
  let mut input_text = input;
  loop {
    input_text = step_react(input_text);
    if length == input_text.len() {
      break;
    }
    length = input_text.len(); 
  }
  input_text
}


fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();
  
  stdin.read_line(&mut input_text).expect("failed to read line");
  input_text = input_text.trim().to_string();
  let units = input_text.to_uppercase().chars().collect::<HashSet<_>>();
  println!("{:?}", units);
  println!("Finding new polymer...");
  let new_input = units
    .iter()
    .map(|unit| (unit, input_text.chars().filter(|c| c.to_uppercase().next().unwrap() != *unit).collect::<String>()))
    .inspect(|e| println!("Reacting polymer without unit {}", e.0))
    .map(|e| react(e.1).len())
    .inspect(|l| println!("Found length of {}", l))
    .min()
    .unwrap();
  println!("Final length: {}", new_input);
}
