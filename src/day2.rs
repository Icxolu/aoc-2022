use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    Win,
    Loss,
    Draw,
}

impl Ending {
    pub fn score(&self) -> usize {
        match self {
            Ending::Win => 6,
            Ending::Loss => 0,
            Ending::Draw => 3,
        }
    }

    pub fn symbol(&self, other: Symbol) -> Symbol {
        match (self, other) {
            (Ending::Win, Symbol::Rock) => Symbol::Paper,
            (Ending::Win, Symbol::Paper) => Symbol::Scissors,
            (Ending::Win, Symbol::Scissors) => Symbol::Rock,
            (Ending::Loss, Symbol::Rock) => Symbol::Scissors,
            (Ending::Loss, Symbol::Paper) => Symbol::Rock,
            (Ending::Loss, Symbol::Scissors) => Symbol::Paper,
            (Ending::Draw, s) => s,
        }
    }
}

impl Symbol {
    pub fn score(&self) -> usize {
        match self {
            Symbol::Rock => 1,
            Symbol::Paper => 2,
            Symbol::Scissors => 3,
        }
    }

    pub fn ending(&self, other: Symbol) -> Ending {
        match (self, other) {
            (Symbol::Rock, Symbol::Rock)
            | (Symbol::Paper, Symbol::Paper)
            | (Symbol::Scissors, Symbol::Scissors) => Ending::Draw,
            (Symbol::Rock, Symbol::Scissors)
            | (Symbol::Scissors, Symbol::Paper)
            | (Symbol::Paper, Symbol::Rock) => Ending::Win,
            (Symbol::Rock, Symbol::Paper)
            | (Symbol::Paper, Symbol::Scissors)
            | (Symbol::Scissors, Symbol::Rock) => Ending::Loss,
        }
    }
}

#[aoc_generator(day2, part1)]
pub fn input_gen_part1(input: &str) -> Vec<(Symbol, Symbol)> {
    input
        .lines()
        .map(|e| {
            let (first, second) = e.split_once(" ").unwrap();
            let other = match first {
                "A" => Symbol::Rock,
                "B" => Symbol::Paper,
                "C" => Symbol::Scissors,
                _ => unreachable!(),
            };
            let we = match second {
                "X" => Symbol::Rock,
                "Y" => Symbol::Paper,
                "Z" => Symbol::Scissors,
                _ => unreachable!(),
            };
            (other, we)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Symbol, Symbol)]) -> usize {
    input
        .iter()
        .copied()
        .map(|(other, this)| this.score() + this.ending(other).score())
        .sum()
}

#[aoc_generator(day2, part2)]
pub fn input_gen_part2(input: &str) -> Vec<(Symbol, Ending)> {
    input
        .lines()
        .map(|e| {
            let (first, second) = e.split_once(" ").unwrap();
            let other = match first {
                "A" => Symbol::Rock,
                "B" => Symbol::Paper,
                "C" => Symbol::Scissors,
                _ => unreachable!(),
            };
            let we = match second {
                "X" => Ending::Loss,
                "Y" => Ending::Draw,
                "Z" => Ending::Win,
                _ => unreachable!(),
            };
            (other, we)
        })
        .collect()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Symbol, Ending)]) -> usize {
    input
        .iter()
        .copied()
        .map(|(other, ending)| ending.score() + ending.symbol(other).score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_gen_part1, input_gen_part2, solve_part1, solve_part2, Ending, Symbol};

    #[test]
    fn test_part1() {
        let input = "A Y\nB X\nC Z";
        let parsed = input_gen_part1(input);
        assert_eq!(
            parsed,
            vec![
                (Symbol::Rock, Symbol::Paper),
                (Symbol::Paper, Symbol::Rock),
                (Symbol::Scissors, Symbol::Scissors)
            ]
        );
        let score = solve_part1(&parsed);
        assert_eq!(score, 15)
    }

    #[test]
    fn test_part2() {
        let input = "A Y\nB X\nC Z";
        let parsed = input_gen_part2(input);
        assert_eq!(
            parsed,
            vec![
                (Symbol::Rock, Ending::Draw),
                (Symbol::Paper, Ending::Loss),
                (Symbol::Scissors, Ending::Win)
            ]
        );
        let score = solve_part2(&parsed);
        assert_eq!(score, 12)
    }
}
