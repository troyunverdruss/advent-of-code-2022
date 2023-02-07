use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::day08::Point;
use crate::day12::lines_to_grid_char_val;
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let input = read_chunks("day24.txt", "\n");
  let grid = lines_to_grid_char_val(&input);
  solve_one(&grid)
}

fn solve_one(grid: &HashMap<Point, char>) -> u64 {
  let (start, end) = get_start_end(grid);
  let walls = get_walls(grid);
  let blizzards = get_blizzards(grid);
  let all_indexed_blizzard_state_locations = index_all_blizzard_state_locations(&walls, &blizzards);

  let mut to_visit = VecDeque::new();
  to_visit.push_back(State { steps: 0, position: start.clone() });
  let mut visited = HashSet::new();

  let default_path_upper_bound = 1_000_000_000_000;
  let mut shortest_path: u64 = default_path_upper_bound;

  while !to_visit.is_empty() {
    let curr = to_visit.pop_front().unwrap();
    if curr.position == end {
      shortest_path = min(shortest_path, curr.steps as u64);
    }
    if curr.steps <= shortest_path as usize {
      let maybe_next_positions = next_step_vectors()
        .iter()
        .map(|v| curr.position + *v)
        .collect::<Vec<Point>>();
      let next_blizzard_positions = all_indexed_blizzard_state_locations
        .get(&((curr.steps + 1) % all_indexed_blizzard_state_locations.len())).unwrap();

      maybe_next_positions
        .iter()
        .for_each(|p| {
          let is_valid = !next_blizzard_positions.contains(p) && !walls.contains(p) && p.y >= 0;

          if is_valid {
            let next_state = State { steps: curr.steps + 1, position: p.clone() };
            let normalized_next_state = normalize_state(
              all_indexed_blizzard_state_locations.len(),
              next_state,
            );
            if !visited.contains(&normalized_next_state) && !to_visit.contains(&next_state) {
              to_visit.push_back(next_state)
            }
            visited.insert(normalize_state(all_indexed_blizzard_state_locations.len(), curr));
          }
        })
    }
  }

  if shortest_path == default_path_upper_bound {
    panic!("never found a path");
  }

  let x = 0;
  shortest_path
}

fn normalize_state(blizzard_states: usize, next_state: State) -> State {
  State {
    steps: next_state.steps % blizzard_states,
    position: next_state.position.clone(),
  }
}

