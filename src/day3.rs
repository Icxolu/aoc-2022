use aoc_runner_derive::{aoc, aoc_generator};

type Item = char;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rucksack {
    items: Vec<Item>,
    compartment_1: Vec<Item>,
    compartment_2: Vec<Item>,
}

pub struct Groups {
    bag_1: Vec<Item>,
    bag_2: Vec<Item>,
    bag_3: Vec<Item>,
}

#[aoc_generator(day3, part1)]
pub fn input_gen_part1(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|e| {
            let (c1, c2) = e.split_at(e.len() / 2);
            Rucksack {
                items: e.chars().collect(),
                compartment_1: c1.chars().collect(),
                compartment_2: c2.chars().collect(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .filter_map(|rucksack| {
            rucksack
                .compartment_1
                .iter()
                .find(|&item| rucksack.compartment_2.iter().any(|i| i == item))
        })
        .map(|&item| match item {
            'a'..='z' => u32::from(item) - u32::from('a') + 1,
            'A'..='Z' => u32::from(item) - u32::from('A') + 27,
            _ => unreachable!(),
        })
        .sum()
}

#[aoc_generator(day3, part2)]
pub fn input_gen_part2(input: &str) -> Vec<Groups> {
    let bags = input_gen_part1(input);
    bags.chunks_exact(3)
        .map(|group| Groups {
            bag_1: group[0].items.clone(),
            bag_2: group[1].items.clone(),
            bag_3: group[2].items.clone(),
        })
        .collect()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Groups]) -> u32 {
    input
        .iter()
        .filter_map(|group| {
            group.bag_1.iter().find(|&item| {
                group.bag_2.iter().any(|i| i == item) && group.bag_3.iter().any(|i| i == item)
            })
        })
        .map(|&item| match item {
            'a'..='z' => u32::from(item) - u32::from('a') + 1,
            'A'..='Z' => u32::from(item) - u32::from('A') + 27,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_gen_part1, input_gen_part2, solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let parsed = input_gen_part1(input);
        let priority = solve_part1(&parsed);
        assert_eq!(priority, 157)
    }

    #[test]
    fn test_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let parsed = input_gen_part2(input);
        let priority = solve_part2(&parsed);
        assert_eq!(priority, 70)
    }
}
