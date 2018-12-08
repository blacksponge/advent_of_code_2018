use std::io;

fn sum_metadata<I>(it: &mut I) -> usize
where
  I: Iterator<Item = usize>
{
  let nb_nodes = it.next().unwrap();
  let nb_metadata = it.next().unwrap();
  let mut res: usize = 0;
  for _ in 0..nb_nodes {
    res += sum_metadata(it);
  }
  res += it.take(nb_metadata).sum::<usize>();
  return res;
}

fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();

  stdin.read_line(&mut input_text).expect("failed to read line");

  let mut license_nbs = input_text.split_whitespace().filter_map(|n| n.parse::<usize>().ok());
  println!("{}", sum_metadata(&mut license_nbs));
}
