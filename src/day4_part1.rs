extern crate chrono;

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

use chrono::{ NaiveDateTime, Timelike };

#[derive(Debug)]
enum Action {
  FallsAsleep,
  WakesUp,
  BeginShift(i32)
}

#[derive(Debug)]
struct TimedAction {
  action: Action,
  time: NaiveDateTime
}

fn main() {
  let stdin = io::stdin();
  let reader = stdin.lock();
  
  let mut guards: HashMap<i32, Vec<(NaiveDateTime, NaiveDateTime)>> = HashMap::new();
  let mut guards_vec: Vec<TimedAction> = Vec::new();

  for line in reader.lines() {
    let mut input = String::from(line.expect("failed to read line"));

    let mut action = input.split_off(17);
    action = action.split_off(2);
    let splited_action: Vec<_> = action.split_whitespace().collect();
    
    let dt = NaiveDateTime::parse_from_str(&input, "[%Y-%m-%d %H:%M").unwrap();

    let action = match splited_action[0] {
      "Guard" => {
        let id_str = &splited_action[1][1..splited_action[1].len()];
        let id = id_str.parse::<i32>().unwrap();
        Some(Action::BeginShift(id))
      },
      "falls" => Some(Action::FallsAsleep),
      "wakes" => Some(Action::WakesUp),
      _ => None
    };

    let t_act = TimedAction {
      action: action.expect("unknown action"),
      time: dt
    };

    guards_vec.push(t_act);
  }
  
  guards_vec.sort_unstable_by_key(|e| e.time);

  let mut guards_iter = guards_vec.iter().peekable();
  
  let mut start_time: NaiveDateTime = NaiveDateTime::from_timestamp(0, 0);
  let mut naps: Vec<(NaiveDateTime, NaiveDateTime)> = Vec::new();
  let mut guard_id: i32 = 0;

  loop {
    let act = match guards_iter.next() {
      Some(x) => x,
      None => break
    };

    match act.action {
      Action::BeginShift(id) => {
        guard_id = id;
      },
      Action::FallsAsleep => { start_time = act.time; },
      Action::WakesUp => { naps.push((start_time, act.time)) }
    };

    match guards_iter.peek() {
      Some(TimedAction { action: Action::BeginShift(_), time: _}) | None => {
        guards.entry(guard_id).and_modify(|v| v.append(&mut naps)).or_insert(naps);
        naps = Vec::new();
      },
      _ => ()
    };
  }
  
  let best = guards
    .iter()
    .map(|(id, naps)| 
      (id,
      naps
        .iter()
        .fold(0, |acc, (start, end)| acc+(*end-*start).num_minutes())
      ))
    .max_by_key(|c| c.1).unwrap();

  let best_guard_id = best.0;
  
  let minutes = guards.get(best_guard_id).unwrap().iter().flat_map(|(start, end)| {
    let s: Vec<_> = (start.minute()..end.minute()).collect();
    s
  });
  let mut min_freq: HashMap<u32, u32> = HashMap::new();

  for m in minutes {
    min_freq.entry(m).and_modify(|e| *e += 1).or_insert(1);
  }

  let best_minute = min_freq.iter().max_by_key(|e| e.1).unwrap().0;
  println!("{}", best_minute * (*best_guard_id as u32));
}
