use aoc_runner_derive::aoc;
use std::collections::{HashSet, VecDeque};

pub struct Grid {
    dim_x: usize,
    dim_y: usize,
    end: usize,
    content: Vec<u32>,
}

impl Grid {
    fn get(&self, (x, y): (usize, usize)) -> u32 {
        self.content[y * self.dim_x + x]
    }

    fn lowest(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.content
            .iter()
            .enumerate()
            .filter(|&(_, e)| *e == 0)
            .map(|(i, _)| (i % self.dim_x, i / self.dim_x))
    }

    fn find_path(&self, start: (usize, usize)) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<(usize, _)> = VecDeque::new();
        queue.push_back((0, start));
        visited.insert(start);

        while let Some((count, pos @ (x, y))) = queue.pop_front() {
            for n_pos @ (nx, ny) in [
                (x + 1, y),
                (x.saturating_sub(1), y),
                (x, y + 1),
                (x, y.saturating_sub(1)),
            ] {
                if nx < self.dim_x
                    && ny < self.dim_y
                    && self.get(n_pos) <= self.get(pos) + 1
                    && visited.insert(n_pos)
                {
                    if ny * self.dim_x + nx == self.end {
                        return Some(count + 1);
                    }
                    queue.push_back((count + 1, n_pos))
                }
            }
        }
        None
    }
}

pub fn input_gen(input: &str) -> ((usize, usize), Grid) {
    let rows = input.lines().count();
    let cols = input
        .chars()
        .position(|c| c == '\n')
        .unwrap_or_else(|| input.chars().count());

    let elevations = input
        .lines()
        .flat_map(|line| {
            line.chars().map(move |c| match c {
                'a'..='z' => u32::from(c) - u32::from('a'),
                'S' => 0,
                'E' => 25,
                _ => panic!(),
            })
        })
        .collect();

    let start = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == 'S')
        .unwrap();
    let end = input
        .chars()
        .filter(|&c| c != '\n')
        .position(|c| c == 'E')
        .unwrap();

    let start_x = start % cols;
    let start_y = start / cols;

    (
        (start_x, start_y),
        Grid {
            dim_x: cols,
            dim_y: rows,
            end,
            content: elevations,
        },
    )
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> Option<usize> {
    let (start, grid) = input_gen(input);
    grid.find_path(start)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> Option<usize> {
    let (_, grid) = input_gen(input);
    grid.lowest().filter_map(|p| grid.find_path(p)).min()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let count = solve_part1(INPUT);
        assert_eq!(count, Some(31));
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let count = solve_part2(INPUT);
        assert_eq!(count, Some(29));
        Ok(())
    }
}
