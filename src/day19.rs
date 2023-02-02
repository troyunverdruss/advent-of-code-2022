use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::ops::Add;
use std::str::Split;


use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day19.txt", "\n");
  let blueprints = parse_input(&lines);
  solve_one(&blueprints)
}

fn solve_one(blueprints: &Vec<Blueprint>) -> u64 {
  let final_score: usize = blueprints
    .iter()
    .map(|b| find_quality_level_for_blueprint(b))
    .sum();

  final_score as u64
}

pub fn part_two() -> u64 {
  0
}

fn parse_input(lines: &Vec<String>) -> Vec<Blueprint> {
  // Blueprint 1:
  // Each ore robot costs 2 ore.
  // Each clay robot costs 4 ore.
  // Each obsidian robot costs 2 ore and 16 clay.
  // Each geode robot costs 2 ore and 9 obsidian.
  lines
    .iter()
    .map(|s| s.replace(":", ""))
    .map(|s| s.replace(".", ""))
    .map(|s| s.split(" ").map(|p| p.to_string()).collect::<Vec<String>>())
    .map(|s| str_to_blueprint(&s))
    .collect()
}

fn str_to_blueprint(parts: &Vec<String>) -> Blueprint {
  Blueprint {
    id: parts.get(1).unwrap().parse().unwrap(),
    ore_robot_cost: Quantity {
      ore: parts.get(6).unwrap().parse().unwrap(),
      clay: 0,
      obsidian: 0,
      geode: 0,
    },
    clay_robot_cost: Quantity {
      ore: parts.get(12).unwrap().parse().unwrap(),
      clay: 0,
      obsidian: 0,
      geode: 0,
    },
    obsidian_robot_cost: Quantity {
      ore: parts.get(18).unwrap().parse().unwrap(),
      clay: parts.get(21).unwrap().parse().unwrap(),
      obsidian: 0,
      geode: 0,
    },
    geode_robot_cost: Quantity {
      ore: parts.get(27).unwrap().parse().unwrap(),
      clay: 0,
      obsidian: parts.get(30).unwrap().parse().unwrap(),
      geode: 0,
    },
  }
}

struct State {
  minute: usize,
  material: Quantity,
  robots: Quantity,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    self.material.geode.cmp(&other.material.geode)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.minute == other.minute &&
      self.material == other.material &&
      self.robots == other.robots
  }
}

impl Eq for State {}


fn max_geodes_possible(blueprint: &Blueprint) -> usize {
  let mut final_states: Vec<State> = Vec::new();
  let mut next_states: BinaryHeap<State> = BinaryHeap::new();
  let mut max_geodes_per_min: HashMap<usize, usize> = HashMap::new();

  next_states.push(State {
    minute: 0,
    material: Quantity { ore: 0, clay: 0, obsidian: 0, geode: 0 },
    robots: Quantity { ore: 1, clay: 0, obsidian: 0, geode: 0 },
  });

  while !next_states.is_empty() {
    let state = next_states.pop().unwrap();
    if state.minute == 24 {
      final_states.push(state);
    } else {
      let states = step(blueprint, &state);
      let y = 0;
      for state in states {
        let curr_max = max_geodes_per_min.get(&state.minute).or(Some(&0)).unwrap().clone();
        let curr_geode_count = state.material.geode;
        if curr_geode_count >= curr_max {
          let curr_min = state.minute.clone();
          next_states.push(state);
          max_geodes_per_min.insert(curr_min, curr_geode_count.clone());
        }
      }
    }
  }

  final_states
    .iter()
    .map(|s| s.material.geode)
    .max().unwrap()
}

