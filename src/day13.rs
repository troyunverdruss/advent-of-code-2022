use std::cmp::{max, Ordering};
use std::collections::HashMap;

use serde_json::{Number, Value};

use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let chunks = read_chunks("day13.txt", "\n\n");
  solve_one(&chunks)
}

pub fn part_two() -> u64 {
  let chunks = read_chunks("day13.txt", "\n\n");

  let packets: Vec<String> = chunks
    .iter()
    .map(|c| c.split('\n'))
    .flatten()
    .map(|s| s.to_string())
    .filter(|s| !s.is_empty())
    .collect();

  solve_two(&packets)
}

fn solve_one(chunks: &Vec<String>) -> u64 {
  let index_to_in_order: HashMap<usize, Ordering> = chunks
    .iter()
    .map(|c| c.split("\n").collect::<Vec<&str>>())
    .enumerate()
    .map(|p| {
      (
        p.0,
        compare(
          &serde_json::from_str(p.1[0]).unwrap(),
          &serde_json::from_str(p.1[1]).unwrap(),
        )
      )
    })
    .collect();

  let sum: usize = index_to_in_order
    .iter()
    .filter(|p| (*p).1 == &Ordering::Less)
    .map(|p| *p.0 + 1)// Indexes are 1-based!!
    .sum();

  sum as u64
}

fn solve_two(packets: &Vec<String>) -> u64 {
  let divider_packets = vec!["[[2]]".to_string(), "[[6]]".to_string()];
  let mut packets = packets.clone();
  packets.extend(divider_packets.clone());
  let mut parsed_packets: Vec<Value> = packets
    .iter()
    .map(|p| serde_json::from_str(p).unwrap())
    .collect();

  parsed_packets.sort_by(compare);

  let product: usize = parsed_packets
    .iter()
    .map(|v| serde_json::to_string(v).unwrap())
    .enumerate()
    .filter(|p| divider_packets.contains(&(*p).1.to_owned()))
    .map(|p| p.0 + 1)
    .product();

  product as u64
}

fn compare(left: &Value, right: &Value) -> Ordering {
  match left {
    Value::Number(n) => { return handle_left_number(n, right); }
    Value::Array(a) => { return handle_left_array(a, right); }
    _ => panic!("Unexpected value type")
  }
}

fn handle_left_array(left_array: &Vec<Value>, right: &Value) -> Ordering {
  match right {
    Value::Number(_) => { handle_two_array(left_array, &vec![right.to_owned()]) }
    Value::Array(a) => { handle_two_array(left_array, a) }
    _ => panic!("Unexpected value type")
  }
}

fn handle_two_array(left_array: &Vec<Value>, right_array: &Vec<Value>) -> Ordering {
  let max_len = max(left_array.len(), right_array.len());

  for i in 0..max_len {
    let next_left = left_array.get(i);
    let next_right = right_array.get(i);

    if next_left.is_none() && next_right.is_none() {
      return Ordering::Equal;
    } else if next_left.is_none() && next_right.is_some() {
      return Ordering::Less;
    } else if next_left.is_some() && next_right.is_none() {
      return Ordering::Greater;
    } else {
      let left = next_left.unwrap();
      let right = next_right.unwrap();

      let result = match left {
        Value::Number(n) => { handle_left_number(n, right) }
        Value::Array(a) => { handle_left_array(a, right) }
        _ => panic!("Unexpected value type")
      };

      if result != Ordering::Equal {
        return result;
      }
    }
  }

  return Ordering::Equal;
}

fn handle_left_number(left_number: &Number, right: &Value) -> Ordering {
  match right {
    Value::Number(n) => {
      if left_number.as_i64() < n.as_i64() {
        Ordering::Less
      } else if left_number.as_i64() > n.as_i64() {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    }
    Value::Array(a) => { handle_two_array(&vec![Value::Number(left_number.to_owned())], a) }
    _ => panic!("Unexpected value type")
  }
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use crate::day13::{compare, solve_one, solve_two};

  #[test]
  fn test_pair_1() {
    let left = serde_json::from_str("[1,1,3,1,1]").unwrap();
    let right = serde_json::from_str("[1,1,5,1,1]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Less
    );
  }

  #[test]
  fn test_pair_2() {
    let left = serde_json::from_str("[[1],[2,3,4]]").unwrap();
    let right = serde_json::from_str("[[1],4]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Less
    );
  }

  #[test]
  fn test_pair_3() {
    let left = serde_json::from_str("[9]").unwrap();
    let right = serde_json::from_str("[[8,7,6]]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Greater
    );
  }

  #[test]
  fn test_pair_4() {
    let left = serde_json::from_str("[[4,4],4,4]").unwrap();
    let right = serde_json::from_str("[[4,4],4,4,4]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Less
    );
  }

  #[test]
  fn test_pair_5() {
    let left = serde_json::from_str("[7,7,7,7]").unwrap();
    let right = serde_json::from_str("[7,7,7]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Greater
    );
  }

  #[test]
  fn test_pair_6() {
    let left = serde_json::from_str("[]").unwrap();
    let right = serde_json::from_str("[3]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Less
    );
  }

  #[test]
  fn test_pair_7() {
    let left = serde_json::from_str("[[[]]]").unwrap();
    let right = serde_json::from_str("[[]]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Greater
    );
  }

  #[test]
  fn test_pair_8() {
    let left = serde_json::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
    let right = serde_json::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();

    assert_eq!(
      compare(&left, &right),
      Ordering::Greater
    );
  }

  #[test]
  fn test_example_1() {
    let chunks = get_input_as_chunks();
    let result = solve_one(&chunks);
    assert_eq!(result, 13);
  }

  #[test]
  fn test_example_2() {
    let all_packets = get_input_all_individual_packets();
    let result = solve_two(&all_packets);
    assert_eq!(result, 140);
  }

  fn get_input_as_chunks() -> Vec<String> {
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

  fn get_input_all_individual_packets() -> Vec<String> {
    get_input_as_chunks()
      .iter()
      .map(|c| c.split('\n'))
      .flatten()
      .map(|s| s.to_string())
      .collect()
  }
}