fn next_step_vectors() -> Vec<Point> {
  vec![
    Point { x: 0, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
  ]
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct State {
  steps: usize,
  position: Point,
}

fn index_all_blizzard_state_locations(walls: &HashSet<Point>, blizzards: &Vec<Blizzard>) -> HashMap<usize, HashSet<Point>> {
  let mut blizzard_states = HashMap::new();

  // Insert t=0
  blizzard_states.insert(0, blizzards.iter().map(|b| b.position.clone()).collect());

  // Now compute and insert them until we fully wrap around
  let min_x = walls.iter().map(|k| k.x).min().unwrap();
  let max_x = walls.iter().map(|k| k.x).max().unwrap();
  let min_y = walls.iter().map(|k| k.y).min().unwrap();
  let max_y = walls.iter().map(|k| k.y).max().unwrap();
  let width = max_x - min_x - 2;
  let height = max_y - min_y - 2;

  let mut next_blizzards = blizzards.clone();
  for i in 0..(height * width) {
    blizzard_states.insert(i as usize, next_blizzards.iter().map(|b| b.position.clone()).collect());
    let tmp_blizzards = step_blizzards(walls, &next_blizzards);
    next_blizzards = tmp_blizzards;
  }

  blizzard_states
}

fn step_blizzards(walls: &HashSet<Point>, curr_blizzards: &Vec<Blizzard>) -> Vec<Blizzard> {
  let mut next_blizzards = Vec::new();

  let min_x = walls.iter().map(|k| k.x).min().unwrap();
  let max_x = walls.iter().map(|k| k.x).max().unwrap();
  let min_y = walls.iter().map(|k| k.y).min().unwrap();
  let max_y = walls.iter().map(|k| k.y).max().unwrap();

  for b in curr_blizzards {
    let maybe_next_position = get_next_position(b);
    if walls.contains(&maybe_next_position) {
      let wrap_position = match b.direction {
        '<' => Point { x: max_x - 1, y: b.position.y },
        '>' => Point { x: min_x + 1, y: b.position.y },
        '^' => Point { x: b.position.x, y: max_y - 1 },
        'v' => Point { x: b.position.x, y: min_y + 1 },
        _ => panic!("unexpected blizzard direction"),
      };
      next_blizzards.push(Blizzard { position: wrap_position, direction: b.direction.clone() })
    } else {
      next_blizzards.push(Blizzard { position: maybe_next_position, direction: b.direction.clone() })
    }
  }

  next_blizzards
}

fn get_next_position(blizzard: &Blizzard) -> Point {
  match blizzard.direction {
    '<' => blizzard.position + Point { x: -1, y: 0 },
    '>' => blizzard.position + Point { x: 1, y: 0 },
    '^' => blizzard.position + Point { x: 0, y: -1 },
    'v' => blizzard.position + Point { x: 0, y: 1 },
    _ => panic!("unexpected blizzard direction"),
  }
}


fn get_walls(grid: &HashMap<Point, char>) -> HashSet<Point> {
  grid
    .iter()
    .filter(|(_k, v)| v == &&'#')
    .map(|(k, _v)| k.clone())
    .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Blizzard {
  position: Point,
  direction: char,
}

fn get_blizzards(grid: &HashMap<Point, char>) -> Vec<Blizzard> {
  let blizzards = vec!['<', '>', '^', 'v'];
  grid
    .iter()
    .filter(|(_k, v)| blizzards.contains(v))
    .map(|(k, v)| Blizzard {
      position: k.clone(),
      direction: v.clone(),
    })
    .collect()
}

fn get_start_end(grid: &HashMap<Point, char>) -> (Point, Point) {
  let start = grid
    .iter()
    .filter(|(k, v)| k.y == 0 && v == &&'.')
    .map(|(k, _v)| k.clone())
    .collect::<Vec<Point>>()
    .get(0).unwrap().clone();
  let max_y = grid.keys().map(|k| k.y).max().unwrap();
  let end = grid
    .iter()
    .filter(|(k, v)| k.y == max_y && v == &&'.')
    .map(|(k, _v)| k.clone())
    .collect::<Vec<Point>>()
    .get(0).unwrap().clone();

  (start.clone(), end.clone())
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

pub fn part_two() -> i64 {
  0
}


#[cfg(test)]
mod tests {
  use crate::day12::lines_to_grid_char_val;
  use crate::day24::{get_blizzards, get_walls, index_all_blizzard_state_locations, solve_one};

  #[test]
  fn test_2_blizzard_environ() {
    let map_lines = vec![
      "#.#####".to_string(),
      "#...v.#".to_string(),
      "#..>..#".to_string(),
      "#.....#".to_string(),
      "#.....#".to_string(),
      "#.....#".to_string(),
      "#####.#".to_string(),
    ];

    let grid = lines_to_grid_char_val(&map_lines);
    let walls = get_walls(&grid);
    let blizzards = get_blizzards(&grid);
    let indexed_locations = index_all_blizzard_state_locations(&walls, &blizzards);
  }

  #[test]
  fn test_part_1() {
    let map_lines = vec![
      "#.######".to_string(),
      "#>>.<^<#".to_string(),
      "#.<..<<#".to_string(),
      "#>v.><>#".to_string(),
      "#<^v^^>#".to_string(),
      "######.#".to_string(),
    ];

    let grid = lines_to_grid_char_val(&map_lines);
    let result = solve_one(&grid);
    assert_eq!(result, 18)
  }
}
