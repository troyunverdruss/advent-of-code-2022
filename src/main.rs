mod day01;
mod utils;

fn main() {
  let solution_day01_part1 = day01::part_one();
  println!("Day 1, part 1: elf with max calories has this total: {}", solution_day01_part1);
  let solution_day01_part2 = day01::part_two();
  println!("Day 1, part 2: top 3 elves with max calories has this total: {}", solution_day01_part2);
}
