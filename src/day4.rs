use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    range_1: RangeInclusive<u32>,
    range_2: RangeInclusive<u32>,
}

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|e| {
            let (r1, r2) = e.split_once(',').unwrap();
            let range = |r: &str| {
                r.split_once('-')
                    .map(|(a, b)| {
                        (
                            u32::from_str_radix(a, 10).unwrap(),
                            u32::from_str_radix(b, 10).unwrap(),
                        )
                    })
                    .map(|(a, b)| a..=b)
                    .unwrap()
            };
            Pair {
                range_1: range(r1),
                range_2: range(r2),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pair]) -> usize {
    input
        .iter()
        .filter_map(|pair| {
            if (pair.range_1.start() <= pair.range_2.start()
                && pair.range_1.end() >= pair.range_2.end())
                || (pair.range_2.start() <= pair.range_1.start()
                    && pair.range_2.end() >= pair.range_1.end())
            {
                Some(pair)
            } else {
                None
            }
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pair]) -> usize {
    input
        .iter()
        .filter_map(|pair| {
            if pair.range_1.start().max(pair.range_2.start())
                <= pair.range_1.end().min(pair.range_2.end())
            {
                Some(pair)
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{input_gen, solve_part1, solve_part2};

    #[test]
    fn test_part1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let parsed = input_gen(input);
        let count = solve_part1(&parsed);
        assert_eq!(count, 2)
    }

    #[test]
    fn test_part2() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let parsed = input_gen(input);
        let priority = solve_part2(&parsed);
        assert_eq!(priority, 4)
    }
}
