use std::io;
use std::io::BufRead;
use std::collections:: { HashMap, HashSet };

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  
  // node -> children
  let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
  let mut deps: HashMap<char, HashSet<char>> = HashMap::new();

  let mut children: HashSet<char> = HashSet::new();
  let mut parents: HashSet<char> = HashSet::new();

  for line in reader.lines().filter_map(|line| line.ok()) {
    let chars: Vec<_> = line.chars().collect();
    let node = chars[5];
    let child_node = chars[36];

    let mut set_c = graph.entry(node).or_insert(HashSet::new());
    set_c.insert(child_node);
    
    let mut set_p = deps.entry(child_node).or_insert(HashSet::new());
    set_p.insert(node);

    parents.insert(node);
    children.insert(child_node);
  }

  println!("{:#?}", graph);

  let roots: Vec<_> = parents.difference(&children).collect();
  for root in roots {
    deps.insert(*root, HashSet::new());
  } 
  
  let mut res: Vec<char> = Vec::new();

  while deps.len() > 0 {
    println!("{:?}", deps);
    let current: char;
    
    {
      let mut available: Vec<_> = deps.iter().by_ref().filter(|(_key, val)| val.len() == 0).map(|e| e.0).collect();
      available.sort_unstable_by(|a, b| a.cmp(b));
      current = *available.first().expect("dependency cycle").clone();
    }
    
    res.push(current);
    
    if let Some(children) = graph.get(&current) { 
      for node in children {
        let mut node_deps = deps.get_mut(node).unwrap();
        node_deps.remove(&current);
      }
    }
    deps.remove(&current);
  }
  println!("{}", res.iter().collect::<String>());
}
