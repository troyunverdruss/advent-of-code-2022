use std::borrow::BorrowMut;
use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use memoize::memoize;
use combinations::Combinations;
use permutations::Permutations;

use crate::day08::Point;
use crate::day09::distance;
use crate::utils::read_chunks;
use std::cell::RefCell;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::SeekFrom::Start;


thread_local!(static VALVES: RefCell<Vec<Valve>> = {
    let mut cache = Vec::new();
    RefCell::new(cache)
});
thread_local!(static VALVE_LOOKUP: RefCell<HashMap<String, Valve>> = {
    let mut cache = HashMap::new();
    RefCell::new(cache)
});
thread_local!(static VALVES_WITH_NON_ZERO_FLOW_RATE: RefCell<HashSet<String>> = {
    let mut cache = HashSet::new();
    RefCell::new(cache)
});
thread_local!(static MEMO_2: RefCell<HashMap<String, i64>> = {
    let mut cache = HashMap::new();
    RefCell::new(cache)
});
thread_local!(static MEMO_TRY_2: RefCell<HashMap<String, i64>> = {
    let mut cache = HashMap::new();
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

#[allow(dead_code)]
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


fn solve_one() -> u64 {
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
    "AA".to_string()
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
    "AA".to_string()
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

impl MemoKey {
  fn make_opened(opened: &HashSet<String>) -> String {
    let mut o = Vec::from_iter(opened.clone());
    o.sort();
    o.join("")
  }
}

fn sort_valves(opened_valves_set: &HashSet<String>, v1: &Valve, v2: &Valve) -> Ordering {
  if v1.flow_rate == 0 && v2.flow_rate > 0 {
    Greater
  } else if v1.flow_rate > 0 && v2.flow_rate == 0 {
    Less
  } else if v1.flow_rate == 0 && v2.flow_rate == 0 {
    Equal
  } else if opened_valves_set.contains(&v1.name) && !opened_valves_set.contains(&v2.name) {
    Greater
  } else if !opened_valves_set.contains(&v1.name) && opened_valves_set.contains(&v2.name) {
    Less
  } else {
    Equal
  }
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

  if minutes_left <= 0 || non_zero_valves_set == opened_valves_set {
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
    let v1v = VALVE_LOOKUP.with(|c| c.borrow().get(v1).cloned()).unwrap();
    let v2v = VALVE_LOOKUP.with(|c| c.borrow().get(v2).cloned()).unwrap();
    sort_valves(&opened_valves_set, &v1v, &v2v)
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
    let (p, s) = solve_from(
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

#[derive(Clone, Eq, PartialEq, Hash)]
struct StartLocation {
  name: String,
  just_opened: bool,
}

impl StartLocation {
  fn to_string(self: &Self) -> String {
    format!("{} {}", self.name, self.just_opened)
  }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct StartData {
  start1: StartLocation,
  start2: StartLocation,
}

impl StartData {
  fn sorted_to_string(self: &Self) -> String {
    let mut v = vec![&self.start1, &self.start2];
    v.sort_by_key(|s| &s.name);
    v.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("")
  }
}


fn make_memo_key(starts: &StartData, opened: &HashSet<String>, score: i64, minutes_left: i64) -> String {
  let mut sorted_opened = Vec::from_iter(opened.clone());
  sorted_opened.sort();
  format!(
    "{}, {:?}, {}, {}",
    starts.sorted_to_string(),
    sorted_opened.join(""),
    score,
    minutes_left,
  )
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

fn part_2_try_2(valves: &Vec<Valve>) {
  let distance_lookup = compute_distances_between_non_zero_nodes(&valves);
  let remaining_non_zero_valves = VALVES_WITH_NON_ZERO_FLOW_RATE.with(|c| c.borrow().clone());
  let valve_lookup = VALVE_LOOKUP.with(|c| c.borrow().clone());

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

fn part_2_try_2_memo_key(
  remaining_valves: &HashSet<String>,
  score: u64,
  minutes_remaining_1: u64,
  minutes_remaining_2: u64,
  location_1: &String,
  location_2: &String,
) -> String {
  let mut sorted_remaining = Vec::from_iter(remaining_valves);
  sorted_remaining.sort();

  format!(
    "{:?}, {}, {}, {}, {}, {}",
    sorted_remaining,
    score,
    minutes_remaining_1,
    minutes_remaining_2,
    location_1,
    location_2
  )
}

fn part_2_try_2_solver(
  distance_lookup: &HashMap<(String, String), u64>,
  valve_lookup: &HashMap<String, Valve>,
  remaining_valves: &HashSet<String>,
  score: u64,
  minutes_remaining_1: u64,
  minutes_remaining_2: u64,
  location_1: String,
  location_2: String,
) -> u64 {
  // let memo_key = part_2_try_2_memo_key(
  //   remaining_valves,
  //   score,
  //   minutes_remaining_1,
  //   minutes_remaining_2,
  //   &location_1,
  //   &location_2
  // );

  // let memo_result = MEMO_TRY_2.with(|c| c.borrow().get(&memo_key).cloned());
  // match memo_result {
  //   None => {}
  //   Some(v) => { return v as u64; }
  // }

  if remaining_valves.is_empty() || (minutes_remaining_1 <= 0 && minutes_remaining_2 <= 0) {
    // MEMO_TRY_2.with(|c| c.borrow_mut().insert(memo_key.clone(), score as i64));
    return score;
  }

  let mut best_score = score;
  if minutes_remaining_1 >= minutes_remaining_2 {
    for next_valve_name in remaining_valves {
      let mut updated_remaining_valves = remaining_valves.clone();
      updated_remaining_valves.remove(next_valve_name);
      let distance = distance_lookup.get(&(location_1.clone(), next_valve_name.clone())).unwrap();
      if (distance + 1) < minutes_remaining_1 {
        let next_valve = VALVE_LOOKUP.with(|c| c.borrow().get(next_valve_name).unwrap().clone());
        let score = part_2_try_2_solver(
          distance_lookup,
          valve_lookup,
          &updated_remaining_valves,
          score + ((minutes_remaining_1 - distance - 1) * next_valve.flow_rate as u64),
          minutes_remaining_1 - distance - 1,
          minutes_remaining_2,
          next_valve_name.clone(),
          location_2.clone()
        );
        if score > best_score {
          best_score = score;
        }
      }
    }
  } else {
    for next_valve_name in remaining_valves {
      let mut updated_remaining_valves = remaining_valves.clone();
      updated_remaining_valves.remove(next_valve_name);
      let distance = distance_lookup.get(&(location_2.clone(), next_valve_name.clone())).unwrap();
      if (distance + 1) < minutes_remaining_2 {
        let next_valve = VALVE_LOOKUP.with(|c| c.borrow().get(next_valve_name).unwrap().clone());
        let score = part_2_try_2_solver(
          distance_lookup,
          valve_lookup,
          &updated_remaining_valves,
          score + ((minutes_remaining_2 - distance - 1) * next_valve.flow_rate as u64),
          minutes_remaining_1,
          minutes_remaining_2 - distance - 1,
          location_1.clone(),
          next_valve_name.clone()
        );
        if score > best_score {
          best_score = score;
        }
      }
    }
  }

  // MEMO_TRY_2.with(|c| c.borrow_mut().insert(memo_key.clone(), best_score as i64));
  best_score
}


// fn part_2_try_2_solver(
//   distance_lookup: &HashMap<(String, String), u64>,
//   valve_lookup: &HashMap<String, Valve>,
//   remaining_valves: &HashSet<String>,
//   score: u64,
//   minutes_remaining_1: u64,
//   minutes_remaining_2: u64,
//   location_1: String,
//   location_2: String,
// ) -> u64 {
//   if remaining_valves.is_empty() {
//     return score;
//   }
//
//   let mut best_score = 0;
//   for next_valve_name in remaining_valves {
//     let mut updated_remaining_valves = remaining_valves.clone();
//     updated_remaining_valves.remove(next_valve_name);
//     let distance = distance_lookup.get(&(location.clone(), next_valve_name.clone())).unwrap();
//     if (distance + 1) < minutes_remaining {
//       let next_valve = VALVE_LOOKUP.with(|c| c.borrow().get(next_valve_name).unwrap().clone());
//       let score = part_1_try_2_solver(
//         distance_lookup,
//         valve_lookup,
//         &updated_remaining_valves,
//         score + ((minutes_remaining - distance - 1) * next_valve.flow_rate as u64),
//         minutes_remaining - distance - 1,
//         next_valve_name.clone(),
//       );
//       if score > best_score {
//         best_score = score;
//       }
//     }
//   }
//
//   best_score
// }


fn solve_from_2(
  starts: StartData,
  opened: HashSet<String>,
  score: i64,
  minutes_left: i64,
) -> i64 {
  let memo_key = make_memo_key(&starts, &opened, score, minutes_left);
  let memo_result = MEMO_2.with(|c| c.borrow().get(&memo_key).cloned());
  match memo_result {
    None => {}
    Some(r) => { return r; }
  }

  let all_opened = VALVES_WITH_NON_ZERO_FLOW_RATE.with(|c| c.borrow().eq(&opened));

  if minutes_left <= 0 || all_opened {
    MEMO_2.with(|c| c.borrow_mut().insert(memo_key, score));
    return score;
  }

  let mut best_score = 0;

  // Both opened
  if starts.start1.just_opened && starts.start2.just_opened {
    let s = solve_from_2(
      StartData {
        start1: StartLocation { name: starts.start1.name, just_opened: false },
        start2: StartLocation { name: starts.start2.name, just_opened: false },
      },
      opened.clone(),
      score,
      minutes_left - 1,
    );
    if s > best_score {
      best_score = s;
    }
  }

  // First opened
  else if starts.start1.just_opened && !starts.start2.just_opened {
    let start2_neighbors = VALVE_LOOKUP.with(|c| c.borrow().get(&starts.start2.name).unwrap().neighbors.clone());
    for n_name in start2_neighbors {
      let n = VALVE_LOOKUP.with(|c| c.borrow().get(&n_name).cloned()).unwrap();
      if !opened.contains(&n_name) && n.flow_rate > 0 {
        let mut updated_opened = opened.clone();
        updated_opened.insert(n_name.clone());
        let s = solve_from_2(
          StartData {
            start1: StartLocation { name: starts.start1.name.clone(), just_opened: false },
            start2: StartLocation { name: n_name.clone(), just_opened: true },
          },
          updated_opened,
          score + ((minutes_left - 2) * n.flow_rate),
          minutes_left - 1,
        );
        if s > best_score {
          best_score = s;
        }
      }
      let s = solve_from_2(
        StartData {
          start1: StartLocation { name: starts.start1.name.clone(), just_opened: false },
          start2: StartLocation { name: n_name.clone(), just_opened: false },
        },
        opened.clone(),
        score,
        minutes_left - 1,
      );
      if s > best_score {
        best_score = s;
      }
    }
  }

  // Second opened
  else if !starts.start1.just_opened && starts.start2.just_opened {
    let start1_neighbors = VALVE_LOOKUP.with(|c| c.borrow().get(&starts.start1.name).unwrap().neighbors.clone());
    for n_name in start1_neighbors {
      let n = VALVE_LOOKUP.with(|c| c.borrow().get(&n_name).cloned()).unwrap();
      if !opened.contains(&n_name) && n.flow_rate > 0 {
        let mut updated_opened = opened.clone();
        updated_opened.insert(n_name.clone());
        let s = solve_from_2(
          StartData {
            start1: StartLocation { name: n_name.clone(), just_opened: true },
            start2: StartLocation { name: starts.start2.name.clone(), just_opened: false },
          },
          updated_opened,
          score + ((minutes_left - 2) * n.flow_rate),
          minutes_left - 1,
        );
        if s > best_score {
          best_score = s;
        }
      }
      let s = solve_from_2(
        StartData {
          start1: StartLocation { name: n_name.clone(), just_opened: false },
          start2: StartLocation { name: starts.start2.name.clone(), just_opened: false },
        },
        opened.clone(),
        score,
        minutes_left - 1,
      );
      if s > best_score {
        best_score = s;
      }
    }
  } else {
    // Neither opened
    let start1_neighbors = VALVE_LOOKUP.with(|c| c.borrow().get(&starts.start1.name).unwrap().neighbors.clone());
    let start2_neighbors = VALVE_LOOKUP.with(|c| c.borrow().get(&starts.start2.name).unwrap().neighbors.clone());
    let combinations = combos(&opened, &start1_neighbors, &start2_neighbors);

    for combo in combinations {
      let n1 = VALVE_LOOKUP.with(|c| c.borrow().get(&combo.0).cloned()).unwrap();
      let n2 = VALVE_LOOKUP.with(|c| c.borrow().get(&combo.1).cloned()).unwrap();

      // Open both
      if combo.0 != combo.1
        && !opened.contains(&n1.name) && n1.flow_rate > 0
        && !opened.contains(&n2.name) && n2.flow_rate > 0 {
        let mut updated_opened = opened.clone();
        updated_opened.insert(n1.name.clone());
        updated_opened.insert(n2.name.clone());

        let s = solve_from_2(
          StartData {
            start1: StartLocation { name: n1.name.clone(), just_opened: true },
            start2: StartLocation { name: n2.name.clone(), just_opened: true },
          },
          updated_opened,
          score + ((minutes_left - 2) * n1.flow_rate) + ((minutes_left - 2) * n2.flow_rate),
          minutes_left - 1,
        );
        if s > best_score {
          best_score = s;
        }
      }
      // Open first
      if !opened.contains(&n1.name) && n1.flow_rate > 0 {
        let mut updated_opened = opened.clone();
        updated_opened.insert(n1.name.clone());

        let s = solve_from_2(
          StartData {
            start1: StartLocation { name: n1.name.clone(), just_opened: true },
            start2: StartLocation { name: n2.name.clone(), just_opened: false },
          },
          updated_opened,
          score + ((minutes_left - 2) * n1.flow_rate),
          minutes_left - 1,
        );
        if s > best_score {
          best_score = s;
        }
      }

      // Open second
      if !opened.contains(&n2.name) && n2.flow_rate > 0 {
        let mut updated_opened = opened.clone();
        updated_opened.insert(n2.name.clone());

        let s = solve_from_2(
          StartData {
            start1: StartLocation { name: n1.name.clone(), just_opened: false },
            start2: StartLocation { name: n2.name.clone(), just_opened: true },
          },
          updated_opened,
          score + ((minutes_left - 2) * n2.flow_rate),
          minutes_left - 1,
        );
        if s > best_score {
          best_score = s;
        }
      }

      // Open neither
      let s = solve_from_2(
        StartData {
          start1: StartLocation { name: n1.name.clone(), just_opened: false },
          start2: StartLocation { name: n2.name.clone(), just_opened: false },
        },
        opened.clone(),
        score,
        minutes_left - 1,
      );
      if s > best_score {
        best_score = s;
      }
    }
  }

  MEMO_2.with(|c| c.borrow_mut().insert(memo_key, best_score));
  return best_score;
}

fn combos(opened: &HashSet<String>, n1: &Vec<String>, n2: &Vec<String>) -> Vec<(String, String)> {
  let mut sorted_1 = n1
    .iter()
    .map(|name| VALVE_LOOKUP.with(|c| c.borrow().get(name).cloned()).unwrap())
    .collect::<Vec<Valve>>();
  sorted_1.sort_by(|a, b| sort_valves(opened, a, b));

  let mut sorted_2 = n2
    .iter()
    .map(|name| VALVE_LOOKUP.with(|c| c.borrow().get(name).cloned()).unwrap())
    .collect::<Vec<Valve>>();
  sorted_2.sort_by(|a, b| sort_valves(opened, a, b));

  let mut combos_set: HashSet<(String, String)> = HashSet::new();
  for _n1 in &sorted_1 {
    for _n2 in &sorted_2 {
      let mut vec = vec![_n1.name.clone(), _n2.name.clone()];
      vec.sort();

      combos_set.insert((vec.get(0).unwrap().to_string(), vec.get(1).unwrap().to_string()));
    }
  };

  Vec::from_iter(combos_set)
}

fn solve_two() -> u64 {
  let highest_score = solve_from_2(
    StartData {
      start1: StartLocation { name: "AA".to_string(), just_opened: false },
      start2: StartLocation { name: "AA".to_string(), just_opened: false },
    },
    HashSet::new(),
    0,
    26,
  );

  let x = 0;

  highest_score as u64
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;
  use std::collections::HashMap;
  use crate::day16::{parse_input, setup_globals, solve_one, solve_one_try_2, solve_two, solve_two_try_2};

  use crate::utils::read_chunks;

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
