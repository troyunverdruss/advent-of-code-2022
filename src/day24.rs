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


pub fn part_two() -> u64 {
  let input = read_chunks("day24.txt", "\n");
  let grid = lines_to_grid_char_val(&input);
  solve_two(&grid)
}

fn solve_one(grid: &HashMap<Point, char>) -> u64 {
  let (start, end) = get_start_end(grid);
  let walls = get_walls(grid);
  let blizzards = get_blizzards(grid);
  let all_indexed_blizzard_state_locations = index_all_blizzard_state_locations(&walls, &blizzards);

  find_shortest_path(&start, &end, &walls, &all_indexed_blizzard_state_locations, 0) as u64
}


fn solve_two(grid: &HashMap<Point, char>) -> u64 {
  let (start, end) = get_start_end(grid);
  let walls = get_walls(grid);
  let blizzards = get_blizzards(grid);
  let all_indexed_blizzard_state_locations = index_all_blizzard_state_locations(&walls, &blizzards);

  let to_the_end = find_shortest_path(&start, &end, &walls, &all_indexed_blizzard_state_locations, 0);
  let back_to_beginning = find_shortest_path(&end, &start, &walls, &all_indexed_blizzard_state_locations, to_the_end);
  let and_the_end_again = find_shortest_path(&start, &end, &walls, &all_indexed_blizzard_state_locations, back_to_beginning);

  and_the_end_again as u64
}

fn find_shortest_path(
  start: &Point,
  end: &Point,
  walls: &HashSet<Point>,
  all_indexed_blizzard_state_locations: &HashMap<usize, HashSet<Point>>,
  starting_step: usize
) -> usize {

  let default_path_upper_bound = 1_000_000_000_000;
  let mut shortest_path: usize = default_path_upper_bound;

  let min_y = walls.iter().map(|k| k.y).min().unwrap();
  let max_y = walls.iter().map(|k| k.y).max().unwrap();

  let mut to_visit = VecDeque::new();
  to_visit.push_back(State { steps: starting_step, position: start.clone() });
  let mut visited = HashSet::new();

  while !to_visit.is_empty() {
    let curr = to_visit.pop_front().unwrap();

    // // Debug breakpoints
    // let test_point = Point { x: 4, y: 3 };
    // let test_step = 14;
    // if curr.position == test_point && curr.steps == test_step {
    //   let blizzards = all_indexed_blizzard_state_locations
    //     .get(&((curr.steps) % all_indexed_blizzard_state_locations.len())).unwrap();
    //
    //   println!("Minute: {}", test_step);
    //   dbg_print_grid(
    //     &walls,
    //     blizzards,
    //     &curr
    //   );
    //
    //   let maybe_blizzards = all_indexed_blizzard_state_locations
    //     .get(&((curr.steps + 1) % all_indexed_blizzard_state_locations.len())).unwrap();
    //   println!("maybe minute 15");
    //   dbg_print_grid(
    //     &walls,
    //     maybe_blizzards,
    //     &State { steps: 15, position: Point{x: 1, y: 0}, }
    //   )
    // }

    if curr.position == *end {
      shortest_path = min(shortest_path, curr.steps);
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
          let is_valid = !next_blizzard_positions.contains(p) && !walls.contains(p) && p.y >= min_y && p.y <= max_y;

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
  let width = max_x - min_x - 1;
  let height = max_y - min_y - 1;

  let mut next_blizzards = blizzards.clone();
  for i in 0..(height * width) {
    blizzard_states.insert(i as usize, next_blizzards.iter().map(|b| b.position.clone()).collect());
    let tmp_blizzards = step_blizzards(i, walls, &next_blizzards);
    next_blizzards = tmp_blizzards;
  }

  blizzard_states
}

fn step_blizzards(_index: i64, walls: &HashSet<Point>, curr_blizzards: &Vec<Blizzard>) -> Vec<Blizzard> {
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

  // println!("Minute {}", _index);
  // dbg_print_grid_with_blizzard_dirs(walls, &next_blizzards);
  // println!();

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
fn dbg_print_grid(walls: &HashSet<Point>, blizzards: &HashSet<Point>, x1: &State) {
  let mut grid = HashMap::new();

  walls.iter().for_each(|p| { grid.insert(p.clone(), '#'); });
  blizzards.iter().for_each(|p| { grid.insert(p.clone(), 'B'); });
  grid.insert(x1.position.clone(), 'E');

  let min_x = grid.keys().map(|k| k.x).min().unwrap();
  let max_x = grid.keys().map(|k| k.x).max().unwrap();
  let min_y = grid.keys().map(|k| k.y).min().unwrap();
  let max_y = grid.keys().map(|k| k.y).max().unwrap();

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      print!("{}", grid.get(&Point { x, y }).unwrap_or(&'.'));
    }
    println!();
  }
}

#[allow(dead_code)]
fn dbg_print_grid_with_blizzard_dirs(walls: &HashSet<Point>, blizzards: &Vec<Blizzard>) {
  let mut grid = HashMap::new();

  walls.iter().for_each(|p| { grid.insert(p.clone(), '#'); });
  blizzards.iter()
    .for_each(|b| {
      let existing = grid.get(&b.position).unwrap_or(&'.').clone();
      if existing != '.' {
        match existing.to_string().parse::<usize>() {
          Ok(num) => {
            grid.insert(b.position.clone(), (num + 1).to_string().chars().nth(0).unwrap());
          }
          Err(_blizzard) => {
            grid.insert(b.position.clone(), '2');
          }
        }


      } else {
        grid.insert(b.position.clone(), b.direction);
      }
    }
    );

  let min_x = grid.keys().map(|k| k.x).min().unwrap();
  let max_x = grid.keys().map(|k| k.x).max().unwrap();
  let min_y = grid.keys().map(|k| k.y).min().unwrap();
  let max_y = grid.keys().map(|k| k.y).max().unwrap();

  for y in min_y..=max_y {
    for x in min_x..=max_x {
      print!("{}", grid.get(&Point { x, y }).unwrap_or(&'.'));
    }
    println!();
  }
}

#[cfg(test)]
mod tests {
  use crate::day12::lines_to_grid_char_val;
  use crate::day24::{get_blizzards, get_walls, index_all_blizzard_state_locations, solve_one, solve_two};

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
    let map_lines = get_example_map();

    let grid = lines_to_grid_char_val(&map_lines);
    let result = solve_one(&grid);
    assert_eq!(result, 18)
  }

  #[test]
  fn test_part_2() {
    let map_lines = get_example_map();

    let grid = lines_to_grid_char_val(&map_lines);
    let result = solve_two(&grid);
    assert_eq!(result, 54)
  }

  fn get_example_map() -> Vec<String> {
    vec![
      "#.######".to_string(),
      "#>>.<^<#".to_string(),
      "#.<..<<#".to_string(),
      "#>v.><>#".to_string(),
      "#<^v^^>#".to_string(),
      "######.#".to_string(),
    ]
  }
}
