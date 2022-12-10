use aoc_runner_derive::aoc;
use lending_iterator::prelude::*;
use std::collections::HashSet;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Move {
    dir: Direction,
    count: usize,
}

pub fn input_gen(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.lines().filter_map(|line| {
        let (dir, count) = line.split_once(' ')?;
        let dir = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return None,
        };

        Some(Move {
            dir,
            count: count.parse().ok()?,
        })
    })
}

struct Rope<const N: usize> {
    knots: [(isize, isize); N],
    tail_positions: HashSet<(isize, isize)>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            knots: [(0, 0); N],
            tail_positions: HashSet::new(),
        }
    }

    fn step(&mut self, dir: &Direction) {
        use Direction::*;

        match dir {
            Left => self.knots[0].0 -= 1,
            Right => self.knots[0].0 += 1,
            Up => self.knots[0].1 += 1,
            Down => self.knots[0].1 -= 1,
        }

        // iterate over the rope as overlapping (head, tail) knot pairs, we move
        // the tail, which becomes the head of the next pair
        let mut iter = self.knots.windows_mut::<2>();
        while let Some(&mut [head, ref mut tail]) = iter.next() {
            // if not in same row or column
            if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
                // move diagonal towards head
                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
            }
        }

        // keep track of visited tail positions
        self.tail_positions.insert(self.knots[N - 1]);
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.count {
            self.step(&m.dir)
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    let moves = input_gen(input);
    moves
        .fold(Rope::<2>::new(), |mut rope, m| {
            rope.apply_move(&m);
            rope
        })
        .tail_positions
        .len()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    let moves = input_gen(input);
    moves
        .fold(Rope::<10>::new(), |mut rope, m| {
            rope.apply_move(&m);
            rope
        })
        .tail_positions
        .len()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const INPUT_2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let count = solve_part1(INPUT);
        assert_eq!(count, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let count = solve_part2(INPUT);
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn test_part2_2() -> anyhow::Result<()> {
        let count = solve_part2(INPUT_2);
        assert_eq!(count, 36);
        Ok(())
    }
}
