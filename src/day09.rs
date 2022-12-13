use std::collections::{HashMap, HashSet};
use crate::day08::Point;
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day09.txt", "\n");
  solve_one(&lines)
}

pub fn part_two() -> u64 {
  // let lines = read_chunks("day09.txt", "\n");
  // solve_two(&lines)
  0
}

fn solve_one(lines: &Vec<String>) -> u64 {
  let mut visited: HashSet<Point> = HashSet::new();
  let mut head = Point { x: 0, y: 0 };
  let mut tail = Point { x: 0, y: 0 };
  visited.insert(tail.clone());

  let dirs: HashMap<char, Point> = HashMap::from_iter(
    vec![
      ('U', Point { x: 0, y: -1 }),
      ('D', Point { x: 0, y: 1 }),
      ('L', Point { x: -1, y: 0 }),
      ('R', Point { x: 1, y: 0 }),
    ]
  );

  lines
    .iter()
    .for_each(|l| {
      let dir = l.chars().nth(0).unwrap();
      let count = l
        .split(' ')
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .get(1)
        .unwrap()
        .parse()
        .unwrap();

      for _ in 0..count {
        head = head + dirs[&dir];

        // move tail
        if head.x == tail.x && distance(head, tail) > 1 {
          if head.y > tail.y {
            tail = tail + dirs[&'D'];
          } else {
            tail = tail + dirs[&'U'];
          }
        } else if head.y == tail.y && distance(head, tail) > 1 {
          if head.x > tail.x {
            tail = tail + dirs[&'R'];
          } else {
            tail = tail + dirs[&'L'];
          }
        } else if distance(head, tail) > 2 {
          // We're at a diagonal so need to move tail diagonally
          if head.x > tail.x {
            tail = tail + dirs[&'R'];
          } else {
            tail = tail + dirs[&'L'];
          }
          if head.y > tail.y {
            tail = tail + dirs[&'D'];
          } else {
            tail = tail + dirs[&'U'];
          }
        } else {
          // Nothing to do since it's close enough
        }
        visited.insert(tail.clone());
      }
    });

  visited.iter().count() as u64
}

fn distance(a: Point, b: Point) -> i64 {
  (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[cfg(test)]
mod tests {
  use crate::day09::solve_one;

  #[test]
  fn test_sample_1() {
    let inputs = get_inputs();
    let result = solve_one(&inputs);
    assert_eq!(result, 13);
  }

  fn get_inputs() -> Vec<String> {
    let inputs: Vec<String> = vec![
      "R 4",
      "U 4",
      "L 3",
      "D 1",
      "R 4",
      "D 1",
      "L 5",
      "R 2",
    ]
      .iter()
      .map(|l| l.to_string())
      .collect();
    inputs
  }
}
