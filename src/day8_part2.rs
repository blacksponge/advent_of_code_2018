use std::io;

fn complex_metadata_calcul<I>(it: &mut I) -> usize
where
  I: Iterator<Item = usize>
{
  let nb_nodes = it.next().unwrap();
  let nb_metadata = it.next().unwrap();
  
  if nb_nodes == 0 {
    return it.take(nb_metadata).sum::<usize>();
  }
  
  let children_value: Vec<_> = (0..nb_nodes).map(|_| complex_metadata_calcul(it)).collect();
  
  it.take(nb_metadata).filter_map(|idx| children_value.get(idx - 1)).sum::<usize>()
}

fn main() {
  let stdin = io::stdin();
  let mut input_text = String::new();

  stdin.read_line(&mut input_text).expect("failed to read line");

  let mut license_nbs = input_text.split_whitespace().filter_map(|n| n.parse::<usize>().ok());
  println!("{}", complex_metadata_calcul(&mut license_nbs));
}
