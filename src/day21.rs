use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::read_chunks;

pub fn part_one() -> i64 {
  let lines = read_chunks("day21.txt", "\n");
  let monkeys = parse_input(&lines);
  solve_one(&monkeys)
}

pub fn part_two() -> i64 {
  let lines = read_chunks("day21.txt", "\n");
  let monkeys = parse_input(&lines);
  solve_two(&monkeys)
}

fn solve_two(monkeys: &HashMap<String, String>) -> i64 {
  let (root_monkey_left, _, root_monkey_right) = split_monkey_into_math_parts(monkeys.get("root").unwrap());
  let orig_left_value = resolver(&monkeys, &root_monkey_left.to_string());
  let orig_right_value = resolver(&monkeys, &root_monkey_right.to_string());

  let mut monkeys_1 = monkeys.clone();
  monkeys_1.insert("humn".to_string(), "1".to_string());
  let new_left_value_1 = resolver(&monkeys_1, &root_monkey_left.to_string());

  let monkeys_2 = monkeys.clone();
  let new_left_value_2 = variable_resolver(&monkeys_2, &root_monkey_left.to_string());
  let new_right_value_2 = variable_resolver(&monkeys_2, &root_monkey_right.to_string());


  let (target_value, equation) = if new_left_value_1 == orig_left_value {
    (orig_left_value, new_right_value_2)
  } else {
    (orig_right_value, new_left_value_2)
  };

  math_solver(target_value, equation)
}

fn math_solver(target_value: i64, equation: String) -> i64 {
  // Need to reduce the expression some more
  let mut chars = equation.chars();
  chars.next();
  chars.next_back();
  let equation_stripped_one_parens = chars.as_str();

  lazy_static! {
        static ref RE_VAL_LEFT: Regex = Regex::new("^(\\d+) ([-+*/]) (\\(.*\\))$").unwrap();
        static ref RE_VAL_RIGHT: Regex = Regex::new("^(\\(.*\\)) ([-+*/]) (\\d+)$").unwrap();
    }

  if equation_stripped_one_parens.starts_with("(") {
    let caps = RE_VAL_RIGHT.captures(equation_stripped_one_parens).unwrap();
    let remaining_equation = caps[1].to_string();
    let operator = caps[2].to_string();
    let value = caps[3].to_string().parse::<i64>().unwrap();
    match operator.as_str() {
      "+" => math_solver(target_value - value, remaining_equation),
      "-" => math_solver(target_value + value, remaining_equation),
      "*" => math_solver(target_value / value, remaining_equation),
      "/" => math_solver(target_value * value, remaining_equation),
      _ => panic!("bad operator found")
    }
  } else if equation_stripped_one_parens.ends_with(")") {
    let caps = RE_VAL_LEFT.captures(equation_stripped_one_parens).unwrap();
    let remaining_equation = caps[3].to_string();
    let operator = caps[2].to_string();
    let value = caps[1].to_string().parse::<i64>().unwrap();
    match operator.as_str() {
      // todo
      "+" => math_solver(target_value - value, remaining_equation),
      "-" => math_solver(-1 * (target_value - value), remaining_equation),
      "*" => math_solver(target_value / value, remaining_equation),
      "/" => math_solver( value / target_value, remaining_equation),
      _ => panic!("bad operator found")
    }
  } else {
    // We have the final step where humn is a value and some other final operation to take
    let parts = equation_stripped_one_parens.split(" ").collect::<Vec<&str>>();
    if parts[0] == "humn" {
      match parts[1] {
        "+" => target_value - parts[2].parse::<i64>().unwrap(),
        "-" => target_value + parts[2].parse::<i64>().unwrap(),
        "*" => target_value / parts[2].parse::<i64>().unwrap(),
        "/" => target_value * parts[2].parse::<i64>().unwrap(),
        _ => panic!("bad operator found")
      }
    } else {
      match parts[1] {
        // todo
        "+" => target_value - parts[0].parse::<i64>().unwrap(),
        "-" => -1 * (target_value - parts[0].parse::<i64>().unwrap()),
        "*" => target_value / parts[0].parse::<i64>().unwrap(),
        "/" =>  parts[0].parse::<i64>().unwrap() / target_value,
        _ => panic!("bad operator found")
      }
    }
  }
}

fn solve_one(monkeys: &HashMap<String, String>) -> i64 {
  resolver(monkeys, &"root".to_string())
}


