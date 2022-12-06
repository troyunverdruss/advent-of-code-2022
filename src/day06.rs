use std::collections::{HashSet};
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day06.txt", "\n");
  let all_chars = input_to_chars(lines);

  find_first_marker_location(&all_chars, 4)
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day06.txt", "\n");
  let all_chars = input_to_chars(lines);

  find_first_marker_location(&all_chars, 14)
}

fn input_to_chars(lines: Vec<String>) -> Vec<String> {
  let all_chars: Vec<String> = lines
    .iter()
    .map(|l| l.chars())
    .map(|chars| chars.map(|c| c.to_string()))
    .flatten()
    .collect();
  all_chars
}

fn find_first_marker_location(all_chars: &Vec<String>, unit_length: u64) -> u64 {
  let index_safe_unit_length: usize = unit_length as usize - 1;

  for index in index_safe_unit_length..(all_chars.len()) {
    let slice = &all_chars[(index-index_safe_unit_length)..=index];
    let set: HashSet<&String> = HashSet::from_iter(slice.iter());
    if set.len() == unit_length as usize {
      return index as u64 + 1;
    }
  }

  panic!("Never found a proper marker")
}

#[cfg(test)]
mod tests {
  use crate::day06::find_first_marker_location;

  #[test]
  fn test_cases_part_1() {
    let inputs = get_inputs();

    let unit_length = 4;
    assert_eq!(find_first_marker_location(inputs.get(0).unwrap(), unit_length), 7);
    assert_eq!(find_first_marker_location(inputs.get(1).unwrap(), unit_length), 5);
    assert_eq!(find_first_marker_location(inputs.get(2).unwrap(), unit_length), 6);
    assert_eq!(find_first_marker_location(inputs.get(3).unwrap(), unit_length), 10);
    assert_eq!(find_first_marker_location(inputs.get(4).unwrap(), unit_length), 11);
  }

  #[test]
  fn test_cases_part_2() {
    let inputs = get_inputs();

    let unit_length = 14;
    assert_eq!(find_first_marker_location(inputs.get(0).unwrap(), unit_length), 19);
    assert_eq!(find_first_marker_location(inputs.get(1).unwrap(), unit_length), 23);
    assert_eq!(find_first_marker_location(inputs.get(2).unwrap(), unit_length), 23);
    assert_eq!(find_first_marker_location(inputs.get(3).unwrap(), unit_length), 29);
    assert_eq!(find_first_marker_location(inputs.get(4).unwrap(), unit_length), 26);
  }

  fn get_inputs() -> Vec<Vec<String>> {
    let inputs: Vec<Vec<String>> = vec![
      "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
      "bvwbjplbgvbhsrlpgdmjqwftvncz",
      "nppdvjthqldpwncqszvftbrmjlhg",
      "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
      "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ]
      .iter()
      .map(|l| l.chars())
      .map(|chars| chars.map(|c| c.to_string()).collect())
      .collect();
    inputs
  }

  #[test]
  fn understand_flatten() {
    let input = vec!["abc", "def"];

    let chars: Vec<char> = input
      .iter()
      .map(|l| l.chars())
      .flatten()
      .collect();

    assert_eq!(chars, vec!['a', 'b', 'c', 'd', 'e', 'f']);
  }

  #[test]
  fn understand_slicing_vecs() {
    let data = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let data_s: Vec<String> = data.iter().map(|c| c.to_string()).collect();
    let x = &data_s[0..3];

    assert_eq!(x, &["a", "b", "c"])
  }
}