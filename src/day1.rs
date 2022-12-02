use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Elf {
    calories: Vec<usize>,
}

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|c| usize::from_str_radix(c, 10).unwrap())
                .collect()
        })
        .map(|c| Elf { calories: c })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> Option<usize> {
    input.iter().map(|elf| elf.calories.iter().sum()).max()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> usize {
    let mut input = input.to_vec();
    input.sort_unstable_by_key(|e| e.calories.iter().sum::<usize>());
    input
        .iter()
        .map(|e| e.calories.iter().sum::<usize>())
        .rev()
        .take(3)
        .sum()
}
