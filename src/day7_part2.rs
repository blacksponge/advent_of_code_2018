use std::io;
use std::io::BufRead;
use std::collections:: { HashMap, HashSet };

const MAX_WORKERS: usize = 5;
const BASE_DURATION: usize = 60;

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

  let roots: Vec<_> = parents.difference(&children).collect();
  for root in roots {
    deps.insert(*root, HashSet::new());
  } 
  
  let mut res: usize = 0;
  let mut workers: HashMap<char, usize> = HashMap::new();

  while deps.len() > 0 {
    res += 1;
    // updating workers and removing from depencies lists if the task is done
    let mut new_workers: HashMap<char, usize> = HashMap::new();
    for (name, cooldown) in &workers {
      if *cooldown == 1 { 
        if let Some(children) = graph.get(name) {
          for node in children {
            let mut node_deps = deps.get_mut(node).unwrap();
            node_deps.remove(name);
          }
        }
      } else {
        new_workers.insert(*name, cooldown - 1);
      }
    }
    workers = new_workers;
    
    // fetching tasks to be done
    let mut available = deps.iter()
      .filter(|(_key, val)| val.len() == 0)
      .map(|e| e.0)
      .cloned()
      .collect::<Vec<_>>();
    available.sort_unstable_by(|a, b| a.cmp(b));
    
    let currents = available.iter()
      .take(MAX_WORKERS - workers.len())
      .cloned()
      .collect::<Vec<_>>();
    
    // assigning tasks to worker and removing from todo
    for current in &currents {
      workers.insert(*current, (*current as usize) - 64 + BASE_DURATION);
      deps.remove(current);
    }
  }
  res += workers.values().max().unwrap_or(&0) - 1;
  println!("{}", res);
}
