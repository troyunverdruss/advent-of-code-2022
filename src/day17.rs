use std::collections::{HashMap, VecDeque};

use crate::day08::Point;
use crate::day09::distance;
use crate::day17::JetBlastDirection::{Left, Right};
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day17.txt", "\n");
  let jet_blasts = parse_input(&lines);
  solve_one(&jet_blasts, 2022)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day15.txt", "\n");
  solve_two(&lines, 4_000_000) as u64
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


fn solve_one(input_jet_blasts: &VecDeque<JetBlastDirection>, max_shapes_to_drop: u64) -> u64 {
  let left = Point { x: -1, y: 0 };
  let right = Point { x: 1, y: 0 };
  let down = Point { x: 0, y: -1 };

  let mut jet_blasts = input_jet_blasts.clone();
  let mut shapes = get_shapes();
  let mut grid: HashMap<Point, char> = HashMap::new();

  let min_x = 0;
  let max_x = 6;
  let min_y = 0;

  for _ in 0..max_shapes_to_drop {
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

  dbg_print_grid(&grid, None);
  grid.keys().map(|k| k.y).max().unwrap() as u64
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
      if y!=0 && (x == -1 || x == 7) {
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


fn solve_two(lines: &Vec<String>, max_range: i64) -> i64 {
  0
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
  use crate::day17::{parse_input, solve_one};

  #[test]
  fn test_part_1() {
    let input = get_input();
    let jet_blasts = parse_input(&input);
    assert_eq!(solve_one(&jet_blasts, 2022), 3068);
  }

  #[test]
  fn test_part_2() {}

  fn get_input() -> Vec<String> {
    vec![
      ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()
    ]
  }
}
