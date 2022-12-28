use std::any::Any;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

use serde_json::{Number, Result, Value};

use crate::day08::Point;
use crate::day13::NestableItemType::{INT, VEC};
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let chunks = read_chunks("day13.txt", "\n\n");
  solve_one(&chunks)
}

fn solve_one(chunks: &Vec<String>) -> u64 {
  let index_to_in_order: HashMap<usize, CompareResult> = chunks
    .iter()
    .map(|c| c.split("\n").collect::<Vec<&str>>())
    .enumerate()
    .map(|p| {
      (
        p.0,
        is_in_order(
          &serde_json::from_str(p.1[0]).unwrap(),
          &serde_json::from_str(p.1[1]).unwrap(),
        )
      )
    })
    .collect();

  let sum: usize = index_to_in_order
    .iter()
    .filter(|p| (*p).1 == &CompareResult::InOrder)
    .map(|p| *p.0 + 1)// Indexes are 1-based!!
    .sum();

  sum as u64
}

#[derive(PartialEq, Eq, Debug)]
enum CompareResult {
  InOrder,
  NotInOrder,
  Equal,
}

fn is_in_order(left: &Value, right: &Value) -> CompareResult {
  match left {
    Value::Number(n) => { return handle_left_number(n, right); }
    Value::Array(a) => { return handle_left_array(a, right); }
    _ => panic!("Unexpected value type")
  }
}

fn handle_left_array(left_array: &Vec<Value>, right: &Value) -> CompareResult {
  match right {
    Value::Number(_) => { handle_two_array(left_array, &vec![right.to_owned()]) }
    Value::Array(a) => { handle_two_array(left_array, a) }
    _ => panic!("Unexpected value type")
  }
}

fn handle_two_array(left_array: &Vec<Value>, right_array: &Vec<Value>) -> CompareResult {
  let max_len = max(left_array.len(), right_array.len());

  for i in 0..max_len {
    let next_left = left_array.get(i);
    let next_right = right_array.get(i);

    if next_left.is_none() && next_right.is_none() {
      return CompareResult::Equal;
    } else if next_left.is_none() && next_right.is_some() {
      return CompareResult::InOrder;
    } else if next_left.is_some() && next_right.is_none() {
      return CompareResult::NotInOrder;
    } else {
      let left = next_left.unwrap();
      let right = next_right.unwrap();

      let result = match left {
        Value::Number(n) => { handle_left_number(n, right) }
        Value::Array(a) => { handle_left_array(a, right) }
        _ => panic!("Unexpected value type")
      };

      if result != CompareResult::Equal {
        return result;
      }
    }
  }

  return CompareResult::Equal;
}

fn handle_left_number(left_number: &Number, right: &Value) -> CompareResult {
  match right {
    Value::Number(n) => {
      if left_number.as_i64() < n.as_i64() {
        CompareResult::InOrder
      } else if left_number.as_i64() > n.as_i64() {
        CompareResult::NotInOrder
      } else {
        CompareResult::Equal
      }
    }
    Value::Array(a) => { handle_two_array(&vec![Value::Number(left_number.to_owned())], a) }
    _ => panic!("Unexpected value type")
  }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum NestableItemType {
  INT,
  VEC,
}

#[derive(Clone)]
struct NestableItem {
  int: Option<u64>,
  vec: Option<Vec<NestableItem>>,
  item_type: NestableItemType,
}

impl NestableItem {
  fn get_item_type(self: &Self) -> NestableItemType {
    self.item_type.to_owned()
  }

  fn get_int(self: &Self) -> u64 {
    assert_eq!(self.item_type, INT);
    self.int.expect("There should have been an int value here")
  }

  fn get_vec(self: &Self) -> Vec<NestableItem> {
    assert_eq!(self.item_type, VEC);
    self.vec.to_owned().expect("There should have been a vec value here")
  }
}

pub fn part_two() -> u64 {
  // let lines = read_chunks("day09.txt", "\n");
  // solve_two(&lines)
  0
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
  use std::result;
  use crate::day13::{CompareResult, is_in_order, solve_one};

  #[test]
  fn test_pair_1() {
    let left = serde_json::from_str("[1,1,3,1,1]").unwrap();
    let right = serde_json::from_str("[1,1,5,1,1]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::InOrder
    );
  }

  #[test]
  fn test_pair_2() {
    let left = serde_json::from_str("[[1],[2,3,4]]").unwrap();
    let right = serde_json::from_str("[[1],4]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::InOrder
    );
  }

  #[test]
  fn test_pair_3() {
    let left = serde_json::from_str("[9]").unwrap();
    let right = serde_json::from_str("[[8,7,6]]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::NotInOrder
    );
  }

  #[test]
  fn test_pair_4() {
    let left = serde_json::from_str("[[4,4],4,4]").unwrap();
    let right = serde_json::from_str("[[4,4],4,4,4]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::InOrder
    );
  }

  #[test]
  fn test_pair_5() {
    let left = serde_json::from_str("[7,7,7,7]").unwrap();
    let right = serde_json::from_str("[7,7,7]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::NotInOrder
    );
  }

  #[test]
  fn test_pair_6() {
    let left = serde_json::from_str("[]").unwrap();
    let right = serde_json::from_str("[3]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::InOrder
    );
  }

  #[test]
  fn test_pair_7() {
    let left = serde_json::from_str("[[[]]]").unwrap();
    let right = serde_json::from_str("[[]]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::NotInOrder
    );
  }

  #[test]
  fn test_pair_8() {
    let left = serde_json::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
    let right = serde_json::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();

    assert_eq!(
      is_in_order(&left, &right),
      CompareResult::NotInOrder
    );
  }

  #[test]
  fn test_example_1() {
    let chunks = get_input();
    let result = solve_one(&chunks);
    assert_eq!(result, 13);
  }

  fn get_input() -> Vec<String> {
    let raw_input =
      "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    raw_input.split("\n\n")
      .map(|l| l.to_string())
      .collect::<Vec<String>>()
  }
}
