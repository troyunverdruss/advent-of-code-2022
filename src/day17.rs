use std::collections::{HashMap, VecDeque};

use crate::day08::Point;
use crate::day17::JetBlastDirection::{Left, Right};
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day17.txt", "\n");
  let jet_blasts = parse_input(&lines);
  let grid = drop_shapes(&jet_blasts, 2022);
  find_highest_point(&grid)
}

fn find_highest_point(grid: &HashMap<Point, char>) -> u64 {
  grid.keys().map(|k| k.y).max().unwrap() as u64
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day17.txt", "\n");
  let mut jet_blasts = parse_input(&lines);
  let mut shapes = get_shapes();
  let grid_to_analyze = drop_shapes(&jet_blasts, 5_000);
  let analyze_result = analyze_find_first_repeating_index_delta(&grid_to_analyze);
  let mut grid = HashMap::new();

  calculate_height_after_dropping_many_shapes(
    &lines,
    1_000_000_000_000,
    analyze_result,
    &mut jet_blasts,
    &mut shapes,
    &mut grid,
  )
}

fn find_full_line(grid: &HashMap<Point, char>, y_min: u64) -> u64 {
  let height = grid.keys().map(|k| k.y).max().unwrap() as u64;
  for y in y_min..=height {
    if check_if_row_is_new_floor(grid, y) {
      return y;
    }
  }

  panic!("Never found a full row");
}

struct AnalyzeResult {
  offset: u64,
  period: u64,
}

fn analyze_find_first_repeating_index_delta(grid: &HashMap<Point, char>) -> AnalyzeResult {
  let highest_point = find_highest_point(&grid);

  let mut delta_to_line: HashMap<u64, u64> = HashMap::new();

  let mut full_line = find_full_line(&grid, 1);
  let mut repeated_delta_between_lines_prev = 0;

  loop {
    let next_full_line = find_full_line(&grid, full_line + 1);
    let delta = next_full_line - full_line;

    let prev_line = delta_to_line.get(&delta);
    match prev_line {
      None => {}
      Some(line) => {
        let count_of_lines = next_full_line - line;
        if repeated_delta_between_lines_prev > 0 {
          if repeated_delta_between_lines_prev == count_of_lines {
            return AnalyzeResult {
              offset: full_line,
              period: repeated_delta_between_lines_prev,
            };
          } else {
            repeated_delta_between_lines_prev = 0;
          }
        } else {
          repeated_delta_between_lines_prev = count_of_lines;
        }
      }
    }

    delta_to_line.insert(delta, next_full_line);
    full_line = next_full_line;
    if full_line > highest_point {
      break;
    }
  }
  panic!("Couldn't find a repeating pattern")
}

fn calculate_height_after_dropping_many_shapes(
  lines: &Vec<String>,
  target_num_shapes_to_drop: u64,
  analyze_result: AnalyzeResult,
  mut jet_blasts: &mut VecDeque<JetBlastDirection>,
  mut shapes: &mut VecDeque<Vec<Point>>,
  mut grid: &mut HashMap<Point, char>,
) -> u64 {
  assert!(target_num_shapes_to_drop >= 10_000, "Must be at least 10k to drop to use this function");

  let mut found_first = false;
  let mut shapes_to_first_floor = 0;
  let mut height_at_first_floor = 0;
  let mut found_second = false;
  let mut shapes_to_second_floor = 0;
  let mut height_at_second_floor = 0;
  let mut count_dropped_shapes = 0;

  loop {
    drop_one_shape(
      &mut jet_blasts,
      &mut shapes,
      &mut grid,
    );
    count_dropped_shapes += 1;

    if check_if_row_is_new_floor(&grid, analyze_result.offset) && !found_first {
      found_first = true;
      shapes_to_first_floor = count_dropped_shapes;
      height_at_first_floor = find_highest_point(&grid);
      println!("Reached first new floor with # shapes: {}", count_dropped_shapes);
    }
    if check_if_row_is_new_floor(&grid, analyze_result.offset + analyze_result.period) && !found_second {
      found_second = true;
      shapes_to_second_floor = count_dropped_shapes;
      height_at_second_floor = find_highest_point(&grid);
      println!("Reached second new floor with # shapes: {}", count_dropped_shapes);
    }

    if found_first && found_second {
      break;
    }

    if count_dropped_shapes > 5_000 {
      panic!("Couldn't determine the period!")
    }
  }

  if shapes_to_first_floor == 0 || shapes_to_second_floor == 0 {
    panic!("Didn't determine the required shapes to the first and second floor");
  }

  if height_at_first_floor == 0 || height_at_second_floor == 0 {
    panic!("Couldn't figure out height at first and second floor");
  }

  let initial_required_shapes = shapes_to_first_floor;
  let shapes_required_before_repeat = shapes_to_second_floor - shapes_to_first_floor;

  let height_delta_between_floors = height_at_second_floor - height_at_first_floor;

  let remainder_after_repeat = (target_num_shapes_to_drop - initial_required_shapes) % shapes_required_before_repeat;
  let full_repeats = (target_num_shapes_to_drop - initial_required_shapes) / shapes_required_before_repeat;

  let jet_blasts = parse_input(&lines);
  let final_grid_2 = drop_shapes(&jet_blasts, initial_required_shapes + remainder_after_repeat);
  let remainder_highest_point = find_highest_point(&final_grid_2);

  let result = (full_repeats * height_delta_between_floors) + remainder_highest_point;
  result
}


