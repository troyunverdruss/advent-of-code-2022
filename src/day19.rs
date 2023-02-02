use std::collections::{HashMap, HashSet, VecDeque};
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
  // Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 9 obsidian.
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
    ore_robot_cost: Cost {
      ore: parts.get(6).unwrap().parse().unwrap(),
      clay: 0,
      obsidian: 0,
      geode: 0,
    },
    clay_robot_cost: Cost {
      ore: parts.get(12).unwrap().parse().unwrap(),
      clay: 0,
      obsidian: 0,
      geode: 0,
    },
    obsidian_robot_cost: Cost {
      ore: parts.get(18).unwrap().parse().unwrap(),
      clay: parts.get(21).unwrap().parse().unwrap(),
      obsidian: 0,
      geode: 0,
    },
    geode_robot_cost: Cost {
      ore: parts.get(27).unwrap().parse().unwrap(),
      clay: 0,
      obsidian: parts.get(30).unwrap().parse().unwrap(),
      geode: 0,
    },
  }
}

fn max_geodes_possible(blueprint: &Blueprint) -> usize {
  0
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

struct Cost {
  ore: usize,
  clay: usize,
  obsidian: usize,
  geode: usize,
}

struct Blueprint {
  id: usize,
  ore_robot_cost: Cost,
  clay_robot_cost: Cost,
  obsidian_robot_cost: Cost,
  geode_robot_cost: Cost,
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
  fn test_find_max_geodes_possible() {
    let input = get_part_1_input();
    let blueprints = parse_input(&input);

    let bp1 = blueprints.get(0).unwrap();
    let geodes = max_geodes_possible(bp1);
    assert_eq!(geodes, 9);

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
