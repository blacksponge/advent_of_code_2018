use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();

  stdin.read_line(&mut input_text).expect("failed to read line");
  let vect_input: Vec<_> = input_text.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect();

  let nb_players = vect_input[0];
  let nb_marbles = vect_input[1] + 1;
 
  let mut circle = Vec::<u32>::new();
  let mut scores = vec![0; nb_players as usize];

  circle.push(0);
  
  let mut players = (0..nb_players).cycle();
  let mut current_pos = 0;

  for current_marble in 1..nb_marbles {
    let current_player = players.next().unwrap();
    if current_marble % 23 == 0 {
      current_pos = (circle.len() + current_pos - 7) % circle.len();
      scores[current_player as usize] += current_marble + circle.remove(current_pos);
    } else {
      current_pos = (current_pos + 1) % circle.len() + 1;
      circle.insert(current_pos, current_marble);
    }
    println!("{:?}", circle);
  }
  println!("{}", scores.iter().max().unwrap());
}
