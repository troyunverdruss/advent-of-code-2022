use std::collections::HashMap;

use crate::day08::Point;

use crate::utils::read_chunks;

#[allow(dead_code)]
pub fn part_one() -> u64 {
  let lines = read_chunks("day14.txt", "\n");
  let grid = parse_input(&lines);
  simulate_with_abyss(&grid)
}

#[allow(dead_code)]
pub fn part_two() -> u64 {
  let lines = read_chunks("day14.txt", "\n");
  let grid = parse_input(&lines);
  simulate_with_infinite_floor(&grid)
}

fn parse_input(lines: &Vec<String>) -> HashMap<Point, char> {
  let mut grid = HashMap::new();

  lines
    .iter()
    .map(|l| l.split(" -> "))
    .map(|parts| parts.map(|p| p.to_string()).collect::<Vec<String>>())
    .map(|parts| points_to_windows_pairs(&parts))
    .flatten()
    .for_each(|point_pair| add_to_grid(&point_pair, &mut grid));

  grid
}

fn add_to_grid(point_pair: &(Point, Point), grid: &mut HashMap<Point, char>) {
  let (p1, p2) = point_pair;
  if p1.x == p2.x {
    if p1.y < p2.y {
      for y in p1.y..=p2.y {
        grid.insert(Point { x: p1.x, y: y }, '#');
      }
    } else {
      for y in p2.y..=p1.y {
        grid.insert(Point { x: p1.x, y: y }, '#');
      }
    }
  } else {
    if p1.x < p2.x {
      for x in p1.x..=p2.x {
        grid.insert(Point { x: x, y: p1.y }, '#');
      }
    } else {
      for x in p2.x..=p1.x {
        grid.insert(Point { x: x, y: p1.y }, '#');
      }
    }
  }
}

fn points_to_windows_pairs(parts: &Vec<String>) -> Vec<(Point, Point)> {
  parts
    .windows(2)
    .map(|w| {
      (
        str_to_point(w.get(0).unwrap()),
        str_to_point(w.get(1).unwrap())
      )
    })
    .collect()
}

fn str_to_point(str_point: &String) -> Point {
  let parts = str_point.split(",").collect::<Vec<&str>>();
  Point {
    x: parts.get(0).unwrap().parse().unwrap(),
    y: parts.get(1).unwrap().parse().unwrap(),
  }
}

fn simulate_with_abyss(grid: &HashMap<Point, char>) -> u64 {
  let mut grid = grid.clone();

  while add_sand_grain(&mut grid) {
    // Just want to add sand grains, nothing to do in here
  }

  let sand_count: usize = grid
    .values()
    .filter(|v| **v == 'o')
    .count();

  // print_grid(&grid);
  sand_count as u64
}

fn simulate_with_infinite_floor(grid: &HashMap<Point, char>) -> u64 {
  let mut grid = grid.clone();
  let floor_y = grid.keys().map(|k| k.y).max().unwrap() + 2;

  while add_sand_grain(&mut grid) {
    // Update the floor
    let min_x = grid.keys().filter(|p| p.y != floor_y).map(|k| k.x).min().unwrap() -5;
    let max_x = grid.keys().filter(|p| p.y != floor_y).map(|k| k.x).max().unwrap() + 5;

    for x in min_x..=max_x {
      grid.insert(Point { x, y: floor_y }, '#');
    }
  }

  let sand_count: usize = grid
    .values()
    .filter(|v| **v == 'o')
    .count();

  // print_grid(&grid);
  sand_count as u64
}

fn add_sand_grain(grid: &mut HashMap<Point, char>) -> bool {
  let max_y = grid.keys().map(|k| k.y).max().unwrap() + 1;
  let down = Point { x: 0, y: 1 };
  let left = Point { x: -1, y: 1 };
  let right = Point { x: 1, y: 1 };
  let mut position = Point { x: 500, y: 0 };

  loop {
    // Failsafe in case we fall off the bottom
    if position.y > max_y {
      return false;
    }

    // Failsafe for once we're blocked at the entrance
    if *grid.get(&position).unwrap_or(&'.') == 'o' {
      return false;
    }

    let next_down_pos = position + down;
    let next_down_val = grid.get(&next_down_pos).unwrap_or(&'.');
    if *next_down_val == '.' {
      position = next_down_pos;
      continue;
    }

    let next_left_pos = position + left;
    let next_left_val = grid.get(&next_left_pos).unwrap_or(&'.');
    if *next_left_val == '.' {
      position = next_left_pos;
      continue;
    }

    let next_right_pos = position + right;
    let next_right_val = grid.get(&next_right_pos).unwrap_or(&'.');
    if *next_right_val == '.' {
      position = next_right_pos;
      continue;
    }

    // If we can't move anywhere, then we've settled at this position
    grid.insert(position, 'o');
    break;
  }

  true
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<Point, char>) {
  let min_x = grid.keys().map(|k| k.x).min().unwrap() - 1;
  let max_x = grid.keys().map(|k| k.x).max().unwrap() + 1;
  let min_y = grid.keys().map(|k| k.y).min().unwrap() - 1;
  let max_y = grid.keys().map(|k| k.y).max().unwrap() + 1;

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      print!("{}", grid.get(&Point { x, y }).unwrap_or(&'.'));
    }
    println!();
  }
}


#[cfg(test)]
mod tests {
  use crate::day14::{parse_input, print_grid, simulate_with_abyss, simulate_with_infinite_floor};

  #[test]
  fn test_part_1() {
    let input = get_input();
    let grid = parse_input(&input);
    print_grid(&grid);
    assert_eq!(simulate_with_abyss(&grid), 24);
  }

  #[test]
  fn test_part_2() {
    let input = get_input();
    let grid = parse_input(&input);
    print_grid(&grid);
    assert_eq!(simulate_with_infinite_floor(&grid), 93);
  }

  fn get_input() -> Vec<String> {
    vec![
      "498,4 -> 498,6 -> 496,6".to_owned(),
      "503,4 -> 502,4 -> 502,9 -> 494,9".to_owned(),
    ]
  }
}
