use std::io;

// useful only for debug
fn print_circle(circle: &Vec<Option<(usize, usize)>>) { 
    let mut idx = 0;
    for _ in circle.iter().filter_map(|x| *x) {
      let new_idx = circle[idx].unwrap().0;
      print!("{}, ", idx);
      idx = new_idx;
    }
    println!("");
}

fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();

  stdin.read_line(&mut input_text).expect("failed to read line");
  let vect_input: Vec<_> = input_text.split_whitespace().filter_map(|word| word.parse::<usize>().ok()).collect();

  let nb_players = vect_input[0];
  let nb_marbles = vect_input[1] * 100 + 1;
 
  let mut circle: Vec<Option<(usize, usize)>> = vec![None; nb_marbles];
  let mut scores = vec![0; nb_players];

  circle[0] = Some((0, 0));
  
  let mut players = (0..nb_players).cycle();
  
  // current_pos = current marble (and current_marble = new marble to be inserted)
  let mut current_pos = 0;

  for current_marble in 1..nb_marbles {
    
    let current_player = players.next().unwrap();
    
    if current_marble % 23 == 0 {
      for _ in 0..7 {
        current_pos = circle[current_pos].unwrap().1;
      }
      
      scores[current_player] += current_marble + current_pos;
      
      let (next_marble, prev_marble) = circle[current_pos].unwrap();
      
      circle[next_marble] = Some((circle[next_marble].unwrap().0, prev_marble));
      circle[prev_marble] = Some((next_marble, circle[prev_marble].unwrap().1));
      circle[current_pos] = None;
      
      current_pos = next_marble;
    } else {
      let prev = circle[current_pos].unwrap().0;
      let next = circle[prev].unwrap().0;
      
      circle[current_marble] = Some((next, prev));
      circle[prev] = Some((current_marble, current_pos));
      circle[next] = Some((circle[next].unwrap().0, current_marble));
      
      current_pos = current_marble;
    }
    //print_circle(&circle);
    //println!("{:?}", circle);
  }
  println!("{}", scores.iter().max().unwrap());
}
