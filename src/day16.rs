use std::borrow::BorrowMut;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use memoize::memoize;

use crate::day08::Point;
use crate::day09::distance;
use crate::utils::read_chunks;
use std::cell::RefCell;
use std::cmp::Ordering::{Equal, Greater, Less};


thread_local!(static VALVES: RefCell<Vec<Valve>> = {
    let mut cache = Vec::new();
    RefCell::new(cache)
});
thread_local!(static VALVE_LOOKUP: RefCell<HashMap<String, Valve>> = {
    let mut cache = HashMap::new();
    RefCell::new(cache)
});

pub fn part_one() -> u64 {
  let lines = read_chunks("day16.txt", "\n");
  setup_globals(&lines);

  solve_one()
}

fn setup_globals(lines: &Vec<String>) {
  let valves = parse_input(&lines);
  valves
    .iter()
    .for_each(|v| {
      VALVES.with(|cache| cache.borrow_mut().push(v.clone()));
      VALVE_LOOKUP.with(|cache| cache.borrow_mut().insert(v.name.clone(), v.clone()));
    })
}

#[allow(dead_code)]
pub fn part_two() -> u64 {
  let lines = read_chunks("day16.txt", "\n");
  solve_two(&lines, 4_000_000) as u64
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Valve {
  name: String,
  flow_rate: i64,
  neighbors: Vec<String>,
}

fn parse_input(lines: &Vec<String>) -> Vec<Valve> {
  lines
    .iter()
    .map(|s| s.replace("=", " "))
    .map(|s| s.replace(";", " "))
    .map(|s| s.replace(",", " "))
    .map(|s| s.split(" ").map(|s| s.to_string()).collect())
    .map(|s| split_to_valve_with_only_names(s))
    .collect()
}

fn split_to_valve_with_only_names(parts: Vec<String>) -> Valve {
  let name = parts.get(1).unwrap().to_string();
  let flow_rate = parts.get(5).unwrap().parse().unwrap();
  let neighbors = parts[11..parts.len()]
    .iter()
    .map(|s| (*s).to_owned())
    .filter(|s| !s.is_empty())
    .collect();

  Valve { name, flow_rate, neighbors }
}


fn solve_one() -> u64 {
  // let tmp_start = Valve { name: "--".to_string(), flow_rate: 0, neighbors: vec!["AA".to_string()] };
  let highest_score = solve_from(
    "AA".to_string(),
    Vec::new(),
    Vec::new(),
    0,
    30,
  );

  let x = 0;

  highest_score.1 as u64
}

#[derive(Eq, PartialEq, Hash)]
struct MemoKey {
  start: String,
  opened: String,
  score: i64,
  mins_left: i64,
}

impl MemoKey {
  fn make_opened(opened: &HashSet<String>) -> String {
    let mut o = Vec::from_iter(opened.clone());
    o.sort();
    o.join("")
  }
}

#[memoize]
fn test(a: String) -> i64 {
  0
}

#[memoize]
fn solve_from(
  start_str: String,
  opened: Vec<String>,
  path: Vec<String>,
  score: i64,
  minutes_left: i64,
) -> (Vec<String>, i64) {
  let non_zero_valves = VALVES.with(|c| c.borrow().iter().filter(|v| v.flow_rate != 0).map(|v| v.name.clone()).collect::<Vec<String>>());
  let non_zero_valves_set: HashSet<String> = HashSet::from_iter(non_zero_valves);
  let opened_valves_set: HashSet<String> = HashSet::from_iter(opened.clone());

  if minutes_left <= 0 || non_zero_valves_set == opened_valves_set  {
    // if score >= 1650 {
    // println!("Score: {}, Path: {:?}", score, opened);
    // }
    return (path, score);
  }

  let mut best_score = 0;
  let mut best_path = Vec::new();

  // let x = opened.borrow_mut().get(0).unwrap();

  let start = VALVE_LOOKUP.with(|c| c.borrow().get(&start_str).cloned()).unwrap();
  let mut neighbors = start.neighbors;
  neighbors.sort_by(|v1, v2| {
    let v1v =VALVE_LOOKUP.with(|c| c.borrow().get(v1).cloned()).unwrap();
    let v2v =VALVE_LOOKUP.with(|c| c.borrow().get(v2).cloned()).unwrap();
    if v1v.flow_rate == 0 && v2v.flow_rate > 0 {
      Greater
    }  else if v1v.flow_rate > 0 && v2v.flow_rate == 0 {
      Less
    } else if v1v.flow_rate == 0 && v2v.flow_rate == 0 {
      Equal
    } else if opened_valves_set.contains(v1) && !opened_valves_set.contains(v2) {
      Greater
    } else if !opened_valves_set.contains(v1) && opened_valves_set.contains(v2) {
      Less
    } else {
      Equal
    }
  });

  for n in neighbors {

    // if start_str == "AA".to_string() && n == "DD".to_string() && minutes_left == 24 {
    if start_str == "DD" && opened == vec!["DD", "BB", "JJ"] {
      let sx = 0;
    }

    let neighbor_valve = VALVE_LOOKUP.with(|c| c.borrow().get(&n).cloned()).unwrap();
    // If it's never been opened, open it
    if !opened.contains(&n) && neighbor_valve.flow_rate > 0 {
      let mut updated_opened = opened.clone();
      updated_opened.push(n.clone());
      updated_opened.sort();
      let additional_score = ((minutes_left - 2) * neighbor_valve.flow_rate);


      // {
      //   if n == "DD".to_string() && additional_score == 28 * 20 {
      //     println!("Opened DD for 560");
      //   }
      //   if n == "BB".to_string() && additional_score == 25 * 13 {
      //     println!("Opened BB for 325");
      //   }
      //   if n == "JJ".to_string() && additional_score == 21 * 21 {
      //     println!("Opened JJ for 441");
      //   }
      //   if n == "HH".to_string() && additional_score == 13 * 22 {
      //     println!("Opened HH for 286");
      //   }
      //   if n == "EE".to_string() && additional_score == 9 * 3 {
      //     println!("Opened EE for 27");
      //   }
      //   if n == "CC".to_string() && additional_score == 6 * 2 {
      //     println!("Opened CC for 12");
      //   }
      // }

      let next_score = score + additional_score;
      let mut next_path = path.clone();
      // next_path.push(n.clone());
      let remaining_minutes = minutes_left - 2;
      let (p, s) = solve_from(
        n.clone(),
        updated_opened,
        next_path,
        next_score,
        remaining_minutes,
      );
      if s > best_score {
        best_score = s;
        best_path = p;
      }
    }
      // And also pass through without changing it to open
      let remaining_minutes = minutes_left - 1;
      let mut next_path = path.clone();
      // next_path.push(n.clone());
      let (p,s) = solve_from(
        n.clone(),
        opened.clone(),
        next_path,
        score,
        remaining_minutes,
      );
      if s > best_score {
        best_score = s;
        best_path = p;
      }

  }

  (best_path, best_score)
}

fn solve_two(lines: &Vec<String>, max_range: i64) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;
  use std::collections::HashMap;
  use crate::day16::{parse_input, setup_globals, solve_one};

  use crate::utils::read_chunks;

  #[test]
  fn test_part_1() {
    let input = get_input();
    setup_globals(&input);
    assert_eq!(solve_one(), 1651);
  }
  #[test]
  fn test_part_doctored_simple_input() {
    let input = get_doctored_simplest_input();
    setup_globals(&input);
    assert_eq!(solve_one(), 416);
  }
  #[test]
  fn test_made_up_simple_input() {
    let input = get_made_up_input();
    setup_globals(&input);
    assert_eq!(solve_one(), 439);
  }

  #[test]
  fn test_part_2() {}

  fn get_input() -> Vec<String> {
    vec![
      "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_owned(),
      "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_owned(),
      "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_owned(),
      "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_owned(),
      "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_owned(),
      "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_owned(),
      "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_owned(),
      "Valve HH has flow rate=22; tunnel leads to valve GG".to_owned(),
      "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_owned(),
      "Valve JJ has flow rate=21; tunnel leads to valve II".to_owned(),
    ]
  }

  fn get_doctored_simplest_input() -> Vec<String> {
    vec![
      "Valve AA has flow rate=0; tunnels lead to valves BB".to_owned(),
      "Valve BB has flow rate=13; tunnels lead to valves AA, CC".to_owned(),
      "Valve CC has flow rate=2; tunnels lead to valves BB".to_owned(),
    ]
  }

  fn get_made_up_input() -> Vec<String> {
    vec![
      "Valve AA has flow rate=0; tunnels lead to valves BB, DD".to_owned(),
      "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_owned(),
      "Valve CC has flow rate=2; tunnels lead to valves AA, BB".to_owned(),
      "Valve DD has flow rate=1; tunnels lead to valves AA".to_owned(),
    ]
  }
}
