use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

struct Move {
    count: usize,
    from_id: usize,
    to_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

#[aoc_generator(day5)]
pub fn input_gen(input: &str) -> anyhow::Result<Puzzle> {
    let num_stacks = (input.chars().take_while(|&c| c != '\n').count() + 1) / 4;
    let re = (0..num_stacks).fold(String::new(), |mut string, _| {
        string += r"(?:\[([A-Z])\]|   )[ ]?";
        string
    });
    let re = Regex::new(&re)?;
    let stacks = input
        .lines()
        .take_while(|line| !line.starts_with("move"))
        .filter_map(|line| re.captures(line))
        .fold(vec![VecDeque::new(); num_stacks], |mut stacks, capture| {
            for (i, stack) in stacks.iter_mut().enumerate() {
                if let Some(c) = capture.get(i + 1) {
                    stack.push_front(c.as_str().chars().next().unwrap());
                }
            }
            stacks
        });

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    let moves: Vec<_> = input
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .filter_map(|line| {
            let captures = re.captures(line)?;
            Some(Move {
                count: captures[1].parse().ok()?,
                from_id: captures[2].parse::<usize>().ok()? - 1,
                to_id: captures[3].parse::<usize>().ok()? - 1,
            })
        })
        .collect();

    Ok(Puzzle { stacks, moves })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Puzzle) -> String {
    let mut stacks = input.stacks.clone();
    for m in input.moves.iter() {
        for _ in 0..m.count {
            let item = stacks[m.from_id].pop_back().unwrap();
            stacks[m.to_id].push_back(item);
        }
    }

    stacks
        .into_iter()
        .fold(String::new(), |mut string, mut stack| {
            string.push(stack.pop_back().unwrap());
            string
        })
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Puzzle) -> String {
    let mut stacks = input.stacks.clone();
    for m in input.moves.iter() {
        let len = stacks[m.from_id].len();
        let other = stacks[m.from_id].split_off(len - m.count);
        stacks[m.to_id].extend(other);
    }

    stacks
        .into_iter()
        .fold(String::new(), |mut string, mut stack| {
            string.push(stack.pop_back().unwrap());
            string
        })
}

#[cfg(test)]
mod tests {
    use super::{input_gen, solve_part1, solve_part2};

    const INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let parsed = input_gen(INPUT)?;
        let crates = solve_part1(&parsed);
        assert_eq!(crates, "CMZ");
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let parsed = input_gen(INPUT)?;
        let crates = solve_part2(&parsed);
        assert_eq!(crates, "MCD");
        Ok(())
    }
}