fn check_if_row_is_new_floor(grid: &HashMap<Point, char>, y: u64) -> bool {
  let count = grid
    .iter()
    .filter(|(k, _)| (*k).y == y as i64)
    .filter(|(_, v)| **v == '#')
    .count();

  count == 7
}

#[derive(Clone, Debug)]
enum JetBlastDirection {
  Left,
  Right,
}

fn parse_input(lines: &Vec<String>) -> VecDeque<JetBlastDirection> {
  lines
    .get(0).unwrap()
    .split("")
    .filter(|s| !s.is_empty())
    .map(|v| {
      if v == "<" {
        Left
      } else if v == ">" {
        Right
      } else {
        panic!("Invalid char found: {}", v);
      }
    })
    .collect()
}


fn drop_shapes(input_jet_blasts: &VecDeque<JetBlastDirection>, max_shapes_to_drop: u64) -> HashMap<Point, char> {
  let mut jet_blasts = input_jet_blasts.clone();
  let mut shapes = get_shapes();
  let mut grid: HashMap<Point, char> = HashMap::new();

  for _ in 0..max_shapes_to_drop {
    drop_one_shape(&mut jet_blasts, &mut shapes, &mut grid)
  }

  // dbg_print_grid(&grid, None);
  grid
}

fn drop_one_shape(
  jet_blasts: &mut VecDeque<JetBlastDirection>,
  shapes: &mut VecDeque<Vec<Point>>,
  grid: &mut HashMap<Point, char>,
) {
  let left = Point { x: -1, y: 0 };
  let right = Point { x: 1, y: 0 };
  let down = Point { x: 0, y: -1 };

  let min_x = 0;
  let max_x = 6;
  let min_y = 0;


  // Determine highest current point
  let curr_highest_y = grid.keys().map(|p| p.y).max().or(Some(0)).unwrap();

  // Get next shape and update the shape generator
  let next_shape_raw = shapes.front().unwrap().clone();
  shapes.rotate_left(1);

  // Position the shape correctly
  let mut shape_location: Vec<Point> = next_shape_raw
    .iter()
    .map(|p| *p + Point { x: 2, y: curr_highest_y + 4 })
    .collect();

  // println!("Starting location");
  // dbg_print_grid(&grid, Some(&shape_location));

  // Loop through jet + move down blast until it settles
  loop {
    // Let's see if we can move left/right according to our next jet blast
    let next_jet_blast = jet_blasts.front().unwrap().clone();
    jet_blasts.rotate_left(1);
    // println!("Jet blast: {:?}", next_jet_blast);

    let next_shape_location: Vec<Point> = match next_jet_blast {
      Left => {
        shape_location
          .iter()
          .map(|p| *p + left)
          .collect()
      }
      Right => {
        shape_location
          .iter()
          .map(|p| *p + right)
          .collect()
      }
    };

    // Count up the collisions in this new location
    let collisions = next_shape_location
      .iter()
      .map(|p| grid.get(p))
      .filter(|v| v.is_some())
      .count();

    // Count up out of bounds in this new location
    let out_of_bounds = next_shape_location
      .iter()
      .filter(|p| p.x < min_x || p.x > max_x)
      .count();


    // If we don't have a collision and we're not out of bounds,
    // then this is our next location for the loop
    if collisions == 0 && out_of_bounds == 0 {
      shape_location = next_shape_location;
      // dbg_print_grid(&grid, Some(&shape_location));
    }

    // println!("Moving down");

    // Now, check if we can move downwards
    // If not, we're done!
    let next_shape_location: Vec<Point> = shape_location
      .iter()
      .map(|p| *p + down)
      .collect();

    // Count up the collisions in this new location
    let collisions = next_shape_location
      .iter()
      .map(|p| grid.get(p))
      .filter(|v| v.is_some())
      .count();

    // Count up out of bounds in this new location
    let out_of_bounds = next_shape_location
      .iter()
      .filter(|p| p.y <= min_y)
      .count();

    // If we have a collision or are out of bounds, then the
    // last location is where we're settling
    if collisions > 0 || out_of_bounds > 0 {
      shape_location
        .iter()
        .for_each(|p| {
          grid.insert(p.clone(), '#');
        });
      // println!("Settled location");
      // dbg_print_grid(&grid, None);
      break;
    }

    // Otherwise this is our next location
    shape_location = next_shape_location;
    // dbg_print_grid(&grid, Some(&shape_location));
  }
}

