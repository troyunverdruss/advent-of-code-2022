use std::collections::HashMap;

use crate::day08::Point;
use crate::day09::distance;
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day15.txt", "\n");
  solve_one(&lines, 2_000_000)
}

#[allow(dead_code)]
pub fn part_two() -> u64 {
  let lines = read_chunks("day15.txt", "\n");
  solve_two(&lines, 4_000_000) as u64
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
  count as u64
}

fn solve_two(lines: &Vec<String>, max_range: i64) -> i64 {
  let input_pairs = parse_input(lines);

  let mut grid = HashMap::new();
  input_pairs
    .iter()
    .enumerate()
    .for_each(|(id, ip)| {
      // println!("{} Processing sensor {:?}", id, ip.sensor);
      draw_circle_in_grid(&mut grid, ip, max_range);
      grid = grid
        .iter()
        .filter(|kv| !within_other_circle((*kv).0, &input_pairs))
        .map(|kv| (kv.0.clone(), kv.1.to_string()))
        .collect();
      // println!("Grid size: {:?}", grid.len());
    });

  let intersections_not_within_other_circles: Vec<(Point, String)> = grid
    .iter()
    .filter(|kv| (*kv).1.parse::<u64>().unwrap() > 1)
    .filter(|kv| !within_other_circle((*kv).0, &input_pairs))
    .map(|kv| (kv.0.clone(), kv.1.to_string()))
    .collect();

  let beacon_location = intersections_not_within_other_circles.get(0).unwrap().clone().0;

  // println!("{:?}", beacon_location);

  compute_tuning_frequency(&beacon_location)
}

fn within_other_circle(point: &Point, input_pairs: &Vec<InputPair>) -> bool {
  for ip in input_pairs {
    let radius = distance(ip.sensor, ip.beacon);
    if distance(ip.sensor, *point) <= radius {
      return true;
    }
  }
  return false;
}

fn compute_tuning_frequency(point: &Point) -> i64{
  (point.x * 4000000) + point.y
}

fn draw_circle_in_grid(grid: &mut HashMap<Point, String>, input_pair: &InputPair, max_range: i64) {
  let radius = distance(input_pair.sensor, input_pair.beacon) + 1;
  let start = Point {x: input_pair.sensor.x, y: input_pair.sensor.y - radius };
  let mut location = start.clone();
  let mut direction = Point { x: 1, y: 1 };

  // grid.insert(input_pair.sensor.clone(), "S".to_owned());

  let default_value = "0".to_string();

  while distance(input_pair.sensor, location) == radius {
    if within_range(&location, max_range) {
      let value = grid.get(&location).unwrap_or(&default_value);
      grid.insert(location.clone(), (value.parse::<u64>().unwrap() + 1).to_string());
    }
      location = location + direction;
  }

  location = Point { x: location.x - 2, y: location.y };
  direction = Point { x: -1, y: 1 };
  while distance(input_pair.sensor, location) == radius {
    if within_range(&location, max_range) {
      let value = grid.get(&location).unwrap_or(&default_value);
      grid.insert(location.clone(), (value.parse::<u64>().unwrap() + 1).to_string());
    }
      location = location + direction;
  }

  location = Point { x: location.x, y: location.y - 2 };
  direction = Point { x: -1, y: -1 };
  while distance(input_pair.sensor, location) == radius {
    if within_range(&location, max_range) {
      let value = grid.get(&location).unwrap_or(&default_value);
      grid.insert(location.clone(), (value.parse::<u64>().unwrap() + 1).to_string());
    }
      location = location + direction;
  }

  location = Point { x: location.x + 2, y: location.y };
  direction = Point { x: 1, y: -1 };
  while distance(input_pair.sensor, location) == radius && location != start {
    if within_range(&location, max_range) {
      let value = grid.get(&location).unwrap_or(&default_value);
      grid.insert(location.clone(), (value.parse::<u64>().unwrap() + 1).to_string());
    }
      location = location + direction;
  }
}

fn within_range(point: &Point, max_range: i64) -> bool {
  point.x >= 0 && point.x <= max_range as i64 && point.y >= 0 && point.y <= max_range as i64
}


#[allow(dead_code)]
pub fn dbg_print_grid(grid: &HashMap<Point, String>) {
  let min_x = grid.keys().map(|k| k.x).min().unwrap();
  let max_x =grid.keys().map(|k| k.x).max().unwrap();
  let min_y = grid.keys().map(|k| k.y).min().unwrap();
  let max_y = grid.keys().map(|k| k.y).max().unwrap();

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      print!("{}", grid.get(&Point { x, y }).unwrap_or(&".".to_string()));
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
      Some(v) => {
        // Only update if this value is a default '.' value
        if v == &'.' {
          grid_row.insert(target, '#');
        }
      }
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
      Some(v) => {
        // Only update if this value is a default '.' value
        if v == &'.' {
          grid_row.insert(target, '#');
        }
      }
    }
    x -= 1;
  }
}


#[cfg(test)]
mod tests {
  use std::cmp::Ordering;
  use std::collections::HashMap;

  use crate::day08::Point;
  use crate::day09::distance;
  use crate::day15::{compute_tuning_frequency, dbg_print_grid, draw_circle_in_grid, InputPair, parse_input, solve_one, solve_two};
  use crate::utils::read_chunks;

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

  #[test]
  fn test_part_2() {
    let input = get_input();
    // let result = solve_two(&input, 20);
    let inputs = parse_input(&input);

    let mut grid = HashMap::new();
    inputs
      .iter()
      .for_each(|ip| draw_circle_in_grid(&mut grid, ip, 20));

    dbg_print_grid(&grid);

    let most_intersections = grid
      .iter()
      .max_by_key(|kv| (*kv).1.parse::<u64>().unwrap())
      .unwrap();

    let tuning_frequency = compute_tuning_frequency(most_intersections.0);

    assert_eq!(tuning_frequency, 56000011);
  }

  #[test]
  fn test_part_2_with_solve_func() {
    let input = get_input();
    let result = solve_two(&input, 20);
    assert_eq!(result, 56000011);
  }

  #[test]
  fn analyze_inputs() {
    impl InputPair {
      fn sort(self: &Self, other: &Self) -> Ordering {
        let self_radius = distance(self.sensor, self.beacon);
        let other_radius = distance(other.sensor, other.beacon);
        if self_radius < other_radius {
          Ordering::Less
        } else if self_radius > other_radius {
          Ordering::Greater
        } else {
          Ordering::Equal
        }
      }
    }

    let lines = read_chunks("day15.txt", "\n");
    let mut inputs = parse_input(&lines);
    inputs.sort_by(InputPair::sort);

    inputs
      .iter()
      .for_each(|ip| {
        println!("Sensor: {:?}, Beacon: {:?}, Diameter: {}", ip.sensor, ip.beacon, distance(ip.sensor, ip.beacon));
      })
  }

  #[test]
  fn test_draw_circle() {
    let ip = InputPair {
      sensor: Point {
        x: -1,
        y: -1,
      },
      beacon: Point {
        x: 2,
        y: 0,
      }
    };

    let mut grid = HashMap::new();
    draw_circle_in_grid(&mut grid, &ip, 200);
    dbg_print_grid(&grid);
  }

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
