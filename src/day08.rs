use std::collections::{HashMap};
use std::ops::Add;
use crate::utils::read_chunks;

#[allow(dead_code)]
pub fn part_one() -> u64 {
  let lines = read_chunks("day08.txt", "\n");
  solve_one(&lines)
}

#[allow(dead_code)]
pub fn part_two() -> u64 {
  let lines = read_chunks("day08.txt", "\n");
  solve_two(&lines)
}

#[allow(dead_code)]
fn lines_to_grid_number_val(lines: &Vec<String>) -> HashMap<Point, u64> {
  let mut grid = HashMap::new();

  lines
    .iter()
    .enumerate()
    .for_each(|(y, line)| {
      line
        .chars()
        .enumerate()
        .for_each(|(x, tree)| {
          let tree_height: u64 = tree.to_string().parse().expect("Should be an int");
          grid.insert(Point { x: x as i64, y: y as i64 }, tree_height);
        })
    });

  grid
}

#[derive(PartialOrd, PartialEq, Hash, Debug, Eq, Ord, Clone, Copy)]
pub struct Point {
  pub(crate) x: i64,
  pub(crate) y: i64,
}

impl Add for Point {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Point {x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

#[allow(dead_code)]
fn solve_one(lines: &Vec<String>) -> u64 {
  let grid = lines_to_grid_number_val(&lines);

  get_visible_locations(&grid)
    .iter()
    .count() as u64
}

#[allow(dead_code)]
fn is_visible(grid: &HashMap<Point, u64>, loc: &Point, height: &u64) -> bool {
  let higher_trees_left = grid
    .iter()
    .filter(|(k, v)| k.y == loc.y && k.x < loc.x && v.to_owned() >= height)
    .count();
  let higher_trees_right = grid
    .iter()
    .filter(|(k, v)| k.y == loc.y && k.x > loc.x && v.to_owned() >= height)
    .count();
  let higher_trees_up = grid
    .iter()
    .filter(|(k, v)| k.x == loc.x && k.y < loc.y && v.to_owned() >= height)
    .count();
  let higher_trees_down = grid
    .iter()
    .filter(|(k, v)| k.x == loc.x && k.y > loc.y && v.to_owned() >= height)
    .count();

  higher_trees_left == 0 || higher_trees_right == 0 || higher_trees_up == 0 || higher_trees_down == 0
}

#[allow(dead_code)]
fn get_visible_locations(grid: &HashMap<Point, u64>) -> Vec<Point> {
  grid
    .iter()
    .filter(|(loc, height)| {
      let visible = is_visible(&grid, loc, height);
      // if !visible { println!("Not visible: {:?}", loc) };
      visible
    })
    .map(|(loc, _)| loc.to_owned())
    .collect()
}

#[allow(dead_code)]
fn solve_two(lines: &Vec<String>) -> u64 {
  let grid = lines_to_grid_number_val(&lines);
  grid
    .keys()
    .map(|loc| compute_scenic_score(&grid, loc))
    .max()
    .unwrap()
}

#[allow(dead_code)]
fn compute_scenic_score(grid: &HashMap<Point, u64>, location: &Point) -> u64 {
  let min_x = 0;
  let min_y = 0;
  let max_x = grid.keys().max_by_key(|k| k.x).unwrap().to_owned().x;
  let max_y = grid.keys().max_by_key(|k| k.y).unwrap().to_owned().y;

  let current_tree_height = grid.get(location).unwrap();

  let mut visible_left = 0;
 if location.x > 0 {
  for x in (min_x..=(location.x - 1)).rev() {
    if grid.get(&Point { x, y: location.y }).unwrap() < current_tree_height {
      visible_left += 1;
    } else {
      visible_left += 1;
      break;
    }
  };
}

  let mut visible_right = 0;
  if location.x < max_x {
    for x in (location.x + 1)..=max_x {
      if grid.get(&Point {x, y: location.y}).unwrap() < current_tree_height {
        visible_right += 1;
      } else {
        visible_right += 1;
        break;
      }
    };
  }

    let mut visible_up = 0;
  if location.y > 0 {
    for y in (min_y..=(location.y - 1)).rev() {
      if grid.get(&Point { x: location.x, y }).unwrap() < current_tree_height {
        visible_up += 1;
      } else {
        visible_up += 1;
        break;
      }
    };
  }

  let mut visible_down = 0;
  if location.y < max_y {
    for y in (location.y + 1)..=max_y {
      if grid.get(&Point { x: location.x, y }).unwrap() < current_tree_height {
        visible_down += 1;
      } else {
        visible_down += 1;
        break;
      }
    };
  }

  visible_left * visible_right * visible_up * visible_down
}

#[cfg(test)]
mod tests {
  use crate::day08::{compute_scenic_score, get_visible_locations, lines_to_grid_number_val, Point, solve_one, solve_two};

  #[test]
  fn test_solving_part_1() {
    let inputs = get_inputs();
    let result = solve_one(&inputs);

    assert_eq!(result, 21);
  }

  #[test]
  fn test_simple_one_line() {
    let result = solve_one(&vec!["30373".to_owned()]);

    assert_eq!(result, 5);
  }

  #[test]
  fn test_simple_column() {
    let inputs: Vec<String> = vec![
      "3",
      "2",
      "6",
      "3",
      "3",
    ]
      .iter()
      .map(|l| l.to_string())
      .collect();
    let result = solve_one(&inputs);

    assert_eq!(result, 5);
  }

  #[test]
  fn test_verify_center_square() {
    let inputs = get_inputs();
    let grid = lines_to_grid_number_val(&inputs);
    let visible = get_visible_locations(&grid);

    assert_eq!(visible.iter().filter(|p| p.y == 0).count(), 5);

    assert_eq!(visible.iter().filter(|p| p.y == 1).count(), 4);

    assert_eq!(visible.iter().filter(|p| p.y == 2).count(), 4);

    assert_eq!(visible.iter().filter(|p| p.y == 3).count(), 3);

    assert_eq!(visible.iter().filter(|p| p.y == 4).count(), 5);


    assert!(visible.contains(&Point { x: 1, y: 1 }));
    assert!(visible.contains(&Point { x: 2, y: 1 }));
    assert!(!visible.contains(&Point { x: 3, y: 1 }));

    assert!(visible.contains(&Point { x: 1, y: 2 }));
    assert!(!visible.contains(&Point { x: 2, y: 2 }));
    assert!(visible.contains(&Point { x: 3, y: 2 }));

    assert!(!visible.contains(&Point { x: 1, y: 3 }));
    assert!(visible.contains(&Point { x: 2, y: 3 }));
    assert!(!visible.contains(&Point { x: 3, y: 3 }));
  }

  #[test]
  fn test_scenic_score_1() {
    let inputs = get_inputs();
    let grid = lines_to_grid_number_val(&inputs);
    let result = compute_scenic_score(&grid, &Point{x: 2, y: 1});

    assert_eq!(result, 4);
  }

  #[test]
  fn test_scenic_score_2() {
    let inputs = get_inputs();
    let grid = lines_to_grid_number_val(&inputs);
    let result = compute_scenic_score(&grid, &Point{x: 2, y: 3});

    assert_eq!(result, 8);
  }
  #[test]
  fn test_solving_part_2() {
    let inputs = get_inputs();
    let result = solve_two(&inputs);

    assert_eq!(result, 8);
  }


  fn get_inputs() -> Vec<String> {
    let inputs: Vec<String> = vec![
      "30373",
      "25512",
      "65332",
      "33549",
      "35390",
    ]
      .iter()
      .map(|l| l.to_string())
      .collect();
    inputs
  }
}