fn resolver(monkeys: &HashMap<String, String>, target_monkey_name: &String) -> i64 {
  let monkey_job = monkeys.get(target_monkey_name).unwrap();
  match monkey_job.parse::<i64>() {
    Ok(v) => { return v; }
    Err(_) => {
      let (m1, operator, m2) = split_monkey_into_math_parts(monkey_job);
      let m1 = resolver(monkeys, &m1.to_string());
      let m2 = resolver(monkeys, &m2.to_string());

      match operator {
        "+" => { m1 + m2 }
        "-" => { m1 - m2 }
        "*" => { m1 * m2 }
        "/" => { m1 / m2 }
        _ => { panic!("uh oh, bad operator") }
      }
    }
  }
}

fn variable_resolver(monkeys: &HashMap<String, String>, target_monkey_name: &String) -> String {
  let monkey_job = monkeys.get(target_monkey_name).unwrap();
  if target_monkey_name == "humn" {
    return "humn".to_string();
  }

  match monkey_job.parse::<i64>() {
    Ok(v) => { return v.to_string(); }
    Err(_) => {
      let (m1, operator, m2) = split_monkey_into_math_parts(monkey_job);
      let m1 = variable_resolver(monkeys, &m1.to_string());
      let m2 = variable_resolver(monkeys, &m2.to_string());

      if m1.contains("humn") || m2.contains("humn") {
        return format!("({} {} {})", m1, operator, m2);
      }

      let result: i64 = match operator {
        "+" => { m1.parse::<i64>().unwrap() + m2.parse::<i64>().unwrap() }
        "-" => { m1.parse::<i64>().unwrap() - m2.parse::<i64>().unwrap() }
        "*" => { m1.parse::<i64>().unwrap() * m2.parse::<i64>().unwrap() }
        "/" => { m1.parse::<i64>().unwrap() / m2.parse::<i64>().unwrap() }
        _ => { panic!("uh oh, bad operator") }
      };

      return format!("{}", result);
    }
  }
}


fn split_monkey_into_math_parts(monkey_job: &String) -> (&str, &str, &str) {
  let parts = monkey_job.split(" ").collect::<Vec<&str>>();

  (parts.get(0).unwrap(), parts.get(1).unwrap(), parts.get(2).unwrap())
}


fn parse_input(lines: &Vec<String>) -> HashMap<String, String> {
  lines
    .iter()
    .map(|s| s.split(":"))
    .map(|s| s.collect::<Vec<&str>>())
    .map(|s| (
      s.get(0).unwrap().to_string(),
      s.get(1).unwrap().trim().to_string()
    )
    )
    .collect()
}

#[cfg(test)]
mod tests {
  use crate::day21::{math_solver, parse_input, solve_one, solve_two};

  #[test]
  fn test_part_1() {
    let input = get_part_1_input();
    let monkeys = parse_input(&input);
    let root_yells = solve_one(&monkeys);
    assert_eq!(root_yells, 152);
  }

  #[test]
  fn test_part_2() {
    let input = get_part_1_input();
    let monkeys = parse_input(&input);
    let value_needed = solve_two(&monkeys);
    assert_eq!(value_needed, 301);
  }

  #[test]
  fn test_solve_expr_with_humn_sec_1_subtract() {
    let target_value = 1;
    let equation = "(5 + (2 - humn))".to_string();
    let res = math_solver(target_value, equation);
    assert_eq!(res, 6);
  }

  #[test]
  fn test_solve_expr_with_humn_sec_2_subtract() {
    let target_value = 1;
    let equation = "(5 - (2 - humn))".to_string();
    let res = math_solver(target_value, equation);
    assert_eq!(res, -2);
  }

  #[test]
  fn test_solve_expr_with_humn_sec_1_divide() {
    let target_value = 1;
    let equation = "(5 + (8 / humn))".to_string();
    let res = math_solver(target_value, equation);
    assert_eq!(res, -2);
  }

  #[test]
  fn test_solve_expr_with_humn_sec_2_divide() {
    let target_value = 1;
    let equation = "(4 / (8 / humn))".to_string();
    let res = math_solver(target_value, equation);
    assert_eq!(res, 2);
  }

  fn get_part_1_input() -> Vec<String> {
    vec![
      "root: pppw + sjmn".to_string(),
      "dbpl: 5".to_string(),
      "cczh: sllz + lgvd".to_string(),
      "zczc: 2".to_string(),
      "ptdq: humn - dvpt".to_string(),
      "dvpt: 3".to_string(),
      "lfqf: 4".to_string(),
      "humn: 5".to_string(),
      "ljgn: 2".to_string(),
      "sjmn: drzm * dbpl".to_string(),
      "sllz: 4".to_string(),
      "pppw: cczh / lfqf".to_string(),
      "lgvd: ljgn * ptdq".to_string(),
      "drzm: hmdt - zczc".to_string(),
      "hmdt: 32".to_string(),
    ]
  }
}
