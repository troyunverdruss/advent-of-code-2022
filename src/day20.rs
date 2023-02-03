use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::ops::{Add, Index};
use std::str::Split;


use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let lines = read_chunks("day20.txt", "\n");
  let numbers = parse_input(&lines);
  let mixed_numbers = mix_numbers(&numbers);
  let grove_coords = find_grove_coords(&mixed_numbers);
  grove_coords
  //6220 too low
}


fn mix_numbers(numbers: &Vec<i64>) -> Vec<i64> {
  let mut deque = VecDeque::from_iter(numbers.clone());
  for n in numbers {
    while deque.front().unwrap() != n {
      deque.rotate_left(1);
    }
    let num = deque.pop_front().unwrap().clone();
    let rotations = num % (numbers.len() as i64 - 1);
    // let wraps = num / numbers.len() as i64;
    // let rotations = rotations + wraps;
    // assert!(rotations.abs() < (numbers.len()-1) as i64);
    if rotations > 0 {
      deque.rotate_left(rotations.abs() as usize);
    } else {
      deque.rotate_right(rotations.abs() as usize);
    }
    deque.push_front(num);
    println!("P: {}, list: {:?}", num, deque);
  }

  while deque.front().unwrap() != &0 {
    deque.rotate_left(1);
  }

  Vec::from_iter(deque)
}

fn find_grove_coords(mixed_numbers: &Vec<i64>) -> i64 {
  let zero_index = mixed_numbers.iter().position(|v| v == &0).unwrap();
  let coord_one = mixed_numbers.get((zero_index + 1000) % mixed_numbers.len()).unwrap();
  let coord_two = mixed_numbers.get((zero_index + 2000) % mixed_numbers.len()).unwrap();
  let coord_three = mixed_numbers.get((zero_index + 3000) % mixed_numbers.len()).unwrap();
  println!("{} {} {} ", coord_one, coord_two, coord_three);
  coord_one + coord_two + coord_three
}


pub fn part_two() -> u64 {
  0
}

fn parse_input(lines: &Vec<String>) -> Vec<i64> {
  lines
    .iter()
    .map(|s| s.parse().unwrap())
    .collect()
}








#[cfg(test)]
mod tests {
  use crate::day20::{find_grove_coords, mix_numbers, parse_input};

  #[test]
  fn test_mixing_example_1() {
    let input = get_part_1_input();
    let numbers = parse_input(&input);
    let mixed_numbers = mix_numbers(&numbers);
    let grove_coords = find_grove_coords(&mixed_numbers);

    assert_eq!(grove_coords, 3);
  }

  #[test]
  fn test_mixing_fake_input() {
    let input = get_part_1_fake_input();
    let numbers = parse_input(&input);
    let mixed_numbers = mix_numbers(&numbers);

    assert_eq!(
      mixed_numbers,
      vec![0, -5, 9, 2]
    );

    let grove_coords = find_grove_coords(&mixed_numbers);

    assert_eq!(grove_coords, 3);
  }

  fn get_part_1_input() -> Vec<String> {
    vec![
      "1".to_string(),
      "2".to_string(),
      "-3".to_string(),
      "3".to_string(),
      "-2".to_string(),
      "0".to_string(),
      "4".to_string(),
    ]
  }

  fn get_part_1_fake_input() -> Vec<String> {
    vec![
      "9".to_string(),
      "0".to_string(),
      "-5".to_string(),
      "2".to_string(),
    ]
  }
}
