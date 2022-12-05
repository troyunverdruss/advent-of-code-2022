use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  let lines = read_chunks("day04.txt", "\n");
  solve_one(lines)
}

fn solve_one(lines: Vec<String>) -> u64 {
  lines
    .iter()
    .map(|l| l.split_once(",").expect("Input should be delimited by a comma"))
    .filter(|pair| {
      let ranges = pair.to_owned();
      first_range_fully_contains_second(ranges.0, ranges.1) ||
        first_range_fully_contains_second(ranges.1, ranges.0)
    })
    .count() as u64
}

pub fn part_two() -> u64 {
  let lines = read_chunks("day04.txt", "\n");
  solve_two(lines)
}

fn solve_two(lines: Vec<String>) -> u64 {
  lines
    .iter()
    .map(|l| l.split_once(",").expect("Input should be delimited by a comma"))
    .filter(|pair| {
      let ranges = pair.to_owned();
      first_range_contains_start_or_end_of_second(ranges.0, ranges.1) ||
        first_range_contains_start_or_end_of_second(ranges.1, ranges.0)
    })
    .count() as u64
}


fn first_range_fully_contains_second(range_string_one: &str, range_string_two: &str) -> bool {
  let range_one = range_to_numeric_start_and_end(range_string_one);
  let range_two = range_to_numeric_start_and_end(range_string_two);

  // To be fully enclosed, the start of the second range must be equal or greater than the first
  // range and the end of the range must be before or equal to the first range's end.
  range_two.start >= range_one.start && range_two.end <= range_one.end
}

fn first_range_contains_start_or_end_of_second(range_string_one: &str, range_string_two: &str) -> bool {
  let range_one = range_to_numeric_start_and_end(range_string_one);
  let range_two = range_to_numeric_start_and_end(range_string_two);

  // For the second range to overlap its ends with the first it needs to either be
  // the start that fits inside the first range or its end that fits inside the first range
  (range_two.start >= range_one.start && range_two.start <= range_one.end) ||
    (range_two.end >= range_one.start && range_two.end <= range_one.end)
}

fn range_to_numeric_start_and_end(range: &str) -> Range {
  let parts = range.split_once("-");
  match parts {
    None => panic!("Should have been able to split on dash in every range"),
    Some(parts) => Range {
      start: parts.0.parse::<u64>().expect("Input range starts should be numeric"),
      end: parts.1.parse::<u64>().expect("Input range ends should be numeric"),
    }
  }
}

struct Range {
  start: u64,
  end: u64,
}


#[cfg(test)]
mod tests {
  use crate::day04::{solve_one, solve_two};

  #[test]
  fn test_finding_part_one_with_full_overlap() {
    let input = vec![
      "2-4,6-8".to_owned(),
      "2-3,4-5".to_owned(),
      "5-7,7-9".to_owned(),
      "2-8,3-7".to_owned(),
      "6-6,4-6".to_owned(),
      "2-6,4-8".to_owned(),
    ];

    assert_eq!(solve_one(input), 2);
  }

  #[test]
  fn test_finding_part_two_with_any_overlap() {
    let input = vec![
      "2-4,6-8".to_owned(),
      "2-3,4-5".to_owned(),
      "5-7,7-9".to_owned(),
      "2-8,3-7".to_owned(),
      "6-6,4-6".to_owned(),
      "2-6,4-8".to_owned(),
    ];

    assert_eq!(solve_two(input), 4);
  }
}