use std::collections::HashSet;
use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day03.txt", "\n");

  lines
    .iter()
    .map(|l| find_common_letter(l))
    .map(|c| score_for_letter(&c))
    .sum()
}

pub fn part_two() -> u64 {
  let groups = read_chunks("day03.txt", "\n");
  groups.chunks_exact(3)
    .map(|chunk| {
      find_common_from_three(&chunk[0], &chunk[1], &chunk[2])
    })
    .map(|c| score_for_letter(&c))
    .sum()
}

fn find_common_from_three(c1: &String, c2: &String, c3: &String) -> char {
  // find the common letters in first 2 chunks
  let common_from_first_two: HashSet<char> = to_set(c1)
    .intersection(&to_set(c2))
    .map(|i| i.to_owned())
    .collect();

  // then find the common letters between that and the third to get the answer
  common_from_first_two
    .intersection(&to_set(c3))
    .map(|i| i.to_owned())
    .nth(0)
    .expect("Should have found at least one common letter")
}


fn find_common_letter(rucksack_contents: &String) -> char {
  let (left, right) = rucksack_contents.split_at(rucksack_contents.len() / 2);
  assert_eq!(left.len(), right.len());

  let left = to_set(&left);
  let right = to_set(&right);

  let intersection = left
    .intersection(&right)
    .map(|c| c.to_owned())
    .collect::<Vec<char>>();
  assert_eq!(intersection.len(), 1);

  intersection
    .iter()
    .nth(0)
    .expect("Should always have an element due to above assertion")
    .to_owned()
}

fn to_set(slice: &str) -> HashSet<char> {
  HashSet::from_iter(slice.chars())
}

fn score_for_letter(letter: &char) -> u64 {
  if letter.is_lowercase() {
    *letter as u64 - 96
  } else {
    *letter as u64 - 38
  }
}

#[cfg(test)]
mod tests {
  use crate::day03::{find_common_letter, score_for_letter};

  #[test]
  fn test_finding_common_and_scoring() {
    let s1 = "vJrwpWtwJgWrhcsFMMfFFhFp".to_owned();
    let s2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_owned();
    let s3 = "PmmdzqPrVvPwwTWBwg".to_owned();
    let s4 = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned();
    let s5 = "ttgJtRGJQctTZtZT".to_owned();
    let s6 = "CrZsJsPPZsGzwwsLwLmpwMDw".to_owned();

    let c1 = find_common_letter(&s1);
    let c2 = find_common_letter(&s2);
    let c3 = find_common_letter(&s3);
    let c4 = find_common_letter(&s4);
    let c5 = find_common_letter(&s5);
    let c6 = find_common_letter(&s6);

    assert_eq!(c1, 'p');
    assert_eq!(c2, 'L');
    assert_eq!(c3, 'P');
    assert_eq!(c4, 'v');
    assert_eq!(c5, 't');
    assert_eq!(c6, 's');

    assert_eq!(score_for_letter(&c1), 16);
    assert_eq!(score_for_letter(&c2), 38);
    assert_eq!(score_for_letter(&c3), 42);
    assert_eq!(score_for_letter(&c4), 22);
    assert_eq!(score_for_letter(&c5), 20);
    assert_eq!(score_for_letter(&c6), 19);
  }
}