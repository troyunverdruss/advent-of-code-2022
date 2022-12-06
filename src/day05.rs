use std::collections::{HashMap, LinkedList, VecDeque};
use crate::utils::read_chunks;

pub fn part_one() -> String {
  let lines = read_chunks("day05.txt", "\n\n");
  let stacks_blob = lines.get(0).expect("Should have gotten a first blob of the stacks");
  let instructions_blob = lines.get(1).expect("Should have gotten a second blob of the instructions");

  solve_one(stacks_blob, instructions_blob)
}

pub fn part_two() -> String {
  let lines = read_chunks("day05.txt", "\n\n");
  let stacks_blob = lines.get(0).expect("Should have gotten a first blob of the stacks");
  let instructions_blob = lines.get(1).expect("Should have gotten a second blob of the instructions");

  solve_two(stacks_blob, instructions_blob)
}

fn solve_one(stacks_blob: &String, instructions_blob: &String) -> String {
  let mut sorted_stacks = parse_sorted_stacks(stacks_blob);
  let instructions = parse_instructions(instructions_blob);

  for instruction in instructions {
    for _ in 1..=instruction.count {
      let mut from = sorted_stacks
        .get(&instruction.from)
        .unwrap()
        .to_owned();
      let mut to = sorted_stacks
        .get(&instruction.to)
        .unwrap()
        .to_owned();

      to.push_front(from.pop_front().unwrap());

      sorted_stacks.insert(instruction.from, from);
      sorted_stacks.insert(instruction.to, to);
    }
  }

  compute_result_value(sorted_stacks)
}

fn solve_two(stacks_blob: &String, instructions_blob: &String) -> String {
  let mut sorted_stacks = parse_sorted_stacks(stacks_blob);
  let instructions = parse_instructions(instructions_blob);

  for instruction in instructions {
    let mut from = sorted_stacks
      .get(&instruction.from)
      .unwrap()
      .to_owned();
    let mut to = sorted_stacks
      .get(&instruction.to)
      .unwrap()
      .to_owned();

    let range = 0..(instruction.count as usize);
    let from_vals: Vec<char> = from.drain(range).rev().collect();
    for v in from_vals {
      to.push_front(v);
    }

    sorted_stacks.insert(instruction.from, from);
    sorted_stacks.insert(instruction.to, to);
  }

  compute_result_value(sorted_stacks)
}



fn parse_sorted_stacks(stacks_blob: &String) -> HashMap<u64, VecDeque<char>> {
  let stacks_blob = stacks_blob
    .split("\n")
    .filter(|l| !l.is_empty())
    .map(|s| s.to_owned())
    .collect::<Vec<String>>();

  let stacks_tuples: Vec<(u64, char)> = stacks_blob
    .iter()
    .map(|l| line_column_tuples(l))
    .flatten()
    .collect();

  let mut unsorted_stacks = HashMap::new();
  for (k, v) in stacks_tuples {
    unsorted_stacks.entry(k).or_insert_with(Vec::new).push(v)
  }

  let mut sorted_stacks = HashMap::new();
  for (k, stack) in unsorted_stacks {
    let new_key = stack.last().unwrap().to_digit(10).unwrap() as u64;

    for v in stack {
      if v != ' ' {
        sorted_stacks.entry(new_key).or_insert_with(VecDeque::new).push_back(v)
      }
    }
  }

  sorted_stacks
}

fn parse_instructions(instructions_blob: &String) -> Vec<Instruction> {
  instructions_blob
    .split("\n")
    .filter(|l| !l.is_empty())
    .map(|line| line.split(" ").collect())
    .map(|chars: Vec<&str>| parse_instruction(&chars))
    .collect()
}



fn compute_result_value(sorted_stacks: HashMap<u64, VecDeque<char>>) -> String {
  let mut return_chars = Vec::new();
  for stack in 1..=sorted_stacks.len() {
    return_chars.push(sorted_stacks.get(&(stack as u64)).unwrap().get(0).unwrap())
  }

  let return_strings: Vec<String> = return_chars
    .iter()
    .map(|c| c.to_string())
    .collect();

  return_strings.join("")
}

fn line_column_tuples(line: &str) -> Vec<(u64, char)> {
  line.chars()
    .collect::<Vec<char>>()
    .chunks(4)
    .enumerate()
    .map(|(idx, val)|
      (
        idx as u64,
        val.get(1).expect("Expected a crate value").to_owned()
      )
    )
    .collect()
}

fn parse_instruction(split: &Vec<&str>) -> Instruction {
  let count = split
    .get(1)
    .expect("Expected a 'move' count")
    .parse::<u64>()
    .expect("Expected an integer");
  let from = split
    .get(3)
    .expect("Expected a 'move' count")
    .parse::<u64>()
    .expect("Expected an integer");
  let to = split
    .get(5)
    .expect("Expected a 'move' count")
    .parse::<u64>()
    .expect("Expected an integer");

  Instruction { count, from, to }
}

struct Instruction {
  count: u64,
  from: u64,
  to: u64,
}

#[cfg(test)]
mod tests {
  use crate::day05::{solve_one, solve_two};

  #[test]
  fn test_part_1_example() {
    let stacks_blob = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3".to_owned();
    let instructions_blob = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_owned();

    let result = solve_one(&stacks_blob, &instructions_blob);
    assert_eq!(result, "CMZ");
  }
  #[test]
  fn test_part_2_example() {
    let stacks_blob = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3".to_owned();
    let instructions_blob = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_owned();

    let result = solve_two(&stacks_blob, &instructions_blob);
    assert_eq!(result, "MCD");
  }
}