use std::collections::VecDeque;

use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let lines = read_chunks("day20.txt", "\n");
  let numbers = parse_input(&lines);
  let mixed_numbers = mix_numbers(&numbers, 1);
  let grove_coords = find_grove_coords(&mixed_numbers);
  grove_coords
}

#[derive(Clone)]
struct Value {
  number: i64,
  position: usize,
}

fn mix_numbers(numbers: &Vec<i64>, times: usize) -> Vec<i64> {
  let mut deque = numbers
    .iter()
    .enumerate()
    .map(|(i, v)| Value { number: *v, position: i })
    .collect::<VecDeque<Value>>();

  for _ in 0..times {
    for curr_position in 0..(numbers.len()) {
      while deque.front().unwrap().position != curr_position {
        deque.rotate_left(1);
      }

      let num = deque.pop_front().unwrap().clone();
      let rotations = num.number % (numbers.len() as i64 - 1);

      if num.number > 0 {
        deque.rotate_left(rotations.abs() as usize);
        deque.push_front(num.clone());
      } else {
        deque.rotate_right(rotations.abs() as usize);
        deque.push_front(num.clone());
      }
    }
  }

  while deque.front().unwrap().number != 0 {
    deque.rotate_left(1);
  }

  deque
    .iter()
    .map(|v| v.number)
    .collect()
}

fn find_grove_coords(mixed_numbers: &Vec<i64>) -> i64 {
  let zero_index = mixed_numbers.iter().position(|v| v == &0).unwrap();
  let coord_one = mixed_numbers.get((zero_index + 1000) % mixed_numbers.len()).unwrap();
  let coord_two = mixed_numbers.get((zero_index + 2000) % mixed_numbers.len()).unwrap();
  let coord_three = mixed_numbers.get((zero_index + 3000) % mixed_numbers.len()).unwrap();
  // println!("{} {} {} ", coord_one, coord_two, coord_three);
  coord_one + coord_two + coord_three
}


pub fn part_two() -> i64 {
  let lines = read_chunks("day20.txt", "\n");
  let numbers = parse_input(&lines);
  let decryption_key = 811589153;
  let updated_numbers = numbers
    .iter()
    .map(|n| n * decryption_key)
    .collect();
  let mixed_numbers = mix_numbers(&updated_numbers, 10);
  let grove_coords = find_grove_coords(&mixed_numbers);
  grove_coords
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
    let mixed_numbers = mix_numbers(&numbers, 1);
    let grove_coords = find_grove_coords(&mixed_numbers);

    assert_eq!(grove_coords, 3);
  }

  #[test]
  fn test_mixing_fake_input() {
    let input = get_part_1_fake_input();
    let numbers = parse_input(&input);
    let mixed_numbers = mix_numbers(&numbers, 1);

    assert_eq!(
      mixed_numbers,
      vec![0, -5, 9, 2]
    );

    let grove_coords = find_grove_coords(&mixed_numbers);

    assert_eq!(grove_coords, 0);
  }

  #[test]
  fn test_0_n1_n1_1() {
    let numbers = vec![0, -1, -1, 1];
    let mixed = mix_numbers(&numbers, 1);
    assert_eq!(
      mixed,
      vec![0, -1, 1, -1]
    )
  }

  #[test]
  fn test_3_1_0() {
    let numbers = vec![3, 1, 0];
    let mixed = mix_numbers(&numbers, 1);
    assert_eq!(
      mixed,
      vec![0, 3, 1]
    )
  }

  #[test]
  fn test_adding_8() {
    let numbers = vec![1, 2, -3, 3, -2, 0, 8];
    let mixed_numbers = mix_numbers(&numbers, 1);
    let grove_coords = find_grove_coords(&mixed_numbers);
    assert_eq!(grove_coords, 7);
  }

  #[test]
  fn test_part_2() {
    let numbers: Vec<i64> = vec![
      811589153, 1623178306, -2434767459, 2434767459, -1623178306, 0, 3246356612,
    ];
    let mixed_numbers = mix_numbers(&numbers, 10);
    assert_eq!(
      mixed_numbers,
      vec![
        0, -2434767459, 1623178306, 3246356612, -1623178306, 2434767459, 811589153,
      ]
    );

    let grove_coords = find_grove_coords(&mixed_numbers);
    assert_eq!(grove_coords, 1623178306);
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
