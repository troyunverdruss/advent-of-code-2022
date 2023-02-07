extern crate core;

mod day01;
mod utils;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

fn main() {
  let day_to_solve = 24;

  if day_to_solve == 0 || day_to_solve == 1 {
    let solution_day01_part1 = day01::part_one();
    println!("Day 1, part 1: elf with max calories has this total: {}", solution_day01_part1);
    let solution_day01_part2 = day01::part_two();
    println!("Day 1, part 2: top 3 elves with max calories has this total: {}", solution_day01_part2);
  }

  if day_to_solve == 0 || day_to_solve == 2 {
    let solution_day02_part1 = day02::part_one();
    println!("Day 2, part 1: your total score according to strategy 1: {}", solution_day02_part1);
    let solution_day02_part2 = day02::part_two();
    println!("Day 2, part 2: your total score according to strategy 2: {}", solution_day02_part2);
  }

  if day_to_solve == 0 || day_to_solve == 3 {
    let solution_day03_part1 = day03::part_one();
    println!("Day 3, part 1: sum of priorities of rucksack halves: {}", solution_day03_part1);
    let solution_day03_part2 = day03::part_two();
    println!("Day 3, part 2: sum of priorities of rucksack triples: {}", solution_day03_part2);
  }

  if day_to_solve == 0 || day_to_solve == 4 {
    let solution_day04_part1 = day04::part_one();
    println!("Day 4, part 1: count of pairs with full overlap: {}", solution_day04_part1);
    let solution_day04_part2 = day04::part_two();
    println!("Day 4, part 2: count of pairs with partial overlap : {}", solution_day04_part2);
  }

  if day_to_solve == 0 || day_to_solve == 5 {
    let solution_day05_part1 = day05::part_one();
    println!("Day 5, part 1: order of CrateMover 9000 final stacks: {}", solution_day05_part1);
    let solution_day05_part2 = day05::part_two();
    println!("Day 5, part 2: order of CrateMover 9001 final stacks: {}", solution_day05_part2);
  }

  if day_to_solve == 0 || day_to_solve == 6 {
    let solution_day06_part1 = day06::part_one();
    println!("Day 6, part 1: chars processed to reach end of first start of packet marker: {}", solution_day06_part1);
    let solution_day06_part2 = day06::part_two();
    println!("Day 6, part 2: chars processed to reach end of first start of message marker: {}", solution_day06_part2);
  }

  if day_to_solve == 0 || day_to_solve == 7 {
    let solution_day07_part1 = day07::part_one();
    println!("Day 7, part 1: sum of sizes for all dirs less than 100000: {}", solution_day07_part1);
    let solution_day07_part2 = day07::part_two();
    println!("Day 7, part 2: smallest size of dir that would free up enough space: {}", solution_day07_part2);
  }

  if day_to_solve == 0 || day_to_solve == 8 {
    let solution_day08_part1 = day08::part_one();
    println!("Day 8, part 1: total number of trees visible from outside the grid: {}", solution_day08_part1);
    let solution_day08_part2 = day08::part_two();
    println!("Day 8, part 2: highest scenic score for any tree: {}", solution_day08_part2);
  }

  if day_to_solve == 0 || day_to_solve == 9 {
    let solution_day09_part1 = day09::part_one();
    println!("Day 9, part 1: Locations that the tail has visited: {}", solution_day09_part1);
    let solution_day09_part2 = day09::part_two();
    println!("Day 9, part 2: Locations that the tail has visited: {}", solution_day09_part2);
  }

  if day_to_solve == 0 || day_to_solve == 10 {
    let solution_day10_part1 = day10::part_one();
    println!("Day 10, part 1: Sum of interesting signal strengths: {}", solution_day10_part1);
    println!("Day 10, part 2: See above printout and read the 8 letters");
  }

  if day_to_solve == 0 || day_to_solve == 11 {
    let solution_day11_part1 = day11::part_one();
    println!("Day 11, part 1: Product of most items passed over 20 rounds: {}", solution_day11_part1);
    let solution_day11_part2 = day11::part_two();
    println!("Day 11, part 2: Product of most items passed over 10000 rounds: {}", solution_day11_part2);
  }

  if day_to_solve == 0 || day_to_solve == 12 {
    let solution_day12_part1 = day12::part_one();
    println!("Day 12, part 1: Shortest path to summit: {}", solution_day12_part1);
    let solution_day12_part2 = day12::part_two();
    println!("Day 12, part 2: Shortest possible path from any start: {}", solution_day12_part2);
  }

  if day_to_solve == 0 || day_to_solve == 13 {
    let solution_day13_part1 = day13::part_one();
    println!("Day 13, part 1: Sum of already sorted indices: {}", solution_day13_part1);
    let solution_day13_part2 = day13::part_two();
    println!("Day 13, part 2: Product of divider packet indices: {}", solution_day13_part2);
  }

  if day_to_solve == 0 || day_to_solve == 14 {
    let solution_day14_part1 = day14::part_one();
    println!("Day 14, part 1: Total number of settled sand grains with abyss: {}", solution_day14_part1);
    let solution_day14_part2 = day14::part_two();
    println!("Day 14, part 2: Total number of settled sand grains with infinite floor: {}", solution_day14_part2);
  }

  if day_to_solve == 0 || day_to_solve == 15 {
    let solution_day15_part1 = day15::part_one();
    println!("Day 15, part 1: Invalid locations in row 2000000: {}", solution_day15_part1);
    let solution_day15_part2 = day15::part_two();
    println!("Day 15, part 2: Tuning frequency of missing beacon: {}", solution_day15_part2);
  }

  if day_to_solve == 0 || day_to_solve == 16 {
    let solution_day16_part1 = day16::part_one();
    println!("Day 16, part 1: Most pressure that can be released: {}", solution_day16_part1);
    let solution_day16_part2 = day16::part_two();
    println!("Day 16, part 2: Most pressure that can be released with an elephant helping: {}", solution_day16_part2);
  }

  if day_to_solve == 0 || day_to_solve == 17 {
    let solution_day17_part1 = day17::part_one();
    println!("Day 17, part 1: Height after 2022 blocks: {}", solution_day17_part1);
    let solution_day17_part2 = day17::part_two();
    println!("Day 17, part 2: Height after 1000000000000 blocks: {}", solution_day17_part2);
  }

  if day_to_solve == 0 || day_to_solve == 18 {
    let solution_day18_part1 = day18::part_one();
    println!("Day 18, part 1: Total surface area: {}", solution_day18_part1);
    let solution_day18_part2 = day18::part_two();
    println!("Day 18, part 2: Exterior surface area: {}", solution_day18_part2);
  }

  if day_to_solve == 0 || day_to_solve == 19 {
    let solution_day19_part1 = day19::part_one();
    println!("Day 19, part 1: Sum of quality scores: {}", solution_day19_part1);
    let solution_day19_part2 = day19::part_two();
    println!("Day 19, part 2: Product of first 3 blueprints: {}", solution_day19_part2);
  }

  if day_to_solve == 0 || day_to_solve == 20 {
    let solution_day20_part1 = day20::part_one();
    println!("Day 20, part 1: Grove coordinates: {}", solution_day20_part1);
    let solution_day20_part2 = day20::part_two();
    println!("Day 20, part 2: Grove coordinates: {}", solution_day20_part2);
  }

  if day_to_solve == 0 || day_to_solve == 21 {
    let solution_day21_part1 = day21::part_one();
    println!("Day 21, part 1: Root yells: {}", solution_day21_part1);
    let solution_day21_part2 = day21::part_two();
    println!("Day 21, part 2: Value needed to make root equal: {}", solution_day21_part2);
  }

  if day_to_solve == 0 || day_to_solve == 22 {
    let solution_day22_part1 = day22::part_one();
    println!("Day 22, part 1: Final password (2d): {}", solution_day22_part1);
    let solution_day22_part2 = day22::part_two();
    println!("Day 22, part 2: Final password (3d): {}", solution_day22_part2);
  }

  if day_to_solve == 0 || day_to_solve == 23 {
    let solution_day23_part1 = day23::part_one();
    println!("Day 23, part 1: Empty ground tiles: {}", solution_day23_part1);
    let solution_day23_part2 = day23::part_two();
    println!("Day 23, part 2: ????: {}", solution_day23_part2);
  }

  if day_to_solve == 0 || day_to_solve == 24 {
    let solution_day24_part1 = day24::part_one();
    println!("Day 24, part 1: Empty ground tiles: {}", solution_day24_part1);
    let solution_day24_part2 = day24::part_two();
    println!("Day 24, part 2: ????: {}", solution_day24_part2);
  }
}
