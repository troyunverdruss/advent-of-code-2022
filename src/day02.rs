use crate::utils::read_chunks;

pub fn part_one() -> u64 {
  solve(score_round_part_one)
}

pub fn part_two() -> u64 {
  solve(score_round_part_two)
}

fn solve<T>(scoring_func: T) -> u64 where T: Fn(&Vec<String>) -> u64 {
  let chunks = read_chunks("day02.txt", "\n");
  let round_pairs: Vec<Vec<String>> = chunks.iter()
    .filter(|l| !l.is_empty())
    .map(|l| l.split(" ").map(|s| s.to_owned()).collect())
    .collect();

  // Total Score
  round_pairs
    .iter()
    .map(|rp| scoring_func(rp))
    .sum()
}

fn score_round_part_one(round_input: &Vec<String>) -> u64 {
  let prediction_opt = round_input.get(0);
  let response_opt = round_input.get(1);

  let prediction = match prediction_opt {
    None => panic!("No input found for prediction"),
    Some(p) => p.as_str()
  };

  let response = match response_opt {
    None => panic!("No input found for response"),
    Some(r) => r.as_str()
  };

  let value_for_game: u64 = match prediction {
    "A" => match response {
      "X" => 3,
      "Y" => 6,
      "Z" => 0,
      _ => panic!("Unknown response value")
    },
    "B" => match response {
      "X" => 0,
      "Y" => 3,
      "Z" => 6,
      _ => panic!("Unknown response value")
    },
    "C" => match response {
      "X" => 6,
      "Y" => 0,
      "Z" => 3,
      _ => panic!("Unknown response value")
    },
    _ => panic!("Unknown prediction value")
  };

  let value_for_response: u64 = match response {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
    _ => panic!("Unknown response value")
  };

  value_for_game + value_for_response
}

fn score_round_part_two(round_input: &Vec<String>) -> u64 {
  let prediction_opt = round_input.get(0);
  let response_opt = round_input.get(1);

  let prediction = match prediction_opt {
    None => panic!("No input found for prediction"),
    Some(p) => p.as_str()
  };

  let required_outcome = match response_opt {
    None => panic!("No input found for required outcome"),
    Some(ro) => ro.as_str()
  };

  let response_played;

  // X = I must lose
  // Y = I must draw
  // Z = I must win
  let value_for_game: u64 = match prediction {
    "A" => match required_outcome {
      "X" => { response_played = "C"; 0},
      "Y" => { response_played = "A"; 3},
      "Z" => { response_played = "B"; 6},
      _ => panic!("Unknown required outcome value")
    },
    "B" => match required_outcome {
      "X" => { response_played = "A"; 0},
      "Y" => { response_played = "B"; 3},
      "Z" => { response_played = "C"; 6},
      _ => panic!("Unknown required outcome value")
    },
    "C" => match required_outcome {
      "X" => { response_played = "B"; 0},
      "Y" => { response_played = "C"; 3},
      "Z" => { response_played = "A"; 6},
      _ => panic!("Unknown required outcome value")
    },
    _ => panic!("Unknown prediction value")
  };

  let value_for_response: u64 = match response_played {
    "A" => 1,
    "B" => 2,
    "C" => 3,
    _ => panic!("Unknown response value")
  };

  value_for_game + value_for_response
}

#[cfg(test)]
mod tests {
  use crate::day02::{score_round_part_one, score_round_part_two};

  #[test]
  fn test_round_one_scoring() {
    let e1: Vec<String> = vec!["A".to_owned(), "Y".to_owned()];
    let e2: Vec<String> = vec!["B".to_owned(), "X".to_owned()];
    let e3: Vec<String> = vec!["C".to_owned(), "Z".to_owned()];

    assert_eq!(score_round_part_one(&e1), 8);
    assert_eq!(score_round_part_one(&e2), 1);
    assert_eq!(score_round_part_one(&e3), 6);
  }

  #[test]
  fn test_round_two_scoring() {
    let e1: Vec<String> = vec!["A".to_owned(), "Y".to_owned()];
    let e2: Vec<String> = vec!["B".to_owned(), "X".to_owned()];
    let e3: Vec<String> = vec!["C".to_owned(), "Z".to_owned()];

    assert_eq!(score_round_part_two(&e1), 4);
    assert_eq!(score_round_part_two(&e2), 1);
    assert_eq!(score_round_part_two(&e3), 7);
  }
}