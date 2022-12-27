use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::day08::Point;
use crate::day13::NestableItemType::{INT, VEC};
use crate::utils::read_chunks;
use serde_json::{Number, Result, Value};
use crate::day13::CompareResult::NotImplemented;

pub fn part_one() -> u64 {
  let chunks = read_chunks("day13.txt", "\n\n");
  solve_one(&chunks)
}

fn solve_one(chunks: &Vec<String>) -> u64 {
  let index_to_in_order: HashMap<usize, bool> = chunks
    .iter()
    .map(|c| c.split("\n").collect::<Vec<&str>>())
    .enumerate()
    .map(|p| (p.0, is_in_order(&p.1)))
    .collect();

  let sum: usize = index_to_in_order
    .iter()
    .filter(|p| (*p).1 == &true)
    .map(|p| *p.0 + 1)
    .sum();

  sum as u64
}

enum CompareResult {
  InOrder,
  NotInOrder,
  Equal,
}

fn is_in_order(left: &Value, right: &Value) -> CompareResult {
  match left {
    Value::Number(n) => { handle_left_number(n, right); }
    Value::Array(a) => { handle_left_array(a, right); }
    _ => panic!("Unexpected value type")
  };

  todo!()
}

fn handle_left_array(left_array: &Vec<Value>, right: &Value) -> CompareResult {
  match right {
    Value::Number(n) => {handle_two_array(left_array, &vec![right.to_owned()])}
    Value::Array(a) => {}
    _ => panic!("Unexpected value type")
  }
}

fn handle_two_array(left_array: &Vec<Value>, right_array: &Vec<Value>) -> CompareResult {
  let left_iter = left_array.iter();
  let right_iter = right_array.iter();
  
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

fn is_in_order(pairs: &Vec<&str>) -> bool {
  assert_eq!(pairs.len(), 2);
  let left = pairs.get(0).unwrap();
  let right = pairs.get(1).unwrap();

  let left1: Value = serde_json::from_str(left).unwrap();
match left1 {
  Value::Number(_) => {}
  Value::Array(xy) => {
    let hi = xy.get(0);
    let hi2 = 0;
  }
  _ => panic!("Unexpected input type")
}
  // let x = &left1[0];


  // If both values are integers, the lower integer should come first.
  // If the left integer is lower than the right integer, the inputs
  // are in the right order. If the left integer is higher than
  // the right integer, the inputs are not in the right order.
  // Otherwise, the inputs are the same integer; continue checking the next part of the input.
  if &left[0..1] != "[" && &right[0..1] != "[" {
    if left[0..1] < right[0..1] {
      return true;
    } else if left[0..1] > right[0..1] {
      return false;
    } else {
      // they're the same, continue
    }
  }
  // If both values are lists, compare the first value of each list,
  // then the second value, and so on. If the left list runs out of
  // items first, the inputs are in the right order. If the right
  // list runs out of items first, the inputs are not in the right
  // order. If the lists are the same length and no comparison makes a
  // decision about the order, continue checking the next part of the input.
  if &left[0..1] == "[" && &right[0..1] == "[" {}
  // If exactly one value is an integer, convert the integer to a list
  // which contains that integer as its only value, then retry the comparison.
  // For example, if comparing [0,0,0] and 2, convert the right value
  // to [2] (a list containing 2); the result is then found by
  // instead comparing [0,0,0] and [2].
  false
}

// fn recursive_parse(input: &str) -> NestableItem {
//   if &input[0..1] == "[" && &input[(input.len()-1)..input.len()] == "]" {
//     if input.len() == 2 {
//       return NestableItem {
//         int: None,
//         vec: Some(Vec::new()),
//         item_type: VEC
//       };
//     }
//     let nested_item = recursive_parse(&input[1..(input.len()-1)]);
//     return NestableItem {
//       int: None,
//       vec: Some(Vec::from_iter(vec![nested_item])),
//       item_type: NestableItemType::VEC,
//     };
//   };
//
//
//   0
// }

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
  use crate::day13::is_in_order;

  #[test]
  fn test_pair_1() {
    assert_eq!(
      is_in_order(&vec!["[1,1,3,1,1]", "[1,1,5,1,1]"]),
      true
    );
  }

  #[test]
  fn test_pair_2() {
    assert_eq!(
      is_in_order(&vec!["[[1],[2,3,4]]", "[[1],4]"]),
      true
    );
  }

  #[test]
  fn test_pair_3() {
    assert_eq!(
      is_in_order(&vec!["[9]", "[[8,7,6]]"]),
      false
    );
  }

  #[test]
  fn test_pair_4() {
    assert_eq!(
      is_in_order(&vec!["[[4,4],4,4]", "[[4,4],4,4,4]"]),
      true
    );
  }

  #[test]
  fn test_pair_5() {
    assert_eq!(
      is_in_order(&vec!["[7,7,7,7]", "[7,7,7]"]),
      false
    );
  }

  #[test]
  fn test_pair_6() {
    assert_eq!(
      is_in_order(&vec!["[]", "[3]"]),
      true
    );
  }

  #[test]
  fn test_pair_7() {
    assert_eq!(
      is_in_order(&vec!["[[[]]]", "[[]]"]),
      false
    );
  }

  #[test]
  fn test_pair_8() {
    assert_eq!(
      is_in_order(&vec!["[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"]),
      false
    );
  }
}
