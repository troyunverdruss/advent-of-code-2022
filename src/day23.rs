use std::collections::{HashMap, HashSet, VecDeque};

use crate::day08::Point;
use crate::day12::lines_to_grid_char_val;
use crate::day23::Direction::{East, North, South, West};
use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let input = read_chunks("day23.txt", "\n");
  let grid = lines_to_grid_char_val(&input);
  solve_one(&grid)
}

pub fn part_two() -> i64 {
  let input = read_chunks("day23.txt", "\n");
  let grid = lines_to_grid_char_val(&input);
  solve_two(&grid)
}

fn solve_one(grid: &HashMap<Point, char>) -> i64 {
  let mut next_grid = grid.clone();
  println!("initial state");
  // dbg_print_grid(&next_grid);
  let mut directions = VecDeque::from_iter(vec![North, South, West, East]);


  for _i in 0..10 {
    let (_stable, updated_grid) = step(&next_grid, &directions);
    // println!("iteration {}", i);
    next_grid = updated_grid;
    // dbg_print_grid(&next_grid);
    directions.rotate_left(1);
  }

  compute_empty_tiles(&next_grid)
}

fn solve_two(grid: &HashMap<Point, char>) -> i64 {
  let mut next_grid = grid.clone();
  println!("initial state");
  // dbg_print_grid(&next_grid);
  let mut directions = VecDeque::from_iter(vec![North, South, West, East]);
  let mut stable = false;
  let mut round = 0;
  while !stable {
    round += 1;
    let (updated_stable, updated_grid) = step(&next_grid, &directions);
    // println!("iteration {}", i);
    next_grid = updated_grid;
    stable = updated_stable;
    // dbg_print_grid(&next_grid);
    directions.rotate_left(1);
  }

  round
}

#[allow(dead_code)]
pub fn dbg_print_grid(grid: &HashMap<Point, char>) {
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


fn compute_empty_tiles(grid: &HashMap<Point, char>) -> i64 {
  let min_x = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.x).min().unwrap();
  let max_x = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.x).max().unwrap();
  let min_y = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.y).min().unwrap();
  let max_y = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.y).max().unwrap();

  let total_area = (max_x - min_x + 1) * (max_y - min_y + 1);

  total_area - grid.values().filter(|v| v == &&'#').count() as i64
}

struct ProposedMove {
  current: Point,
  destination: Point,
  stable: bool,
}

fn step(grid: &HashMap<Point, char>, directions: &VecDeque<Direction>) -> (bool, HashMap<Point, char>) {
  let mut proposed_moves: Vec<ProposedMove> = Vec::new();

  let elf_locations: Vec<Point> = grid.iter().filter(|(_k, v)| v == &&'#').map(|(k, _v)| k.clone()).collect();
  for elf_pos in elf_locations {
    let can_stay = all_neighbor_vectors()
      .iter()
      .map(|delta| grid.get(&(elf_pos + *delta)).or(Some(&'.')))
      .all(|v| v == Some(&'.'));

    if can_stay {
      proposed_moves.push(ProposedMove { current: elf_pos.clone(), destination: elf_pos.clone(), stable: true });
    } else {
      let test = Point { x: 3, y: 2 };
      if test == elf_pos {
        let x = 0;
      }

      let mut moved = false;
      for d in directions {
        let can_move = d.consideration_vectors()
          .iter()
          .map(|delta| grid.get(&(elf_pos + *delta)).or(Some(&'.')))
          .all(|v| v == Some(&'.'));
        if can_move {
          proposed_moves.push(ProposedMove { current: elf_pos.clone(), destination: elf_pos + d.move_vector(), stable: false });
          moved = true;
          break;
        }
      }

      if !moved {
        proposed_moves.push(ProposedMove { current: elf_pos.clone(), destination: elf_pos.clone(), stable: false });
      }
    }
  }

  let mut dest_count = HashMap::new();
  for pm in &proposed_moves {
    let val = dest_count.get(&pm.destination).unwrap_or(&0);
    dest_count.insert(pm.destination.clone(), val + 1);
  }

  let mut new_grid = HashMap::new();
  for pm in &proposed_moves {
    if dest_count.get(&pm.destination).unwrap() > &1 {
      new_grid.insert(pm.current.clone(), '#');
    } else {
      new_grid.insert(pm.destination.clone(), '#');
    }
  }

  let stable_positions = proposed_moves.iter().all(|pm| pm.stable);
  (stable_positions, new_grid)
}