fn step(blueprint: &Blueprint, state: &State) -> Vec<State> {
  let mut next_states = Vec::new();
  let mut built_robots = 0;
  let mut built_geode_robot = false;
  // If I can afford a geode robot, make one
  if state.material.ore >= blueprint.geode_robot_cost.ore
    && state.material.obsidian >= blueprint.geode_robot_cost.obsidian {
    built_geode_robot = true;
    let s = State {
      minute: state.minute + 1,
      material: Quantity {
        ore: (state.material.ore - blueprint.geode_robot_cost.ore) + state.robots.ore,
        clay: state.material.clay + state.robots.clay,
        obsidian: (state.material.obsidian - blueprint.geode_robot_cost.obsidian) + state.robots.obsidian,
        geode: state.material.geode + state.robots.geode,
      },
      robots: Quantity {
        ore: state.robots.ore,
        clay: state.robots.clay,
        obsidian: state.robots.obsidian,
        geode: state.robots.geode + 1,
      },
    };
    next_states.push(s);
  } else {
    // If I can afford an obsidian robot, make one
    let max_req_obsidian =
      max(
        blueprint.ore_robot_cost.obsidian,
        max(blueprint.obsidian_robot_cost.obsidian,
            max(blueprint.obsidian_robot_cost.obsidian,
                blueprint.geode_robot_cost.obsidian),
        ),
      );
    if state.material.ore >= blueprint.obsidian_robot_cost.ore &&
      state.material.clay >= blueprint.obsidian_robot_cost.clay &&
      state.robots.obsidian < max_req_obsidian {
      built_robots += 1;
      let s = State {
        minute: state.minute + 1,
        material: Quantity {
          ore: (state.material.ore - blueprint.obsidian_robot_cost.ore) + state.robots.ore,
          clay: (state.material.clay - blueprint.obsidian_robot_cost.clay) + state.robots.clay,
          obsidian: state.material.obsidian + state.robots.obsidian,
          geode: state.material.geode + state.robots.geode,
        },
        robots: Quantity {
          ore: state.robots.ore,
          clay: state.robots.clay,
          obsidian: state.robots.obsidian + 1,
          geode: state.robots.geode,
        },
      };
      next_states.push(s);
    }
    // If I can afford a clay robot, make one
    let max_req_clay =
      max(
        blueprint.ore_robot_cost.clay,
        max(blueprint.clay_robot_cost.clay,
            max(blueprint.obsidian_robot_cost.clay,
                blueprint.geode_robot_cost.clay),
        ),
      );
    if state.material.ore >= blueprint.clay_robot_cost.ore && state.robots.clay < max_req_clay {
      built_robots += 1;
      let s = State {
        minute: state.minute + 1,
        material: Quantity {
          ore: (state.material.ore - blueprint.clay_robot_cost.ore) + state.robots.ore,
          clay: state.material.clay + state.robots.clay,
          obsidian: state.material.obsidian + state.robots.obsidian,
          geode: state.material.geode + state.robots.geode,
        },
        robots: Quantity {
          ore: state.robots.ore,
          clay: state.robots.clay + 1,
          obsidian: state.robots.obsidian,
          geode: state.robots.geode,
        },
      };
      next_states.push(s);
    }

    // If I can afford an ore robot, make one
    let max_req_ore =
      max(
        blueprint.ore_robot_cost.ore,
        max(blueprint.clay_robot_cost.ore,
            max(blueprint.obsidian_robot_cost.ore,
                blueprint.geode_robot_cost.ore),
        ),
      );
    if state.material.ore >= blueprint.ore_robot_cost.ore && state.robots.ore < max_req_ore {
      built_robots += 1;
      let s = State {
        minute: state.minute + 1,
        material: Quantity {
          ore: (state.material.ore - blueprint.ore_robot_cost.ore) + state.robots.ore,
          clay: state.material.clay + state.robots.clay,
          obsidian: state.material.obsidian + state.robots.obsidian,
          geode: state.material.geode + state.robots.geode,
        },
        robots: Quantity {
          ore: state.robots.ore + 1,
          clay: state.robots.clay,
          obsidian: state.robots.obsidian,
          geode: state.robots.geode,
        },
      };
      next_states.push(s);
    }
  }


  // Last option is to just collect ore
  // if !built_any_robot {
  if built_robots < 3 && !built_geode_robot {
    let s = State {
      minute: state.minute + 1,
      material: Quantity {
        ore: state.material.ore + state.robots.ore,
        clay: state.material.clay + state.robots.clay,
        obsidian: state.material.obsidian + state.robots.obsidian,
        geode: state.material.geode + state.robots.geode,
      },
      robots: Quantity {
        ore: state.robots.ore,
        clay: state.robots.clay,
        obsidian: state.robots.obsidian,
        geode: state.robots.geode,
      },
    };
    next_states.push(s);
  }


  next_states
}


fn find_quality_level_for_blueprint(blueprint: &Blueprint) -> usize {
  let geodes = max_geodes_possible(blueprint);
  geodes * blueprint.id
}

enum Material {
  Ore,
  Clay,
  Obsidian,
  Geode,
}

#[derive(PartialEq, Eq)]
struct Quantity {
  ore: usize,
  clay: usize,
  obsidian: usize,
  geode: usize,
}

struct Blueprint {
  id: usize,
  ore_robot_cost: Quantity,
  clay_robot_cost: Quantity,
  obsidian_robot_cost: Quantity,
  geode_robot_cost: Quantity,
}

#[cfg(test)]
mod tests {
  use crate::day19::{max_geodes_possible, parse_input, solve_one};

  #[test]
  fn test_parsing_simple_input() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);
    let bp1 = blueprints.get(0).unwrap();
    let bp2 = blueprints.get(1).unwrap();

    assert_eq!(bp1.id, 1);
    assert_eq!(bp1.ore_robot_cost.ore, 4);
    assert_eq!(bp1.clay_robot_cost.ore, 2);
    assert_eq!(bp1.obsidian_robot_cost.ore, 3);
    assert_eq!(bp1.obsidian_robot_cost.clay, 14);
    assert_eq!(bp1.geode_robot_cost.ore, 2);
    assert_eq!(bp1.geode_robot_cost.obsidian, 7);

    assert_eq!(bp2.id, 2);
    assert_eq!(bp2.ore_robot_cost.ore, 2);
    assert_eq!(bp2.clay_robot_cost.ore, 3);
    assert_eq!(bp2.obsidian_robot_cost.ore, 3);
    assert_eq!(bp2.obsidian_robot_cost.clay, 8);
    assert_eq!(bp2.geode_robot_cost.ore, 3);
    assert_eq!(bp2.geode_robot_cost.obsidian, 12);
  }

  #[test]
  fn test_find_max_geodes_possible_example_1() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);

    let bp1 = blueprints.get(0).unwrap();
    let geodes = max_geodes_possible(bp1);
    assert_eq!(geodes, 9);
  }

  #[test]
  fn test_find_max_geodes_possible_example_2() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);

    let bp2 = blueprints.get(1).unwrap();
    let geodes = max_geodes_possible(bp2);
    assert_eq!(geodes, 12);
  }

  #[test]
  fn test_part_1() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);
    let total_quality = solve_one(&blueprints);
    assert_eq!(total_quality, 33);
  }

  fn get_part_1_input() -> Vec<String> {
    vec![
      "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_string(),
      "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_string(),
    ]
  }
}
