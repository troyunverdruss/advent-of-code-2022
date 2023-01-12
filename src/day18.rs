use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::str::Split;

use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day18.txt", "\n");
  let points = parse_input(&lines);
  find_surface_area(&points)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day18.txt", "\n");
  let points = parse_input(&lines);

  solve_part_2(&points)
}

fn solve_part_2(points: &Vec<Point3>) -> u64 {
  let empty_fill_grid = fill_grid_with_surrounding_cube(&points);
  let filled_grid = fill_grid_with_points(&points, &empty_fill_grid);
  let cleared_exterior_grid = clear_exterior_grid_points(&filled_grid);
  let interior_surface_area = compute_surface_area_of_interior_space(&cleared_exterior_grid);
  let exterior_surface_area = find_surface_area(&points) - interior_surface_area;

  exterior_surface_area
}

fn compute_surface_area_of_interior_space(cleared_exterior_grid: &HashMap<Point3, char>) -> u64 {
  let interior_cubes = cleared_exterior_grid
    .iter()
    .filter(|(_, v)| **v == '.')
    .map(|(k, _)| k.clone())
    .collect::<Vec<Point3>>();

  find_surface_area(&interior_cubes)
}

fn clear_exterior_grid_points(filled_grid: &HashMap<Point3, char>) -> HashMap<Point3, char> {
  let min_x = filled_grid.keys().map(|k| k.x).min().unwrap();
  let min_y = filled_grid.keys().map(|k| k.y).min().unwrap();
  let min_z = filled_grid.keys().map(|k| k.z).min().unwrap();
  let search_start = Point3 { x: min_x, y: min_y, z: min_z };

  let mut cleared_grid = filled_grid.clone();
  cleared_grid.insert(search_start.clone(), ' ');

  let mut to_visit: VecDeque<Point3> = VecDeque::new();
  let mut visited: HashSet<Point3> = HashSet::new();

  to_visit.push_back(search_start.clone());

  while !to_visit.is_empty() {
    let current = to_visit.pop_front().unwrap();

    if visited.contains(&current) {
      continue;
    }

    for n in get_3d_neighbors(&current) {
      if visited.contains(&n) {
        continue;
      }

      let curr_value = cleared_grid.get(&n);
      if curr_value.is_none() {
        continue
      }
      if curr_value.unwrap() != &'#' {
        cleared_grid.insert(n.clone(), ' ');
        to_visit.push_back(n);
      }

      visited.insert(current);
    }
  }

  cleared_grid
}

fn fill_grid_with_points(points: &Vec<Point3>, empty_fill_grid: &HashMap<Point3, char>) -> HashMap<Point3, char> {
  let mut filled_grid = empty_fill_grid.clone();
  points
    .iter()
    .for_each(|p| {
      filled_grid.insert(*p, '#');
    });

  filled_grid
}

fn fill_grid_with_surrounding_cube(points: &Vec<Point3>) -> HashMap<Point3, char> {
  let min_x = points.iter().map(|p| p.x).min().unwrap() - 1;
  let max_x = points.iter().map(|p| p.x).max().unwrap() + 1;
  let min_y = points.iter().map(|p| p.y).min().unwrap() - 1;
  let max_y = points.iter().map(|p| p.y).max().unwrap() + 1;
  let min_z = points.iter().map(|p| p.z).min().unwrap() - 1;
  let max_z = points.iter().map(|p| p.z).max().unwrap() + 1;

  let mut grid = HashMap::new();
  for x in min_x..=max_x {
    for y in min_y..=max_y {
      for z in min_z..=max_z {
        grid.insert(Point3 {x,y,z}, '.');
      }
    }
  }

  grid
}


fn parse_input(lines: &Vec<String>) -> Vec<Point3> {
  lines
    .iter()
    .map(|l| l.split(","))
    .map(|split| split_to_point3(split))
    .collect()
}

fn split_to_point3(split: Split<&str>) -> Point3 {
  let collected = split.collect::<Vec<&str>>();
  Point3 {
    x: collected[0].to_string().parse().unwrap(),
    y: collected[1].to_string().parse().unwrap(),
    z: collected[2].to_string().parse().unwrap(),
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point3 {
  x: i64,
  y: i64,
  z: i64,
}

impl Add for Point3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Point3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
  }
}

fn find_surface_area(points: &Vec<Point3>) -> u64 {
  let neighbors = get_3d_neighbors_modifiers();

  let mut processed: HashSet<Point3> = HashSet::new();
  let mut surface_area = 0;

  for point in points {
    // Every cube adds 6 surface area, then we have to subtract away any overlap
    surface_area += 6;
    for n in &neighbors {
      let test_point = *point + *n;
      if processed.contains(&test_point) {
        // Remove 2 sides for any conjoining cubes
        surface_area -= 2;
      }
    }
    processed.insert(*point);
  }

  surface_area
}

fn get_3d_neighbors(point: &Point3) -> Vec<Point3> {
  get_3d_neighbors_modifiers()
    .iter()
    .map(|n| *point + *n)
    .collect()
}

fn get_3d_neighbors_modifiers() -> Vec<Point3> {
  vec![
    Point3 { x: -1, y: 0, z: 0 },
    Point3 { x: 1, y: 0, z: 0 },
    Point3 { x: 0, y: -1, z: 0 },
    Point3 { x: 0, y: 1, z: 0 },
    Point3 { x: 0, y: 0, z: -1 },
    Point3 { x: 0, y: 0, z: 1 },
  ]
}


#[cfg(test)]
mod tests {
  use crate::day18::{find_surface_area, parse_input, solve_part_2};
  use crate::utils::read_chunks;

  #[test]
  fn test_simple_input() {
    let input = get_simple_input();
    let points = parse_input(&input);
    let surface_area = find_surface_area(&points);
    assert_eq!(surface_area, 10);
  }

  #[test]
  fn test_part_1() {
    let input = get_part_1_sample_input();
    let points = parse_input(&input);
    let surface_area = find_surface_area(&points);
    assert_eq!(surface_area, 64);
  }


  #[test]
  fn test_part_2() {
    let input = get_part_1_sample_input();
    let points = parse_input(&input);
    assert_eq!(solve_part_2(&points), 58);
  }

  fn get_simple_input() -> Vec<String> {
    vec![
      "1,1,1".to_string(),
      "2,1,1".to_string(),
    ]
  }

  fn get_part_1_sample_input() -> Vec<String> {
    vec![
      "2,2,2".to_string(),
      "1,2,2".to_string(),
      "3,2,2".to_string(),
      "2,1,2".to_string(),
      "2,3,2".to_string(),
      "2,2,1".to_string(),
      "2,2,3".to_string(),
      "2,2,4".to_string(),
      "2,2,6".to_string(),
      "1,2,5".to_string(),
      "3,2,5".to_string(),
      "2,1,5".to_string(),
      "2,3,5".to_string(),
    ]
  }
}
