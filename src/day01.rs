use crate::utils::read_chunks;
use crate::utils::split_chunks_into_line_groups;
use crate::utils::lines_into_numbers;

pub fn part_one() -> u64 {
  let elf_sums = get_elf_sums();
  let opt_max = elf_sums.iter().max();

  let max = match opt_max {
    None => panic!("Unable to find any sum"),
    Some(max) => *max
  };
  max
}

pub fn part_two() -> u64 {
  let mut elf_sums = get_elf_sums();
  elf_sums.sort();
  elf_sums.iter().rev().take(3).sum()
}

fn get_elf_sums() -> Vec<u64> {
  let chunks = read_chunks("day01.txt", "\n\n");
  let line_groups = split_chunks_into_line_groups(&chunks);

  line_groups
    .iter()
    .map(|lg| lines_into_numbers(&lg))
    .map(|f| f.iter().sum::<u64>())
    .collect()
}

