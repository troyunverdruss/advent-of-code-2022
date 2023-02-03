use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashSet};

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
  let lines = read_chunks("day19.txt", "\n");
  let blueprints = parse_input(&lines);
  // too low 11275
  solve_two(&blueprints)
}

fn solve_two(blueprints: &Vec<Blueprint>) -> u64 {
  let first_three_blueprints = blueprints[0..=2].to_vec();

  let product: usize = first_three_blueprints
    .iter()
    .map(|b| max_geodes_possible(b, 32))
    .product();

  product as u64
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
    .map(|b| fill_in_max_required(&b))
    .collect()
}

fn fill_in_max_required(blueprint: &Blueprint) -> Blueprint {
  Blueprint {
    id: blueprint.id,
    ore_robot_cost: blueprint.ore_robot_cost.clone(),
    clay_robot_cost: blueprint.clay_robot_cost.clone(),
    obsidian_robot_cost: blueprint.obsidian_robot_cost.clone(),
    geode_robot_cost: blueprint.geode_robot_cost.clone(),
    max_robots_required: Quantity {
      ore: max(
        blueprint.ore_robot_cost.ore,
        max(blueprint.clay_robot_cost.ore,
            max(blueprint.obsidian_robot_cost.ore,
                blueprint.geode_robot_cost.ore),
        ),
      ),
      clay: max(
        blueprint.ore_robot_cost.clay,
        max(blueprint.clay_robot_cost.clay,
            max(blueprint.obsidian_robot_cost.clay,
                blueprint.geode_robot_cost.clay),
        ),
      ),
      obsidian: max(
        blueprint.ore_robot_cost.obsidian,
        max(blueprint.obsidian_robot_cost.obsidian,
            max(blueprint.obsidian_robot_cost.obsidian,
                blueprint.geode_robot_cost.obsidian),
        ),
      ),
      geode: 0,
    },
  }
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
    max_robots_required: Quantity { ore: 0, clay: 0, obsidian: 0, geode: 0 },
  }
}

#[derive(Clone, Hash)]
struct State {
  max_required_robots: Quantity,
  minute: usize,
  material: Quantity,
  robots: Quantity,
}

impl State {
  fn score(&self) -> i64 {
    self.minute as i64
  }

  fn to_string(&self) -> String {
    format!("{} {} {}", self.minute, self.robots.to_string(), self.material.to_string())
  }
}

impl Quantity {
  fn to_string(&self) -> String {
    format!("{} {} {} {}", self.ore, self.clay, self.obsidian, self.geode)
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    self.score().cmp(&other.score())
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


fn max_geodes_possible(blueprint: &Blueprint, total_minutes: usize) -> usize {
  let mut final_states: Vec<State> = Vec::new();
  let mut next_states: BinaryHeap<State> = BinaryHeap::new();
  let mut state_seen: HashSet<String> = HashSet::new();


  next_states.push(State {
    max_required_robots: blueprint.max_robots_required.clone(),
    minute: 0,
    material: Quantity { ore: 0, clay: 0, obsidian: 0, geode: 0 },
    robots: Quantity { ore: 1, clay: 0, obsidian: 0, geode: 0 },
  });

  while !next_states.is_empty() {
    let state = next_states.pop().unwrap();
    state_seen.insert(state.to_string());

    if state.minute == total_minutes {
      final_states.push(state);
    } else {
      let states = step(blueprint, &state);
      for state in states {
        if !state_seen.contains(&state.to_string())
        {
          next_states.push(state);
        }
      }
    }
  }

  let max_geodes_cracked = final_states
    .iter()
    .map(|s| s.material.geode)
    .max().unwrap();

  max_geodes_cracked
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
      max_required_robots: blueprint.max_robots_required.clone(),
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
        max_required_robots: blueprint.max_robots_required.clone(),
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
        max_required_robots: blueprint.max_robots_required.clone(),
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
        max_required_robots: blueprint.max_robots_required.clone(),
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
      max_required_robots: blueprint.max_robots_required.clone(),
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
  let geodes = max_geodes_possible(blueprint, 24);
  geodes * blueprint.id
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct Quantity {
  ore: usize,
  clay: usize,
  obsidian: usize,
  geode: usize,
}

#[derive(Clone)]
struct Blueprint {
  id: usize,
  ore_robot_cost: Quantity,
  clay_robot_cost: Quantity,
  obsidian_robot_cost: Quantity,
  geode_robot_cost: Quantity,
  max_robots_required: Quantity,
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
    let geodes = max_geodes_possible(bp1, 24);
    assert_eq!(geodes, 9);
  }

  #[test]
  fn test_find_max_geodes_possible_example_2() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);

    let bp2 = blueprints.get(1).unwrap();
    let geodes = max_geodes_possible(bp2, 24);
    assert_eq!(geodes, 12);
  }

  #[test]
  fn test_find_max_geodes_possible_part_2_example_1() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);

    let bp1 = blueprints.get(0).unwrap();
    let geodes = max_geodes_possible(bp1, 32);
    assert_eq!(geodes, 56);
  }

  #[test]
  fn test_find_max_geodes_possible_part_2_example_2() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);

    let bp2 = blueprints.get(1).unwrap();
    let geodes = max_geodes_possible(bp2, 32);
    assert_eq!(geodes, 62);
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
