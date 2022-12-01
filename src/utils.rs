use std::fs;

pub fn lines_into_numbers(lines: &Vec<String>) -> Vec<u64> {
  lines
    .iter()
    .map(|l| l.parse::<u64>())
    .map(|r|
      match r {
        Err(e) => panic!("Unable to parse to number {:?}", e),
        Ok(n) => n
      }
    )
    .collect()
}

pub fn split_chunks_into_line_groups(chunks: &Vec<String>) -> Vec<Vec<String>> {
  chunks
    .iter()
    .map(|c|
      c.split('\n')
        .map(String::from)
        .filter(|s| !s.is_empty())
        .collect()
    )
    .collect()
}

pub fn read_chunks(filename: &str, splitter: &str) -> Vec<String> {
  let input = read_input(filename);
  input.split(splitter)
    .map(String::from)
    .collect()
}

fn read_input(filename: &str) -> String {
  let file_path = format!("inputs/{}", filename);
  let input_string = fs::read_to_string(file_path);

  match input_string {
    Err(error) => panic!("Unable to read input file {:?}", error),
    Ok(s) => s
  }
}