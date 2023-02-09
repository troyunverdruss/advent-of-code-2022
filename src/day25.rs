use radix::RadixNum;

use crate::utils::read_chunks;

pub fn part_one() -> String {
  let snafu_input = read_chunks("day25.txt", "\n");
  let decimal_sum: u64 = snafu_input
    .iter()
    .map(|sn| snafu_to_decimal(sn))
    .sum();

  decimal_to_snafu(&decimal_sum)
}

fn decimal_to_snafu(d: &u64) -> String {
  let base_5_num = RadixNum::from_str(&d.to_string(), 10).unwrap()
    .with_radix(5).unwrap();

  let base_5_digits = base_5_num
    .as_str()
    .chars()
    .map(|c| c.to_string().parse::<u64>().unwrap())
    .collect::<Vec<u64>>();

  let (converted_digits, final_overflowed) = base_5_digits
    .iter()
    .rev()
    .fold((vec![], false), |(digits, overlflowed), next_digit| {
      let digit_to_process = if overlflowed {
        *next_digit + 1
      } else {
        *next_digit
      };

      let (result_digit, result_overflowed) = match digit_to_process {
        5 => ("0", true),
        4 => ("-", true),
        3 => ("=", true),
        2 => ("2", false),
        1 => ("1", false),
        0 => ("0", false),
        _ => panic!("unknown digit")
      };

      let mut updated_digits = digits.clone();
      updated_digits.push(result_digit.to_string());

      (updated_digits, result_overflowed)
    });

  let almost_string_rep = converted_digits
    .iter()
    .rev()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join("");

  if final_overflowed {
    format!("1{}", almost_string_rep)
  } else {
    almost_string_rep
  }
}

fn snafu_to_decimal(snafu: &String) -> u64 {
  let chars = snafu.chars().collect::<Vec<char>>();
  let mut power = chars.len() as u32;
  let mut result: i64 = 0;
  for c in chars {
    power -= 1;
    let _ = (5 as i64).pow(3);

    result += match c {
      '0' => ((5 as i64).pow(power)) * 0,
      '1' => ((5 as i64).pow(power)) * 1,
      '2' => ((5 as i64).pow(power)) * 2,
      '-' => ((5 as i64).pow(power)) * -1,
      '=' => ((5 as i64).pow(power)) * -2,
      _ => panic!("unknown digit")
    };
  }

  result as u64
}

#[cfg(test)]
mod tests {
  use crate::day25::{decimal_to_snafu, snafu_to_decimal};

  #[test]
  fn test_snafu_to_decimal() {
    let pairs = get_decimal_to_snafu_tuples();
    for p in pairs {
      let converted_from_snafu = snafu_to_decimal(&p.1);
      assert_eq!(converted_from_snafu, p.0);
    }
  }

  #[test]
  fn test_decimal_to_snafu() {
    let pairs = get_decimal_to_snafu_tuples();
    for p in pairs {
      let converted_to_snafu = decimal_to_snafu(&p.0);
      println!("  => {}", converted_to_snafu);
      assert_eq!(converted_to_snafu, p.1);
    }
  }

  #[test]
  fn verify_sample_snafu_numbers_sum_to_correct_value() {
    let snafu_input = part_1_sample_snafu_numbers();

    let sum: u64 = snafu_input
      .iter()
      .map(|sn| snafu_to_decimal(sn))
      .sum();

    assert_eq!(sum, 4890);

    let snafu = decimal_to_snafu(&4890);
    assert_eq!(snafu, "2=-1=0".to_string());
  }

  fn get_decimal_to_snafu_tuples() -> Vec<(u64, String)> {
    vec![
      (1, "1".to_string()),
      (2, "2".to_string()),
      (3, "1=".to_string()),
      (4, "1-".to_string()),
      (5, "10".to_string()),
      (6, "11".to_string()),
      (7, "12".to_string()),
      (8, "2=".to_string()),
      (9, "2-".to_string()),
      (10, "20".to_string()),
      (15, "1=0".to_string()),
      (20, "1-0".to_string()),
      (2022, "1=11-2".to_string()),
      (12345, "1-0---0".to_string()),
      (314159265, "1121-1110-1=0".to_string()),
    ]
  }

  fn part_1_sample_snafu_numbers() -> Vec<String> {
    vec![
      "1=-0-2".to_string(),
      "12111".to_string(),
      "2=0=".to_string(),
      "21".to_string(),
      "2=01".to_string(),
      "111".to_string(),
      "20012".to_string(),
      "112".to_string(),
      "1=-1=".to_string(),
      "1-12".to_string(),
      "12".to_string(),
      "1=".to_string(),
      "122".to_string(),
    ]
  }
}
