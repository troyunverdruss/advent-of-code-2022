mod day01;
mod utils;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
  let solution_day01_part1 = day01::part_one();
  println!("Day 1, part 1: elf with max calories has this total: {}", solution_day01_part1);
  let solution_day01_part2 = day01::part_two();
  println!("Day 1, part 2: top 3 elves with max calories has this total: {}", solution_day01_part2);

  let solution_day02_part1 = day02::part_one();
  println!("Day 2, part 1: your total score according to strategy 1: {}", solution_day02_part1);
  let solution_day02_part2 = day02::part_two();
  println!("Day 2, part 2: your total score according to strategy 2: {}", solution_day02_part2);

  let solution_day03_part1 = day03::part_one();
  println!("Day 3, part 1: sum of priorities of rucksack halves: {}", solution_day03_part1);
  let solution_day03_part2 = day03::part_two();
  println!("Day 3, part 2: sum of priorities of rucksack triples: {}", solution_day03_part2);

  let solution_day04_part1 = day04::part_one();
  println!("Day 4, part 1: count of pairs with full overlap: {}", solution_day04_part1);
  let solution_day04_part2 = day04::part_two();
  println!("Day 4, part 2: count of pairs with partial overlap : {}", solution_day04_part2);

  let solution_day05_part1 = day05::part_one();
  println!("Day 5, part 1: order of CrateMover 9000 final stacks: {}", solution_day05_part1);
  let solution_day05_part2 = day05::part_two();
  println!("Day 5, part 2: order of CrateMover 9001 final stacks: {}", solution_day05_part2);

  let solution_day06_part1 = day06::part_one();
  println!("Day 6, part 1: chars processed to reach end of first start of packet marker: {}", solution_day06_part1);
  let solution_day06_part2 = day06::part_two();
  println!("Day 6, part 2: chars processed to reach end of first start of message marker: {}", solution_day06_part2);

  let solution_day07_part1 = day07::part_one();
  println!("Day 7, part 1: sum of sizes for all dirs less than 100000: {}", solution_day07_part1);
  let solution_day07_part2 = day07::part_two();
  println!("Day 7, part 2: smallest size of dir that would free up enough space: {}", solution_day07_part2);
}
