use std::collections::{HashMap, HashSet};
use crate::day08::Point;
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day09.txt", "\n");
  solve_one(&lines, 2)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day09.txt", "\n");
  solve_one(&lines, 10)
}

fn solve_one(lines: &Vec<String>, knot_count: u64) -> u64 {
  let mut visited: HashSet<Point> = HashSet::new();

  let mut knots = vec![];
  for _ in 0..knot_count {
    knots.push(Point { x: 0, y: 0 });
  }
  visited.insert(Point { x: 0, y: 0 });

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
        // Update the head first
        knots[0] = knots[0] + dirs[&dir];

        // Now move each following knot approriately
        for k in 1..knots.len() {
          let prev_knot = knots[k - 1];

          if prev_knot.x == knots[k].x && distance(prev_knot, knots[k]) > 1 {
            while distance(prev_knot, knots[k]) > 1 {
              if prev_knot.y > knots[k].y {
                knots[k] = knots[k] + dirs[&'D'];
              } else {
                knots[k] = knots[k] + dirs[&'U'];
              }
            }
          } else if prev_knot.y == knots[k].y && distance(prev_knot, knots[k]) > 1 {
            while distance(prev_knot, knots[k]) > 1 {
              if prev_knot.x > knots[k].x {
                knots[k] = knots[k] + dirs[&'R'];
              } else {
                knots[k] = knots[k] + dirs[&'L'];
              }
            }
          } else if distance(prev_knot, knots[k]) > 2 {
            while distance(prev_knot, knots[k]) > 2 {
              // We're at a diagonal so need to move tail diagonally
              if prev_knot.x > knots[k].x {
                knots[k] = knots[k] + dirs[&'R'];
              } else {
                knots[k] = knots[k] + dirs[&'L'];
              }
              if prev_knot.y > knots[k].y {
                knots[k] = knots[k] + dirs[&'D'];
              } else {
                knots[k] = knots[k] + dirs[&'U'];
              }
            }
          } else {
            // Nothing to do since it's close enough
          }
        }

        visited.insert(knots[knots.len() - 1].clone());
      }
    });

  visited.iter().count() as u64
}

pub fn distance(a: Point, b: Point) -> i64 {
  (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[cfg(test)]
mod tests {
  use crate::day09::solve_one;

  #[test]
  fn test_sample_1() {
    let inputs = get_small_example();
    let result = solve_one(&inputs, 2);
    assert_eq!(result, 13);
  }

  #[test]
  fn test_small_sample_2() {
    let inputs = get_small_example();
    let result = solve_one(&inputs, 10);
    assert_eq!(result, 1);
  }

  #[test]
  fn test_large_sample_2() {
    let inputs = get_large_example();
    let result = solve_one(&inputs, 10);
    assert_eq!(result, 36);
  }


  fn get_small_example() -> Vec<String> {
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

  fn get_large_example() -> Vec<String> {
    let inputs: Vec<String> = vec![
      "R 5",
      "U 8",
      "L 8",
      "D 3",
      "R 17",
      "D 10",
      "L 25",
      "U 20",
    ]
      .iter()
      .map(|l| l.to_string())
      .collect();
    inputs
  }
}
