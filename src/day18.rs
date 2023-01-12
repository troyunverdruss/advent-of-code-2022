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
  0
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
  let neighbors = vec![
    Point3 { x: -1, y: 0, z: 0 },
    Point3 { x: 1, y: 0, z: 0 },
    Point3 { x: 0, y: -1, z: 0 },
    Point3 { x: 0, y: 1, z: 0 },
    Point3 { x: 0, y: 0, z: -1 },
    Point3 { x: 0, y: 0, z: 1 },
  ];

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


#[cfg(test)]
mod tests {
  use crate::day18::{find_surface_area, parse_input};
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
