use aoc_runner_derive::aoc;

#[derive(Debug, Default, Clone, Copy)]
enum Operand {
    #[default]
    Lvl,
    Factor(u32),
}

impl std::str::FromStr for Operand {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::Lvl)
        } else {
            u32::from_str(s).map(Self::Factor)
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
enum Operation {
    #[default]
    Add,
    Mul,
}

#[derive(Debug, Default)]
pub struct Monkey {
    items: Vec<u32>,
    op: Operation,
    lhs: Operand,
    rhs: Operand,
    test_divisor: u32,
    test_success: usize,
    test_failure: usize,
    inspected: usize,
}

impl Monkey {
    fn turn(&mut self, div: u32) -> impl Iterator<Item = (usize, u32)> + '_ {
        std::mem::take(&mut self.items)
            .into_iter()
            .map(|item| {
                self.inspected += 1;
                let (lhs, rhs) = match (self.lhs, self.rhs) {
                    (Operand::Lvl, Operand::Lvl) => (item, item),
                    (Operand::Lvl, Operand::Factor(f)) => (item, f),
                    (Operand::Factor(f), Operand::Lvl) => (f, item),
                    (Operand::Factor(f1), Operand::Factor(f2)) => (f1, f2),
                };
                match self.op {
                    Operation::Add => lhs + rhs,
                    Operation::Mul => lhs * rhs,
                }
            })
            .map(move |item| item / div)
            .map(|item| {
                if item % self.test_divisor == 0 {
                    (self.test_success, item)
                } else {
                    (self.test_failure, item)
                }
            })
    }
}

fn input_gen(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    input.split("\n\n").map(|chunk| {
        let mut monkey = Monkey::default();
        for line in chunk.lines() {
            if let Some(items) = line.trim().strip_prefix("Starting items: ") {
                monkey.items = items
                    .split(", ")
                    .map(|item| item.parse().unwrap())
                    .collect();
            }

            if let Some(op) = line.trim().strip_prefix("Operation: new = ") {
                let mut ops = op.split(' ');
                monkey.lhs = ops.next().unwrap().parse().unwrap();

                monkey.op = match ops.next() {
                    Some("*") => Operation::Mul,
                    Some("+") => Operation::Add,
                    _ => panic!("Unknown operation"),
                };

                monkey.rhs = ops.next().unwrap().parse().unwrap();
            }

            if let Some(divisor) = line.trim().strip_prefix("Test: divisible by ") {
                monkey.test_divisor = divisor.parse().unwrap();
            }

            if let Some(test_success) = line.trim().strip_prefix("If true: throw to monkey ") {
                monkey.test_success = test_success.parse().unwrap();
            }

            if let Some(test_failure) = line.trim().strip_prefix("If false: throw to monkey ") {
                monkey.test_failure = test_failure.parse().unwrap();
            }
        }
        monkey
    })
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut dummy = Monkey::default();
    let mut monkeys = input_gen(input).collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            std::mem::swap(&mut monkeys[i], &mut dummy);
            for (idx, item) in dummy.turn(3) {
                monkeys[idx].items.push(item)
            }
            std::mem::swap(&mut monkeys[i], &mut dummy);
        }
    }

    let (a, b) = monkeys
        .into_iter()
        .map(|m| m.inspected)
        .fold((0, 0), |(a, b), count| (a.max(b), a.min(b).max(count)));
    a * b
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut dummy = Monkey::default();
    let mut monkeys = input_gen(input).collect::<Vec<_>>();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            std::mem::swap(&mut monkeys[i], &mut dummy);
            for (idx, item) in dummy.turn(1) {
                monkeys[idx].items.push(item)
            }
            std::mem::swap(&mut monkeys[i], &mut dummy);
        }
    }

    let (a, b) = monkeys
        .into_iter()
        .map(|m| m.inspected)
        .fold((0, 0), |(a, b), count| (a.max(b), a.min(b).max(count)));
    a * b
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let lvl = solve_part1(INPUT);
        assert_eq!(lvl, 10605);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let lvl = solve_part2(INPUT);
        assert_eq!(lvl, 2713310158);
        Ok(())
    }
}
