use std::collections::{HashMap, HashSet};
use crate::day08::Point;
use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let lines = read_chunks("day09.txt", "\n");
  solve_one(&lines)
}

pub fn part_two() -> u64 {
  // let lines = read_chunks("day09.txt", "\n");
  // solve_two(&lines)
  0
}

fn solve_one(lines: &Vec<String>) -> i64 {
  let mut x_register = 1;
  let mut cycle = 1;

  let key_cycles = vec![20, 60, 100, 140, 180, 220];
  let mut key_values: Vec<i64> = Vec::new();

  for line in lines {
    if line == "noop" {
      // do nothing
      cycle += 1;
    } else {
      let parts: Vec<String> = line.split(' ').map(|p| p.to_string()).collect();
      let value: i64 = parts.get(1).unwrap().parse().unwrap();
      cycle += 1;
      if key_cycles.contains(&cycle) {
        key_values.push(cycle * x_register.clone());
      }
      x_register += value;
      cycle += 1;
    }
    if key_cycles.contains(&cycle) {
      key_values.push(cycle * x_register.clone());
    }
  }

  key_values.iter().sum()
}

#[cfg(test)]
mod tests {
  use crate::day10::solve_one;

  #[test]
  fn test_sample_1() {
    let inputs = get_inputs();
    let result = solve_one(&inputs);
    assert_eq!(result, 13140);
  }
  #[test]
  fn test_simple_sample_1() {
    let inputs = get_basic_input();
    let result = solve_one(&inputs);
    assert_eq!(result, 0);
  }

  fn get_basic_input() -> Vec<String> {
    vec![
      "noop",
      "addx 3",
      "addx -5",
    ]
      .iter()
      .map(|s| s.to_string())
      .collect()
  }

  fn get_inputs() -> Vec<String> {
    let inputs: Vec<String> = vec![
      "addx 15",
      "addx -11",
      "addx 6",
      "addx -3",
      "addx 5",
      "addx -1",
      "addx -8",
      "addx 13",
      "addx 4",
      "noop",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx 5",
      "addx -1",
      "addx -35",
      "addx 1",
      "addx 24",
      "addx -19",
      "addx 1",
      "addx 16",
      "addx -11",
      "noop",
      "noop",
      "addx 21",
      "addx -15",
      "noop",
      "noop",
      "addx -3",
      "addx 9",
      "addx 1",
      "addx -3",
      "addx 8",
      "addx 1",
      "addx 5",
      "noop",
      "noop",
      "noop",
      "noop",
      "noop",
      "addx -36",
      "noop",
      "addx 1",
      "addx 7",
      "noop",
      "noop",
      "noop",
      "addx 2",
      "addx 6",
      "noop",
      "noop",
      "noop",
      "noop",
      "noop",
      "addx 1",
      "noop",
      "noop",
      "addx 7",
      "addx 1",
      "noop",
      "addx -13",
      "addx 13",
      "addx 7",
      "noop",
      "addx 1",
      "addx -33",
      "noop",
      "noop",
      "noop",
      "addx 2",
      "noop",
      "noop",
      "noop",
      "addx 8",
      "noop",
      "addx -1",
      "addx 2",
      "addx 1",
      "noop",
      "addx 17",
      "addx -9",
      "addx 1",
      "addx 1",
      "addx -3",
      "addx 11",
      "noop",
      "noop",
      "addx 1",
      "noop",
      "addx 1",
      "noop",
      "noop",
      "addx -13",
      "addx -19",
      "addx 1",
      "addx 3",
      "addx 26",
      "addx -30",
      "addx 12",
      "addx -1",
      "addx 3",
      "addx 1",
      "noop",
      "noop",
      "noop",
      "addx -9",
      "addx 18",
      "addx 1",
      "addx 2",
      "noop",
      "noop",
      "addx 9",
      "noop",
      "noop",
      "noop",
      "addx -1",
      "addx 2",
      "addx -37",
      "addx 1",
      "addx 3",
      "noop",
      "addx 15",
      "addx -21",
      "addx 22",
      "addx -6",
      "addx 1",
      "noop",
      "addx 2",
      "addx 1",
      "noop",
      "addx -10",
      "noop",
      "noop",
      "addx 20",
      "addx 1",
      "addx 2",
      "addx 2",
      "addx -6",
      "addx -11",
      "noop",
      "noop",
      "noop",
    ]
      .iter()
      .map(|l| l.to_string())
      .collect();
    inputs
  }
}
