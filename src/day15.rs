use std::collections::HashMap;
use std::str::Split;

use crate::day08::Point;
use crate::day09::distance;
use crate::day14::print_grid;

use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day15.txt", "\n");
  solve_one(&lines, 2000000)
}

#[allow(dead_code)]
pub fn part_two() -> u64 {
  0
}

struct InputPair {
  sensor: Point,
  beacon: Point,
}

fn parse_input(lines: &Vec<String>) -> Vec<InputPair> {
  lines
    .iter()
    .map(|s| s.replace(",", ""))
    .map(|s| s.replace("=", " "))
    .map(|s| s.replace(":", " "))
    .map(|s| s.split(" ").map(|s| s.to_string()).collect())
    .map(|s| parts_to_input_pair(&s))
    .collect()
}

fn parts_to_input_pair(parts: &Vec<String>) -> InputPair {
  let x = 0;
  InputPair {
    sensor: Point {
      x: parts.get(3).unwrap().parse().unwrap(),
      y: parts.get(5).unwrap().parse().unwrap(),
    },
    beacon: Point {
      x: parts.get(12).unwrap().parse().unwrap(),
      y: parts.get(14).unwrap().parse().unwrap(),
    },
  }
}

fn solve_one(lines: &Vec<String>, target_row: i64) -> u64 {
  let input_pairs = parse_input(lines);

  let mut grid_row = HashMap::new();
  input_pairs
    .iter()
    .for_each(|ip| {
      maybe_insert_sensor_or_beacon(ip, &mut grid_row, target_row);
    });

  input_pairs
    .iter()
    .for_each(|ip| {
      // let dbg_target = Point { x: 8, y: 7 };
      // if ip.sensor == dbg_target {
              update_target_row(ip, &mut grid_row, target_row);
      // }
    });


  let count: usize = grid_row
    .values()
    .filter(|v| **v == '#' || **v == 'S')
    .count();

  // dbg_print_grid(&grid_row);
  let x = 0;
  count as u64
}

#[allow(dead_code)]
pub fn dbg_print_grid(grid: &HashMap<Point, char>) {
  let min_x = -3;
  let max_x = 25;
  let min_y = grid.keys().map(|k| k.y).min().unwrap();
  let max_y = grid.keys().map(|k| k.y).max().unwrap();

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      print!("{}", grid.get(&Point { x, y }).unwrap_or(&'.'));
    }
    println!();
  }
}

fn maybe_insert_sensor_or_beacon(input_pair: &InputPair, grid_row: &mut HashMap<Point, char>, target_row: i64) {
  if input_pair.sensor.y == target_row {
    grid_row.insert(input_pair.sensor.clone(), 'S');
  }
  if input_pair.beacon.y == target_row {
    grid_row.insert(input_pair.beacon.clone(), 'B');
  }
}

fn update_target_row(input_pair: &InputPair, grid_row: &mut HashMap<Point, char>, target_row: i64) {
  let max_distance = distance(input_pair.sensor, input_pair.beacon);

  // Count up first
  let mut x = input_pair.sensor.x;
  while distance(input_pair.sensor, Point { x, y: target_row }) <= max_distance {
    let target = Point { x, y: target_row };
    match grid_row.get(&target) {
      None => {
        grid_row.insert(target, '#');
      }
      Some(_) => { /* Don't do anything if there's already something here */ }
    }
    x += 1;
  }

  // Then count down
  x = input_pair.sensor.x;
  while distance(input_pair.sensor, Point { x, y: target_row }) <= max_distance {
    let target = Point { x, y: target_row };
    match grid_row.get(&target) {
      None => {
        grid_row.insert(target, '#');
      }
      Some(_) => { /* Don't do anything if there's already something here */ }
    }
    x -= 1;
  }
}


#[cfg(test)]
mod tests {
  use crate::day15::{parse_input, solve_one};

  #[test]
  fn test_part_1() {
    let input = get_input();
    // let parsed = parse_input(&input);
    assert_eq!(solve_one(&input, 10), 26);
  }

  #[test]
  fn debug_print_grid() {
    let input = get_input();
    for y in -2..22 {
      solve_one(&input, y);
    }
  }

  // #[test]
  // fn test_part_2() {
  //   let input = get_input();
  //   let grid = parse_input(&input);
  //   print_grid(&grid);
  //   assert_eq!(simulate_with_infinite_floor(&grid), 93);
  // }

  fn get_input() -> Vec<String> {
    vec![
      "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_owned(),
      "Sensor at x=9, y=16: closest beacon is at x=10, y=16".to_owned(),
      "Sensor at x=13, y=2: closest beacon is at x=15, y=3".to_owned(),
      "Sensor at x=12, y=14: closest beacon is at x=10, y=16".to_owned(),
      "Sensor at x=10, y=20: closest beacon is at x=10, y=16".to_owned(),
      "Sensor at x=14, y=17: closest beacon is at x=10, y=16".to_owned(),
      "Sensor at x=8, y=7: closest beacon is at x=2, y=10".to_owned(),
      "Sensor at x=2, y=0: closest beacon is at x=2, y=10".to_owned(),
      "Sensor at x=0, y=11: closest beacon is at x=2, y=10".to_owned(),
      "Sensor at x=20, y=14: closest beacon is at x=25, y=17".to_owned(),
      "Sensor at x=17, y=20: closest beacon is at x=21, y=22".to_owned(),
      "Sensor at x=16, y=7: closest beacon is at x=15, y=3".to_owned(),
      "Sensor at x=14, y=3: closest beacon is at x=15, y=3".to_owned(),
      "Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_owned(),
    ]
  }
}