fn all_neighbor_vectors() -> Vec<Point> {
  vec![
    Point { x: -1, y: -1 }, Point { x: 0, y: -1 }, Point { x: 1, y: -1 },
    Point { x: -1, y: 0 }, Point { x: 1, y: 0 },
    Point { x: -1, y: 1 }, Point { x: 0, y: 1 }, Point { x: 1, y: 1 },
  ]
}

enum Direction {
  North,
  South,
  West,
  East,
}

impl Direction {
  fn consideration_vectors(&self) -> Vec<Point> {
    match self {
      North => vec![Point { x: 0, y: -1 }, Point { x: 1, y: -1 }, Point { x: -1, y: -1 }],
      South => vec![Point { x: 0, y: 1 }, Point { x: 1, y: 1 }, Point { x: -1, y: 1 }],
      West => vec![Point { x: -1, y: 0 }, Point { x: -1, y: -1 }, Point { x: -1, y: 1 }],
      East => vec![Point { x: 1, y: 0 }, Point { x: 1, y: -1 }, Point { x: 1, y: 1 }],
    }
  }

  fn move_vector(&self) -> Point {
    match self {
      North => Point { x: 0, y: -1 },
      South => Point { x: 0, y: 1 },
      West => Point { x: -1, y: 0 },
      East => Point { x: 1, y: 0 }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::cmp::min;
  use std::collections::VecDeque;
  use crate::day12::lines_to_grid_char_val;
  use crate::day23::{compute_empty_tiles, dbg_print_grid, solve_one, solve_two, step};
  use crate::day23::Direction::{East, North, South, West};

  #[test]
  fn test_part_1() {
    let map_data = get_part_1_map_data();
    let grid = lines_to_grid_char_val(&map_data);
    let result = solve_one(&grid);

    assert_eq!(result, 110);
  }

  #[test]
  fn test_part_2() {
    let map_data = get_part_1_map_data();
    let grid = lines_to_grid_char_val(&map_data);
    let result = solve_two(&grid);

    assert_eq!(result, 20);
  }

  #[test]
  fn test_part_1_smaller_example() {
    let map_data = get_part_smaller_example();
    let mut grid = lines_to_grid_char_val(&map_data);
    println!("initial state");
    dbg_print_grid(&grid);
    let mut directions = VecDeque::from_iter(vec![North, South, West, East]);

    for i in 0..3 {
      let (_stable, updated_grid) = step(&grid, &directions);
      println!("iteration {}", i);
      grid = updated_grid;
      dbg_print_grid(&grid);
      directions.rotate_left(1);
    }
  }

  #[test]
  fn check_end_state_for_part_1_matches() {
    let given = vec![
      "......#.....",
      "..........#.",
      ".#.#..#.....",
      ".....#......",
      "..#.....#..#",
      "#......##...",
      "....##......",
      ".#........#.",
      "...#.#..#...",
      "............",
      "...#..#..#..",
    ];
    let mine = vec![
      "......#.....",
      "..........#.",
      ".#.#..#.....",
      ".....#......",
      "..#.....#..#",
      "#......##...",
      "....##......",
      ".#........#.",
      "...#.#..#...",
      "............",
      "...#..#..#..",
    ];
    assert_eq!(given, mine);
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

  fn get_part_smaller_example() -> Vec<String> {
    vec![
      ".....".to_string(),
      "..##.".to_string(),
      "..#..".to_string(),
      ".....".to_string(),
      "..##.".to_string(),
      ".....".to_string(),
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
