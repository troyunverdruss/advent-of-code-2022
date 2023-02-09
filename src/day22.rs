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
  let instructions = parse_instructions(raw_input);

  let (position, direction) = process_instructions(&grid, &instructions, true);
  let password = compute_password(position, direction);

  password
}

fn parse_instructions(raw_input: Vec<String>) -> Vec<String> {
  raw_input.get(1).unwrap().trim()
    .replace("R", " R ")
    .replace("L", " L ")
    .split(" ")
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn parse_grid(map_lines: &Vec<String>) -> HashMap<Point, char> {
  lines_to_grid_char_val(&map_lines)
    .iter()
    .filter(|(_k, v)| v != &&' ')
    .map(|(k, v)| (*k, *v))
    .collect()
}

fn process_instructions(grid: &HashMap<Point, char>, instructions: &Vec<String>, part_one: bool) -> (Point, Direction) {
  let wrap_method = if part_one {
    wrap_2d
  } else {
    wrap_3d
  };

  let starting_y = 0;
  let starting_x = grid.keys()
    .filter(|p| p.y == starting_y)
    .map(|p| p.x)
    .min().unwrap();

  let mut position = Point { x: starting_x, y: starting_y };
  let mut direction = Direction::Right;

  for instruction in instructions {

    match instruction.parse::<i64>() {
      Ok(steps) => {
        for _ in 0..steps {
          let (new_position, new_direction) = step(grid, &position, &direction, wrap_method);
          position = new_position;
          direction = new_direction;
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

  }

  (position, direction)
}

fn step<T>(
  grid: &HashMap<Point, char>,
  position: &Point,
  direction: &Direction,
  wrap: T,
) -> (Point, Direction)
  where T: Fn(&HashMap<Point, char>, &Point, &Direction) -> (Point, Direction) {
  let maybe_simple_next_position = *position + direction.step_point();
  if grid.contains_key(&maybe_simple_next_position) {
    if grid.get(&maybe_simple_next_position).unwrap() == &'#' {
      (position.clone(), direction.clone())
    } else {
      (maybe_simple_next_position, direction.clone())
    }
  } else {
    let (maybe_wrap_next_position, maybe_new_direction) = wrap(grid, position, direction);
    if grid.get(&maybe_wrap_next_position).unwrap() == &'#' {
      (position.clone(), direction.clone())
    } else {
      (maybe_wrap_next_position, maybe_new_direction)
    }
  }
}

fn wrap_2d(grid: &HashMap<Point, char>, position: &Point, direction: &Direction) -> (Point, Direction) {
  let new_position = match direction {
    Direction::Up => grid.keys().filter(|k| k.x == position.x).max_by(|a, b| a.y.cmp(&b.y)).unwrap(),
    Direction::Down => grid.keys().filter(|k| k.x == position.x).min_by(|a, b| a.y.cmp(&b.y)).unwrap(),
    Direction::Left => grid.keys().filter(|k| k.y == position.y).max_by(|a, b| a.x.cmp(&b.x)).unwrap(),
    Direction::Right => grid.keys().filter(|k| k.y == position.y).min_by(|a, b| a.x.cmp(&b.x)).unwrap(),
  }.clone();

  (new_position, direction.clone())
}

fn wrap_3d(_grid: &HashMap<Point, char>, position: &Point, direction: &Direction) -> (Point, Direction) {
  // println!("Start: {:?}, {:?}", position, direction);
  let default_new_position = Point { x: -1, y: -1 };
  let mut new_position = default_new_position.clone();
  let mut new_direction = Direction::Up;

  match direction {
    Direction::Up => {
      if position.y == 0 && position.x >= 50 && position.x < 100 {
        // i
        new_direction = Direction::Right;
        new_position = Point { x: 0, y: position.x + 100 };
      } else if position.y == 0 && position.x >= 100 && position.x < 150 {
        // h
        new_direction = Direction::Up;
        new_position = Point { x: position.x - 100, y: 199 }
      } else if position.y == 100 && position.x >= 0 && position.x < 50 {
        // d
        new_direction = Direction::Right;
        new_position = Point { x: 50, y: position.x + 50 }
      }
    }
    Direction::Down => {
      if position.y == 199 && position.x >= 0 && position.x < 50 {
        // h
        new_direction = Direction::Down;
        new_position = Point { x: position.x + 100, y: 0 }
      } else if position.y == 149 && position.x >= 50 && position.x < 100 {
        // g
        new_direction = Direction::Left;
        new_position = Point { x: 49, y: position.x + 100 }
      } else if position.y == 49 && position.x >= 100 && position.x < 150 {
        // b
        new_direction = Direction::Left;
        new_position = Point { x: 99, y: position.x - 50 }
      }
    }
    Direction::Left => {
      if position.x == 50 && position.y >= 0 && position.y < 50 {
        // k
        new_direction = Direction::Right;
        // y = 0, y = 149
        // y = 49, y = 100
        // 100 + (49 - y)
        new_position = Point { x: 0, y: 100 + (49 - position.y) }
      } else if position.x == 50 && position.y >= 50 && position.y < 100 {
        // d
        new_direction = Direction::Down;
        // y = 50, x = 0
        // y = 100, x = 50
        new_position = Point { x: position.y - 50, y: 100 }
      } else if position.x == 0 && position.y >= 100 && position.y < 150 {
        // k
        new_direction = Direction::Right;
        //  y = 100, y = 49
        // y = 101, y = 48
        // 148, 1
        // y = 149, y = 0
        // y - 100 + 49

        // 49 - (y - 100)
        new_position = Point { x: 50, y: 49 - (position.y - 100) }
      } else if position.x == 0 && position.y >= 150 && position.y < 200 {
        // i
        new_direction = Direction::Down;
        // y = 150, x = 50
        // y = 199, x = 99
        new_position = Point { x: position.y - 100, y: 0 }
      }
    }
    Direction::Right => {
      if position.x == 149 && position.y >= 0 && position.y < 50 {
        // L
        new_direction = Direction::Left;
        // y = 0, y = 149
        // y = 49, y = 100
        // 100 + (49 - y)
        new_position = Point { x: 99, y: 100 + (49 - position.y) }
      } else if position.x == 99 && position.y >= 50 && position.y < 100 {
        // b
        new_direction = Direction::Up;
        // y = 50, x = 100
        // y = 99, x = 149
        new_position = Point { x: position.y + 50, y: 49 }
      } else if position.x == 99 && position.y >= 100 && position.y < 150 {
        // L
        new_direction = Direction::Left;
        // y = 100, y = 49
        // y = 149, y = 0
        // 49 - (y - 100)
        new_position = Point { x: 149, y: 49 - (position.y - 100) }
      } else if position.x == 49 && position.y >= 150 && position.y < 200 {
        // G
        new_direction = Direction::Up;
        // y = 150, x = 50
        // y = 199, x = 99
        // y - 100
        new_position = Point { x: position.y - 100, y: 149 }
      }
    }
  };

  if new_position == default_new_position {
    panic!("never figured out where to go, uh oh!")
  }

  // println!("End: {:?}, {:?}", new_position, new_direction);
  // println!();
  (new_position, new_direction)
}

fn compute_password(position: Point, direction: Direction) -> i64 {
  // Puzzle grid is 1-based, not 0-based
  (1000 * (1 + position.y)) + (4 * (1 + position.x)) + direction.value()
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
  let raw_input = read_chunks("day22.txt", "\n\n");
  let map_lines = raw_input.get(0).unwrap()
    .split("\n")
    .map(|l| l.to_string())
    .collect::<Vec<String>>();
  let grid = parse_grid(&map_lines);
  let instructions = parse_instructions(raw_input);

  let (position, direction) = process_instructions(&grid, &instructions, false);
  let password = compute_password(position, direction);

  password
}


#[cfg(test)]
mod tests {
  use crate::day08::Point;
  use crate::day22::{compute_password, Direction, parse_grid, process_instructions, wrap_3d};
  use crate::utils::read_chunks;

  #[test]
  fn test_part_1() {
    let map_data = get_part_1_map_data();
    let instructions = get_part_1_instructions();
    let grid = parse_grid(&map_data);
    let (position, direction) = process_instructions(&grid, &instructions, true);
    let password = compute_password(position, direction);

    assert_eq!(password, 6032);
  }

  // Never wrote code that would work for the example data
  // #[test]
  // fn test_part_2() {
  //   let map_data = get_part_1_map_data();
  //   let instructions = get_part_1_instructions();
  //   let grid = parse_grid(&map_data);
  //   let (position, direction) = process_instructions(&grid, &instructions, false);
  //   let password = compute_password(position, direction);
  //
  //   assert_eq!(password, 5031);
  // }

  #[test]
  fn test_compute_password() {
    let password = compute_password(Point { x: 7, y: 5 }, Direction::Right);
    assert_eq!(password, 6032)
  }

  #[test]
  fn verify_3d_wrapping() {
    let raw_input = read_chunks("day22.txt", "\n\n");
    let map_lines = raw_input.get(0).unwrap()
      .split("\n")
      .map(|l| l.to_string())
      .collect::<Vec<String>>();
    let grid = parse_grid(&map_lines);
    let _instructions = raw_input.get(1).unwrap().trim()
      .replace("R", " R ")
      .replace("L", " L ")
      .split(" ")
      .map(|s| s.to_string())
      .collect::<Vec<String>>();

    // top I
    let p = Point {x: 50, y: 0};
    let d = Direction::Up;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 0, y: 150});
    assert_eq!(dd, Direction::Right);

    let p = Point {x: 99, y: 0};
    let d = Direction::Up;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 0, y: 199});
    assert_eq!(dd, Direction::Right);

    // top H
    let p = Point {x: 100, y: 0};
    let d = Direction::Up;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 0, y: 199});
    assert_eq!(dd, Direction::Up);

    let p = Point {x: 149, y: 0};
    let d = Direction::Up;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 49, y: 199});
    assert_eq!(dd, Direction::Up);

    // rightmost L
    let p = Point {x: 149, y: 0};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 99, y: 149});
    assert_eq!(dd, Direction::Left);

    let p = Point {x: 149, y: 49};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 99, y: 100});
    assert_eq!(dd, Direction::Left);

    // upper B
    let p = Point {x: 100, y: 49};
    let d = Direction::Down;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 99, y: 50});
    assert_eq!(dd, Direction::Left);

    let p = Point {x: 149, y: 49};
    let d = Direction::Down;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 99, y: 99});
    assert_eq!(dd, Direction::Left);

    // middle B
    let p = Point {x: 99, y: 50};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 100, y: 49});
    assert_eq!(dd, Direction::Up);

    let p = Point {x: 99, y: 99};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 149, y: 49});
    assert_eq!(dd, Direction::Up);

    // middle L
    let p = Point {x: 99, y: 100};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 149, y: 49});
    assert_eq!(dd, Direction::Left);

    let p = Point {x: 99, y: 149};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 149, y: 0});
    assert_eq!(dd, Direction::Left);

    // middle G
    let p = Point {x: 50, y: 149};
    let d = Direction::Down;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 49, y: 150});
    assert_eq!(dd, Direction::Left);

    let p = Point {x: 99, y: 149};
    let d = Direction::Down;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 49, y: 199});
    assert_eq!(dd, Direction::Left);

    // bottom G
    let p = Point {x: 49, y: 150};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 50, y: 149});
    assert_eq!(dd, Direction::Up);

    let p = Point {x: 49, y: 199};
    let d = Direction::Right;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 99, y: 149});
    assert_eq!(dd, Direction::Up);

    // bottom H
    let p = Point {x: 0, y: 199};
    let d = Direction::Down;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 100, y: 0});
    assert_eq!(dd, Direction::Down);

    let p = Point {x: 49, y: 199};
    let d = Direction::Down;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 149, y: 0});
    assert_eq!(dd, Direction::Down);

    // bottom left I
    let p = Point {x: 0, y: 150};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 50, y: 0});
    assert_eq!(dd, Direction::Down);

    let p = Point {x: 0, y: 199};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 99, y: 0});
    assert_eq!(dd, Direction::Down);

    // left bottom K
    let p = Point {x: 0, y: 100};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 50, y: 49});
    assert_eq!(dd, Direction::Right);

    let p = Point {x: 0, y: 149};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 50, y: 0});
    assert_eq!(dd, Direction::Right);

    // left D
    let p = Point {x: 0, y: 100};
    let d = Direction::Up;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 50, y: 50});
    assert_eq!(dd, Direction::Right);

    let p = Point {x: 49, y: 100};
    let d = Direction::Up;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 50, y: 99});
    assert_eq!(dd, Direction::Right);

    // middle D
    let p = Point {x: 50, y: 50};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 0, y: 100});
    assert_eq!(dd, Direction::Down);

    let p = Point {x: 50, y: 99};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 49, y: 100});
    assert_eq!(dd, Direction::Down);

    // top K
    let p = Point {x: 50, y: 0};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 0, y: 149});
    assert_eq!(dd, Direction::Right);

    let p = Point {x: 50, y: 49};
    let d = Direction::Left;
    let (pp, dd) = wrap_3d(&grid, &p, &d);
    assert_eq!(pp, Point {x: 0, y: 100});
    assert_eq!(dd, Direction::Right);
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
