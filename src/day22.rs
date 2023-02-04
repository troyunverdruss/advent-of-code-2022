use std::collections::HashMap;

use crate::day08::Point;
use crate::day12::lines_to_grid_char_val;
use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let raw_input = read_chunks("day22.txt", "\n\n");
  let map_lines = raw_input.get(0).unwrap()
    .split("\n")
    .map(|l| l.to_string())
    .collect::<Vec<String>>();
  let grid = parse_grid(&map_lines);
  let instructions = raw_input.get(1).unwrap().trim()
    .replace("R", " R ")
    .replace("L", " L ")
    .split(" ")
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  let (position, direction) = process_instructions(&grid, &instructions);
  let password = compute_password(position, direction);

  password
}

fn parse_grid(map_lines: &Vec<String>) -> HashMap<Point, char> {
  lines_to_grid_char_val(&map_lines)
    .iter()
    .filter(|(k, v)| v != &&' ')
    .map(|(k, v)| (*k, *v))
    .collect()
}

fn process_instructions(grid: &HashMap<Point, char>, instructions: &Vec<String>) -> (Point, Direction) {
  let starting_y = 0;
  let starting_x = grid.keys()
    .filter(|p| p.y == starting_y)
    .map(|p| p.x)
    .min().unwrap();

  let mut position = Point { x: starting_x, y: starting_y };
  let mut direction = Direction::Right;

  for instruction in instructions {
    println!("Start: {:?}, {:?}", position, direction);
    match instruction.parse::<i64>() {
      Ok(steps) => {
        for _ in 0..steps {
          position = step(grid, &position, &direction, wrap_2d);
        }
      }
      Err(_) => {
        direction = match instruction.as_str() {
          "R" => direction.turn_right(),
          "L" => direction.turn_left(),
          _ => panic!("unknown direction")
        }
      }
    }
    println!("End: {:?}, {:?}", position, direction);
    println!();
  }

  (position, direction)
}

fn step<T>(
  grid: &HashMap<Point, char>,
  position: &Point,
  direction: &Direction,
  wrap: T
) -> Point
where T: Fn(&HashMap<Point, char>, &Point, &Direction) -> Point {
  let maybe_simple_next_position = *position + direction.step_point();
  if grid.contains_key(&maybe_simple_next_position) {
    if grid.get(&maybe_simple_next_position).unwrap() == &'#' {
      position.clone()
    } else {
      maybe_simple_next_position
    }
  } else {
    let maybe_wrap_next_position = wrap(grid, position, direction);

    if grid.get(&maybe_wrap_next_position).unwrap() == &'#' {
      position.clone()
    } else {
      maybe_wrap_next_position
    }
  }
}

fn wrap_2d(grid: &HashMap<Point, char>, position: &Point, direction: &Direction) -> Point {
  match direction {
    Direction::Up => grid.keys().filter(|k| k.x == position.x).max_by(|a, b| a.y.cmp(&b.y)).unwrap(),
    Direction::Down => grid.keys().filter(|k| k.x == position.x).min_by(|a, b| a.y.cmp(&b.y)).unwrap(),
    Direction::Left => grid.keys().filter(|k| k.y == position.y).max_by(|a, b| a.x.cmp(&b.x)).unwrap(),
    Direction::Right => grid.keys().filter(|k| k.y == position.y).min_by(|a, b| a.x.cmp(&b.x)).unwrap(),
  }.clone()
}

fn wrap_3d(grid: &HashMap<Point, char>, position: &Point, direction: &Direction) -> Point {
  let is_small = grid.len() == 96;

  match direction {
    Direction::Up => grid.keys().filter(|k| k.x == position.x).max_by(|a, b| a.y.cmp(&b.y)).unwrap(),
    Direction::Down => grid.keys().filter(|k| k.x == position.x).min_by(|a, b| a.y.cmp(&b.y)).unwrap(),
    Direction::Left => grid.keys().filter(|k| k.y == position.y).max_by(|a, b| a.x.cmp(&b.x)).unwrap(),
    Direction::Right => grid.keys().filter(|k| k.y == position.y).min_by(|a, b| a.x.cmp(&b.x)).unwrap(),
  }.clone()
}

fn compute_password(position: Point, direction: Direction) -> i64 {
  // Puzzle grid is 1-based, not 0-based
  (1000 * (1 + position.y)) + (4 * (1 + position.x)) + direction.value()
}

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn value(&self) -> i64 {
    match self {
      Direction::Up => 3,
      Direction::Down => 1,
      Direction::Left => 2,
      Direction::Right => 0,
    }
  }

  fn turn_right(&self) -> Direction {
    match self {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }

  fn turn_left(&self) -> Direction {
    match self {
      Direction::Up => Direction::Left,
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
      Direction::Right => Direction::Up,
    }
  }

  fn step_point(&self) -> Point {
    match self {
      Direction::Up => Point { x: 0, y: -1 },
      Direction::Down => Point { x: 0, y: 1 },
      Direction::Left => Point { x: -1, y: 0 },
      Direction::Right => Point { x: 1, y: 0 },
    }
  }
}

pub fn part_two() -> i64 {
  0
}


#[cfg(test)]
mod tests {
  use crate::day08::Point;
  use crate::day22::{compute_password, Direction, parse_grid, process_instructions};

  #[test]
  fn test_part_1() {
    let map_data = get_part_1_map_data();
    let instructions = get_part_1_instructions();
    let grid = parse_grid(&map_data);
    let (position, direction) = process_instructions(&grid, &instructions);
    let password = compute_password(position, direction);

    assert_eq!(password, 6032);
  }

  #[test]
  fn test_part_2() {
    let map_data = get_part_1_map_data();
    let instructions = get_part_1_instructions();
    let grid = parse_grid(&map_data);
    let (position, direction) = process_instructions(&grid, &instructions);
    let password = compute_password(position, direction);

    assert_eq!(password, 6032);
  }

  #[test]
  fn test_compute_password() {
    let password = compute_password(Point { x: 7, y: 5 }, Direction::Right);
    assert_eq!(password, 6032)
  }


  fn get_part_1_map_data() -> Vec<String> {
    vec![
      "        ...#".to_string(),
      "        .#..".to_string(),
      "        #...".to_string(),
      "        ....".to_string(),
      "...#.D.....#".to_string(),
      "........#...".to_string(),
      "B.#....#...A".to_string(),
      ".....C....#.".to_string(),
      "        ...#....".to_string(),
      "        .....#..".to_string(),
      "        .#......".to_string(),
      "        ......#.".to_string(),
    ]
  }

  fn get_part_1_instructions() -> Vec<String> {
    vec![
      "10".to_string(),
      "R".to_string(),
      "5".to_string(),
      "L".to_string(),
      "5".to_string(),
      "R".to_string(),
      "10".to_string(),
      "L".to_string(),
      "4".to_string(),
      "R".to_string(),
      "5".to_string(),
      "L".to_string(),
      "5".to_string(),
    ]
  }
}
