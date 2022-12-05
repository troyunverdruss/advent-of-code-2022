mod day01;
mod utils;
mod day02;
mod day03;
mod day04;

fn main() {
  let solution_day01_part1 = day01::part_one();
  println!("Day 1, part 1: elf with max calories has this total: {}", solution_day01_part1);
  let solution_day01_part2 = day01::part_two();
  println!("Day 1, part 2: top 3 elves with max calories has this total: {}", solution_day01_part2);

  let solution_day02_part1 = day02::part_one();
  println!("Day 2, part 1: your total score: {}", solution_day02_part1);
  let solution_day02_part2 = day02::part_two();
  println!("Day 2, part 2: your total score: {}", solution_day02_part2);

  let solution_day03_part1 = day03::part_one();
  println!("Day 3, part 1: score for all common letters: {}", solution_day03_part1);
  let solution_day03_part2 = day03::part_two();
  println!("Day 3, part 2: your total score: {}", solution_day03_part2);

  let solution_day04_part1 = day04::part_one();
  println!("Day 4, part 1: score for all common letters: {}", solution_day04_part1);
  let solution_day04_part2 = day04::part_two();
  println!("Day 4, part 2: your total score: {}", solution_day04_part2);
}
