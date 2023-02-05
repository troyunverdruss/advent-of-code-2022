use std::collections::{HashMap, VecDeque};

use crate::day08::Point;
use crate::day12::lines_to_grid_char_val;
use crate::day23::Direction::{East, North, South, West};
use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let input = read_chunks("day23.txt", "\n");
  let grid = lines_to_grid_char_val(&input);
  solve_one(&grid);


  0
}

fn solve_one(grid: &HashMap<Point, char>) -> i64 {
  let mut next_grid = grid.clone();
  let mut directions = VecDeque::from_iter(vec![North, South, West, East]);

  for _ in 0..10 {
    next_grid = step(&next_grid, directions.front().unwrap());
    directions.rotate_left(1);
  }

  compute_empty_tiles(&next_grid)
}

fn compute_empty_tiles(grid: &HashMap<Point, char>) -> i64 {
  let min_x = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.x).min().unwrap();
  let max_x = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.x).max().unwrap();
  let min_y = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.y).min().unwrap();
  let max_y = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.y).max().unwrap();

  let total_area = (max_x - min_x + 1) * (max_y - min_y + 1);

  total_area - grid.values().filter(|v| v == &&'#').count() as i64
}

fn step(grid: &HashMap<Point, char>, direction: &Direction) -> HashMap<Point, char> {
  todo!()
}

enum Direction {
  North,
  South,
  West,
  East,
}

impl Direction {
  fn consideration_points(&self) -> Vec<Point> {
    match self {
      North => vec![Point { x: 0, y: -1 }, Point { x: 1, y: -1 }, Point { x: -1, y: -1 }],
      South => vec![Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: -1, y: 1 }],
      West => vec![Point { x: -1, y: 0 }, Point { x: -1, y: -1 }, Point { x: -1, y: 1 }],
      East => vec![Point { x: 1, y: 0 }, Point { x: 1, y: -1 }, Point { x: 1, y: -1 }],
    }
  }
}

pub fn part_two() -> i64 {
  0
}


#[cfg(test)]
mod tests {
  use crate::day12::lines_to_grid_char_val;
  use crate::day23::compute_empty_tiles;

  #[test]
  fn test_part_1() {
    // let map_data = get_part_1_map_data();
    // let instructions = get_part_1_instructions();
    // let grid = parse_grid(&map_data);
    // let (position, direction) = process_instructions(&grid, &instructions, true);
    // let password = compute_password(position, direction);
    //
    // assert_eq!(password, 6032);
  }

  #[test]
  fn verify_empty_tiles_math() {
    let end_grid = vec![
      ".......#......".to_string(),
      "...........#..".to_string(),
      "..#.#..#......".to_string(),
      "......#.......".to_string(),
      "...#.....#..#.".to_string(),
      ".#......##....".to_string(),
      ".....##.......".to_string(),
      "..#........#..".to_string(),
      "....#.#..#....".to_string(),
      "..............".to_string(),
      "....#..#..#...".to_string(),
      "..............".to_string(),
    ];
    let grid = lines_to_grid_char_val(&end_grid);
    let empty_tiles = compute_empty_tiles(&grid);
    assert_eq!(empty_tiles, 110)
  }


  fn get_part_1_map_data() -> Vec<String> {
    vec![
      "....#..".to_string(),
      "..###.#".to_string(),
      "#...#.#".to_string(),
      ".#...##".to_string(),
      "#.###..".to_string(),
      "##.#.##".to_string(),
      ".#..#..".to_string(),
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
