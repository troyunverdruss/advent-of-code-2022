use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use combinations::Combinations;

use crate::utils::read_chunks;

thread_local!(static VALVES: RefCell<Vec<Valve>> = {
    let cache = Vec::new();
    RefCell::new(cache)
});
thread_local!(static VALVE_LOOKUP: RefCell<HashMap<String, Valve>> = {
    let cache = HashMap::new();
    RefCell::new(cache)
});
thread_local!(static VALVES_WITH_NON_ZERO_FLOW_RATE: RefCell<HashSet<String>> = {
    let cache = HashSet::new();
    RefCell::new(cache)
});


pub fn part_one() -> u64 {
  let lines = read_chunks("day16.txt", "\n");
  setup_globals(&lines);

  solve_one_try_2()
}

fn setup_globals(lines: &Vec<String>) {
  let valves = parse_input(&lines);
  valves
    .iter()
    .for_each(|v| {
      VALVES.with(|cache| cache.borrow_mut().push(v.clone()));
      VALVE_LOOKUP.with(|cache| cache.borrow_mut().insert(v.name.clone(), v.clone()));
      if v.flow_rate > 0 {
        VALVES_WITH_NON_ZERO_FLOW_RATE.with(|cache| cache.borrow_mut().insert(v.name.clone()));
      }
    })
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day16.txt", "\n");
  setup_globals(&lines);

  solve_two_try_2()
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
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

fn solve_one_try_2() -> u64 {
  let valves = VALVES.with(|c| c.borrow().clone());
  let distance_lookup = compute_distances_between_non_zero_nodes(&valves);
  let remaining_non_zero_valves = VALVES_WITH_NON_ZERO_FLOW_RATE.with(|c| c.borrow().clone());
  let valve_lookup = VALVE_LOOKUP.with(|c| c.borrow().clone());

  let all_results = part_1_try_2_solver(
    &distance_lookup,
    &valve_lookup,
    &remaining_non_zero_valves,
    &HashSet::new(),
    0,
    30,
    "AA".to_string(),
  );

  all_results.iter().map(|r| (*r).1).max().unwrap()
}

fn hash_set_to_string(hs: &HashSet<String>) -> String {
  let mut v = Vec::from_iter(hs);
  v.sort();
  v.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("")
}

fn solve_two_try_2() -> u64 {
  let valves = VALVES.with(|c| c.borrow().clone());
  let distance_lookup = compute_distances_between_non_zero_nodes(&valves);
  let remaining_non_zero_valves = VALVES_WITH_NON_ZERO_FLOW_RATE.with(|c| c.borrow().clone());
  let valve_lookup = VALVE_LOOKUP.with(|c| c.borrow().clone());

  // TOO LOW 2580

  let all_results = part_1_try_2_solver(
    &distance_lookup,
    &valve_lookup,
    &remaining_non_zero_valves,
    &HashSet::new(),
    0,
    26,
    "AA".to_string(),
  );


  let mut string_to_hash_set: HashMap<String, HashSet<String>> = HashMap::new();


  let mut best_score_for_set: HashMap<String, u64> = HashMap::new();
  for r in all_results {
    let string_key = hash_set_to_string(&r.0);
    string_to_hash_set.insert(string_key.clone(), r.0.clone());

    match best_score_for_set.get(&string_key) {
      None => { best_score_for_set.insert(string_key.clone(), r.1); }
      Some(v) => {
        if v < &r.1 {
          best_score_for_set.insert(string_key.clone(), r.1);
        }
      }
    }
  }

  let mut best_score = 0;
  let all_keys = best_score_for_set.keys().map(|k| k.to_string()).collect::<Vec<String>>();
  for pair in Combinations::new(all_keys, 2) {
    let set_key_1 = pair.get(0).unwrap();
    let set_key_2 = pair.get(1).unwrap();
    let set_1 = string_to_hash_set.get(set_key_1).unwrap();
    let set_2 = string_to_hash_set.get(set_key_2).unwrap();

    let intersection = set_1.intersection(set_2);
    if intersection.count() == 0 {
      let score_1 = best_score_for_set.get(set_key_1).unwrap();
      let score_2 = best_score_for_set.get(set_key_2).unwrap();
      if score_1 + score_2 > best_score {
        best_score = score_1 + score_2;
      }
    }
  }

  best_score
}

#[derive(Eq, PartialEq, Hash)]
struct MemoKey {
  start: String,
  opened: String,
  score: i64,
  mins_left: i64,
}


fn compute_distances_between_non_zero_nodes(valves: &Vec<Valve>) -> HashMap<(String, String), u64> {
  let mut distances = HashMap::new();

  let important_valves: Vec<&Valve> = valves
    .iter()
    .filter(|v| v.name == "AA" || v.flow_rate > 0)
    .collect();

  for combo in Combinations::new(important_valves, 2) {
    let start = combo.get(0).unwrap();
    let end = combo.get(1).unwrap();
    let dist = shortest_distance_between_valves(start, end);

    distances.insert((start.name.clone(), end.name.clone()), dist);
    distances.insert((end.name.clone(), start.name.clone()), dist);
  }

  distances
}

fn shortest_distance_between_valves(v1: &Valve, v2: &Valve) -> u64 {
  let mut to_visit = VecDeque::new();
  let mut visited = HashSet::new();
  to_visit.push_back((v1.name.clone(), 0));

  let valve_lookup = VALVE_LOOKUP.with(|c| c.borrow().clone());

  while !to_visit.is_empty() {
    let (curr_node_name, curr_steps) = to_visit.pop_front().unwrap();
    let curr_node = valve_lookup.get(&curr_node_name).unwrap();

    visited.insert(curr_node_name.clone());
    for n in curr_node.neighbors.clone() {
      if n == v2.name {
        return curr_steps + 1;
      }
      to_visit.push_back((n.clone(), curr_steps + 1));
    }
  }

  panic!("No path could be found between {:?} and {:?}", v1, v2);
}

fn part_1_try_2_solver(
  distance_lookup: &HashMap<(String, String), u64>,
  valve_lookup: &HashMap<String, Valve>,
  remaining_valves: &HashSet<String>,
  opened: &HashSet<String>,
  score: u64,
  minutes_remaining: u64,
  location: String,
) -> Vec<(HashSet<String>, u64)> {
  if remaining_valves.is_empty() || minutes_remaining <= 0 {
    return vec![(opened.clone(), score)];
  }

  let mut all_results = Vec::new();
  all_results.push((opened.clone(), score));


  for next_valve_name in remaining_valves {
    let mut updated_remaining_valves = remaining_valves.clone();
    updated_remaining_valves.remove(next_valve_name);
    let mut updated_opened = opened.clone();
    updated_opened.insert(next_valve_name.clone());
    let distance = distance_lookup.get(&(location.clone(), next_valve_name.clone())).unwrap();
    if (distance + 1) < minutes_remaining {
      let next_valve = VALVE_LOOKUP.with(|c| c.borrow().get(next_valve_name).unwrap().clone());
      let result = part_1_try_2_solver(
        distance_lookup,
        valve_lookup,
        &updated_remaining_valves,
        &updated_opened,
        score + ((minutes_remaining - distance - 1) * next_valve.flow_rate as u64),
        minutes_remaining - distance - 1,
        next_valve_name.clone(),
      );

      all_results.extend(result);
    }
  }

  all_results
}

#[cfg(test)]
mod tests {
  use crate::day16::{setup_globals, solve_one_try_2, solve_two_try_2};

  #[test]
  fn test_part_1() {
    let input = get_input();
    setup_globals(&input);
    assert_eq!(solve_one_try_2(), 1651);
  }

  #[test]
  fn test_part_doctored_simple_input() {
    let input = get_doctored_simplest_input();
    setup_globals(&input);
    assert_eq!(solve_one_try_2(), 416);
  }

  #[test]
  fn test_part_doctored_simple_input_2() {
    let input = get_made_up_input();
    setup_globals(&input);
    assert_eq!(solve_two_try_2(), 416);
  }

  #[test]
  fn test_made_up_simple_input() {
    let input = get_made_up_input();
    setup_globals(&input);
    assert_eq!(solve_one_try_2(), 439);
  }

  #[test]
  fn test_part_2() {
    let input = get_input();
    setup_globals(&input);
    assert_eq!(solve_two_try_2(), 1707);
  }

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
