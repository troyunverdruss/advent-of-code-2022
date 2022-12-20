use std::collections::{HashMap, HashSet};
use crate::day08::Point;
use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let lines = read_chunks("day11.txt", "Monkey");
  solve_one(&lines)
}

pub fn part_two() -> u64 {
  // let lines = read_chunks("day09.txt", "\n");
  // solve_two(&lines)
  0
}


#[derive(Clone)]
struct Monkey {
  name: String,
  items: Vec<i64>,
  operator: String,
  operand: String,
  test_divisible_by: i64,
  dest_if_true: i64,
  dest_if_false: i64,
}

impl Monkey {
  fn clone_new_items(self: &Self, new_items: &Vec<i64>) -> Monkey {
    Monkey {
      name: self.name.to_string(),
      items: new_items.clone(),
      operator: self.operator.to_string(),
      operand: self.operand.to_string(),
      test_divisible_by: self.test_divisible_by.clone(),
      dest_if_true: self.dest_if_true.clone(),
      dest_if_false: self.dest_if_false.clone(),
    }
  }

}

fn solve_one(groups: &Vec<String>) -> i64 {
  let mut monkeys: Vec<Monkey> = groups
    .iter()
    .map(|g| {
      let lines = g.split("\n").collect::<Vec<&str>>();
      let name = lines.get(0).unwrap().to_string();
      let items: Vec<i64> = lines
        .get(1).unwrap()
        .split(':')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .get(1).unwrap()
        .split(",")
        .map(|v| v.trim().to_string().parse().unwrap())
        .collect();
      let operator = lines
        .get(2).unwrap()
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .get(4).unwrap()
        .to_string();
      let operand = lines
        .get(2).unwrap()
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .get(5).unwrap()
        .to_string();
      let test_divisible_by = lines
        .get(3).unwrap()
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .get(3).unwrap()
        .to_string()
        .parse().unwrap();
      let dest_if_true = lines
        .get(4).unwrap()
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .get(5).unwrap()
        .to_string()
        .parse().unwrap();
      let dest_if_false = lines
        .get(5).unwrap()
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .get(5).unwrap()
        .to_string()
        .parse().unwrap();

      Monkey {
        name,
        items,
        operator,
        operand,
        test_divisible_by,
        dest_if_true,
        dest_if_false
      }
    })
    .collect();

  let mut inspections = vec![];
  for i in 0..monkeys.len() {
    inspections.push(0);
  }

  for round in 0..20 {
    for curr_monkey_id in 0..monkeys.len() {
      let curr_monkey = monkeys.get(curr_monkey_id).unwrap().clone();
      for item in &curr_monkey.items {
        inspections[curr_monkey_id] = inspections.get(curr_monkey_id).unwrap() + 1;
        let worry_operand = if curr_monkey.operand == "old" {
          item.to_owned()
        } else {
          curr_monkey.operand.parse::<i64>().unwrap()
        };
        let interim_worry_value = if curr_monkey.operator == "+" {
          item + worry_operand
        } else {
          item * worry_operand
        };
        let worry_operand = interim_worry_value / 3;

        let mut updated_dest_monkey;
        let mut dest_monkey_id;
        if worry_operand % curr_monkey.test_divisible_by == 0 {
          dest_monkey_id = curr_monkey.dest_if_true as usize;
          let dest_monkey = monkeys.get(dest_monkey_id).unwrap().clone();
          let mut new_items = dest_monkey.items.clone();
          new_items.push(worry_operand);
          updated_dest_monkey = dest_monkey.clone_new_items(&new_items);
        } else {
          dest_monkey_id = curr_monkey.dest_if_false as usize;
          let dest_monkey = monkeys.get(dest_monkey_id).unwrap().clone();
          let mut new_items = dest_monkey.items.clone();
          new_items.push(worry_operand);
          updated_dest_monkey = dest_monkey.clone_new_items(&new_items);
        }
        monkeys[dest_monkey_id] = updated_dest_monkey;
        monkeys[curr_monkey_id] = curr_monkey.clone_new_items(&vec![]);
      }
    }
  }

  inspections.sort();
  inspections.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
  use crate::day11::solve_one;

  #[test]
  fn test_sample_1() {
    let inputs = get_inputs();
    let result = solve_one(&inputs);
    assert_eq!(result, 10605);
  }

  fn get_inputs() -> Vec<String> {
    let raw_input =
      "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    raw_input.split("Monkey")
      .collect::<Vec<&str>>()
      .iter()
      .map(|l| l.to_string())
      .filter(|l| !l.is_empty())
      .collect()
  }
}
