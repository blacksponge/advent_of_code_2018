use std::io;

fn remove_unit(input: String) -> String {
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


fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();
  
  stdin.read_line(&mut input_text).expect("failed to read line");
  input_text = input_text.trim().to_string();
  
  let mut length = input_text.len();
  loop {
    input_text = remove_unit(input_text);
    if length == input_text.len() {
      break;
    }
    length = input_text.len(); 
  }
  println!("{}", length);
}
