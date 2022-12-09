use std::collections::{HashMap};
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day08.txt", "\n");
  solve_one(&lines)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day08.txt", "\n");
  solve_two(&lines)
}

fn lines_to_grid(lines: &Vec<String>) -> HashMap<Point, u64> {
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
          grid.insert(Point { x: x as u64, y: y as u64 }, tree_height);
        })
    });

  grid
}

#[derive(PartialOrd, PartialEq, Hash, Debug, Eq, Ord, Clone, Copy)]
struct Point {
  x: u64,
  y: u64,
}

fn solve_one(lines: &Vec<String>) -> u64 {
  let grid = lines_to_grid(&lines);

  get_visible_locations(&grid)
    .iter()
    .count() as u64
}

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

  let test = Point{ x: 3, y: 1 };
  if loc == &test {
    let x = 0;
  }

  higher_trees_left == 0 || higher_trees_right == 0 || higher_trees_up == 0 || higher_trees_down == 0
}

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


fn solve_two(lines: &Vec<String>) -> u64 {
  let grid = lines_to_grid(&lines);
  0
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;
  use crate::day08::{get_visible_locations, lines_to_grid, Point, solve_one, solve_two};

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
    let grid = lines_to_grid(&inputs);
    let visible = get_visible_locations(&grid);

    let visible_hs: HashSet<Point> = HashSet::from_iter(visible.clone());
    let x = 0;

    assert_eq!(visible.iter().filter(|p| p.y == 0).count(), 5);

    let row_2: Vec<Point> = visible.iter().filter(|p| p.y == 1).map(|p| p.clone()).collect();
    assert_eq!(visible.iter().filter(|p| p.y == 1).count(), 4);

    let row_3: Vec<Point> = visible.iter().filter(|p| p.y == 2).map(|p| p.clone()).collect();
    assert_eq!(visible.iter().filter(|p| p.y == 2).count(), 4);

    let row_4: Vec<Point> = visible.iter().filter(|p| p.y == 4).map(|p| p.clone()).collect();
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