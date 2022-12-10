use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    NoOp,
    AddX(i64),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

pub struct Cpu<I> {
    program: I,
    x: i64,
    last_instruction: Option<(Instruction, usize)>,
}

impl<I> Cpu<I>
where
    I: Iterator<Item = Instruction>,
{
    fn new(program: I) -> Self {
        Self {
            program,
            x: 1,
            last_instruction: None,
        }
    }

    fn single_step(&mut self) -> i64 {
        let x = self.x;

        // we either take the last unfinished instruction or we take the next on
        // of the program, if there is none, the program ended
        let Some((inst, left_cycles)) = self.last_instruction.take().or_else(|| {
            self.program.by_ref().next().map(|inst| (inst, inst.cycles()-1))
        }) else {return x};

        // if there are still cycles left, we wait
        if left_cycles > 0 {
            self.last_instruction = Some((inst, left_cycles - 1))
        } else {
            // otherwise we execute the instruction
            match inst {
                Instruction::NoOp => {}
                Instruction::AddX(rhs) => self.x += rhs,
            }
        }

        x
    }

    fn step(&mut self, cycles: usize) -> i64 {
        (0..cycles).fold(0, |_, _| self.single_step())
    }
}

struct Crt<I> {
    cpu: Cpu<I>,
}

impl<I> Crt<I>
where
    I: Iterator<Item = Instruction>,
{
    fn new(cpu: Cpu<I>) -> Self {
        Self { cpu }
    }

    fn draw_line(&mut self) -> impl Iterator<Item = char> + '_ {
        (0..40)
            .map(|i| {
                let x = self.cpu.single_step();
                if (i - 1..=i + 1).contains(&x) {
                    '#'
                } else {
                    '.'
                }
            })
            .chain(std::iter::once('\n'))
    }

    fn draw(&mut self) -> String {
        let mut result = String::new();

        for _ in 0..6 {
            self.draw_line().for_each(|c| result.push(c));
        }

        result
    }
}

fn input_gen(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().filter_map(|line| {
        if line.starts_with("noop") {
            Some(Instruction::NoOp)
        } else if line.starts_with("addx") {
            let (_, val) = line.split_once(' ')?;
            val.parse().map(Instruction::AddX).ok()
        } else {
            None
        }
    })
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let program = input_gen(input);
    let mut cpu = Cpu::new(program);
    (20..=220)
        .step_by(40)
        .enumerate()
        .map(|(i, cycles)| {
            let x = cpu.step(if i == 0 { 20 } else { 40 });
            cycles * x
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> String {
    let program = input_gen(input);
    let cpu = Cpu::new(program);
    let mut crt = Crt::new(cpu);
    crt.draw()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    const SCREEN: &str = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let sum = solve_part1(INPUT);
        assert_eq!(sum, 13140);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let screen = solve_part2(INPUT);
        assert_eq!(screen, SCREEN);
        Ok(())
    }
}