#[allow(dead_code)]
pub fn dbg_print_grid(grid: &HashMap<Point, char>, intermediate_rock: Option<&Vec<Point>>) {
  let mut grid_copy = grid.clone();
  if intermediate_rock.is_some() {
    intermediate_rock.unwrap()
      .iter()
      .for_each(|p| {
        grid_copy.insert(p.clone(), '@');
      })
  }

  let max_y = grid_copy.keys().map(|k| k.y).max().unwrap();

  for y in (0..=max_y).rev() {
    for x in -1..=7 {
      if y == 0 && (x == -1 || x == 7) {
        print!("+");
      } else if y == 0 && x != -1 && x != 7 {
        print!("-");
      }
      if y != 0 && (x == -1 || x == 7) {
        print!("|");
      }
      if y != 0 && x != -1 && x != 7 {
        print!("{}", grid_copy.get(&Point { x, y }).unwrap_or(&'.'));
      }
    }
    println!();
  }
  println!();
}

fn get_shapes() -> VecDeque<Vec<Point>> {
  let shapes_vec = vec![
    // ####
    vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 }, Point { x: 3, y: 0 }],

    // .#.
    // ###
    // .#.
    vec![
      Point { x: 1, y: 2 },
      Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: 2, y: 1 },
      Point { x: 1, y: 0 },
    ],

    // ..#
    // ..#
    // ###
    vec![
      Point { x: 2, y: 2 },
      Point { x: 2, y: 1 },
      Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 },
    ],

    // #
    // #
    // #
    // #
    vec![
      Point { x: 0, y: 3 },
      Point { x: 0, y: 2 },
      Point { x: 0, y: 1 },
      Point { x: 0, y: 0 },
    ],

    // ##
    // ##
    vec![
      Point { x: 0, y: 1 }, Point { x: 1, y: 1 },
      Point { x: 0, y: 0 }, Point { x: 1, y: 0 },
    ],
  ];

  VecDeque::from_iter(shapes_vec)
}


#[cfg(test)]
mod tests {
  use std::collections::{HashMap, VecDeque};

  use crate::day08::Point;
  use crate::day17::{analyze_find_first_repeating_index_delta, AnalyzeResult, calculate_height_after_dropping_many_shapes, check_if_row_is_new_floor, drop_one_shape, drop_shapes, find_full_line, find_highest_point, get_shapes, JetBlastDirection, parse_input};
  use crate::utils::read_chunks;

  #[test]
  fn test_part_1() {
    let input = get_input();
    let jet_blasts = parse_input(&input);
    let final_grid = drop_shapes(&jet_blasts, 2022);
    let highest_point = find_highest_point(&final_grid);
    assert_eq!(highest_point, 3068);
  }

  #[test]
  fn analyze_input_data() {
    // Run the real simulation for 10k shapes to get an actual answer
    let lines = read_chunks("day17.txt", "\n");
    let jet_blasts = parse_input(&lines);
    let target_num_shapes_to_drop = 10_000;
    let final_grid = drop_shapes(&jet_blasts, target_num_shapes_to_drop);
    let actual_highest_point = find_highest_point(&final_grid);
    let analyze_result = analyze_find_first_repeating_index_delta(&final_grid);


    let mut jet_blasts = parse_input(&lines);
    let mut shapes = get_shapes();
    let mut grid = HashMap::new();

    let result = calculate_height_after_dropping_many_shapes(&lines, target_num_shapes_to_drop, analyze_result, &mut jet_blasts, &mut shapes, &mut grid);

    assert_eq!(result, actual_highest_point);
  }

  fn get_input() -> Vec<String> {
    vec![
      ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()
    ]
  }
}
