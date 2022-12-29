use std::collections::{HashMap, HashSet, VecDeque};
use crate::day08::Point;
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day12.txt", "\n");
  let grid = lines_to_grid_char_val(&lines);
  let start = find_point_for_value(&grid, 'S');
  let dest = find_point_for_value(&grid, 'E');
  let grid = convert_grid_values_to_numeric(&grid);

  solve_one(&grid, &start, &dest)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day12.txt", "\n");
  let grid = lines_to_grid_char_val(&lines);
  let dest = find_point_for_value(&grid, 'E');
  let grid = convert_grid_values_to_numeric(&grid);

  solve_two(&grid, &dest)
}

fn solve_one(grid: &HashMap<Point, u64>, start: &Point, dest: &Point) -> u64 {
  let neighbors = vec![
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
  ];
  let mut to_visit: VecDeque<Point> = VecDeque::new();
  let mut visited: HashSet<Point> = HashSet::new();
  let mut distance: HashMap<Point, u64> = HashMap::new();

  to_visit.push_back(start.to_owned());
  distance.insert(start.to_owned(), 0);

  while !to_visit.is_empty() {
    let current = to_visit.pop_front().unwrap();
    let current_distance = *distance.get(&current).unwrap();
    if current == *dest {
      return *distance.get(&current).unwrap();
    }

    if visited.contains(&current) {
      continue;
    }
    for n in &neighbors {
      let possible = current + *n;
      if visited.contains(&possible) || to_visit.contains(&&possible) {
        continue;
      }
      if !move_allowed(grid, &current, &possible) {
        continue;
      }

      distance.insert(possible.to_owned(), current_distance + 1);
      to_visit.push_back(possible);
      visited.insert(current);
    }
  }

  // Failsafe here for part 2 to be longer than any possible route could be in
  // case there's a start that isn't "solvable" without doubling back over the path
  (grid.len() + 1) as u64
}

fn solve_two(grid: &HashMap<Point, u64>, dest: &Point) -> u64 {
  grid
    .iter()
    .filter(|kv| *(*kv).1 == 1)
    .map(|kv| solve_one(grid, kv.0, dest))
    .min()
    .expect("Really should have found at least one path!")
}

fn move_allowed(grid: &HashMap<Point, u64>, point_at: &Point, point_next: &Point) -> bool {
  if !grid.contains_key(point_next) {
    return false;
  }

  let at = grid.get(point_at).unwrap();
  let next = grid.get(point_next).unwrap();

  (*at as i64 - *next as i64) >= -1
}

fn convert_grid_values_to_numeric(grid: &HashMap<Point, char>) -> HashMap<Point, u64> {
  grid
    .iter()
    .map(|t| (t.0.clone(), score_for_letter(t.1)))
    .collect()
}

fn score_for_letter(letter: &char) -> u64 {
  if letter == &'E' {
    26
  } else if letter == &'S' {
    1
  } else {
    *letter as u64 - 96
  }
}

fn find_point_for_value(grid: &HashMap<Point, char>, target: char) -> Point {
  grid
    .iter()
    .filter(|t| (*t).1 == &target)
    .nth(0).expect("Should have found the target, typo?").0
    .clone()
}



fn lines_to_grid_char_val(lines: &Vec<String>) -> HashMap<Point, char> {
  let mut grid = HashMap::new();

  lines
    .iter()
    .enumerate()
    .for_each(|(y, line)| {
      line
        .chars()
        .enumerate()
        .for_each(|(x, c)| {
          grid.insert(Point { x: x as i64, y: y as i64 }, c);
        })
    });

  grid
}


#[cfg(test)]
mod tests {
  use crate::day08::Point;
  use crate::day12::{convert_grid_values_to_numeric, find_point_for_value, lines_to_grid_char_val, move_allowed, score_for_letter, solve_one, solve_two};

  #[test]
  fn test_scoring_for_grid() {
    assert_eq!(score_for_letter(&'a'), 1);
    assert_eq!(score_for_letter(&'z'), 26);
    assert_eq!(score_for_letter(&'a'), score_for_letter(&'S'));
    assert_eq!(score_for_letter(&'z'), score_for_letter(&'E'));
  }

  #[test]
  fn test_moves_allowed_correctly() {
    let lines = get_input();
    let grid = lines_to_grid_char_val(&lines);
    let grid = convert_grid_values_to_numeric(&grid);
    assert!(move_allowed(&grid, &Point { x: 0, y: 0 }, &Point { x: 1, y: 0 }));
    assert!(move_allowed(&grid, &Point { x: 1, y: 0 }, &Point { x: 2, y: 0 }));
    assert!(!move_allowed(&grid, &Point { x: 2, y: 0 }, &Point { x: 3, y: 0 }));
    assert!(move_allowed(&grid, &Point { x: 2, y: 0 }, &Point { x: 2, y: 1 }));
    assert!(move_allowed(&grid, &Point { x: 2, y: 0 }, &Point { x: 2, y: 1 }));
    assert!(move_allowed(&grid, &Point { x: 2, y: 1 }, &Point { x: 1, y: 1 }));

    assert!(move_allowed(&grid, &Point { x: 4, y: 2 }, &Point { x: 3, y: 2 }));
  }

  #[test]
  fn test_sample_1() {
    let lines = get_input();
    let grid = lines_to_grid_char_val(&lines);
    let start = find_point_for_value(&grid, 'S');
    let dest = find_point_for_value(&grid, 'E');
    let grid = convert_grid_values_to_numeric(&grid);

    let result = solve_one(&grid, &start, &dest);
    assert_eq!(result, 31);
  }

  #[test]
  fn test_sample_2() {
    let lines = get_input();
    let grid = lines_to_grid_char_val(&lines);
    let start = find_point_for_value(&grid, 'S');
    let dest = find_point_for_value(&grid, 'E');
    let grid = convert_grid_values_to_numeric(&grid);

    let result = solve_two(&grid, &dest);
    assert_eq!(result, 29);
  }

  fn get_input() -> Vec<String> {
    vec![
      "Sabqponm".to_owned(),
      "abcryxxl".to_owned(),
      "accszExk".to_owned(),
      "acctuvwj".to_owned(),
      "abdefghi".to_owned(),
    ]
  }
}
