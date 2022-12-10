use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct Tree {
    height: u32,
}

pub struct Grid {
    dim_x: usize,
    dim_y: usize,
    content: Vec<Tree>,
}

impl Grid {
    fn is_border(&self, (y, x): (usize, usize)) -> bool {
        x == 0 || y == 0 || x == self.dim_x - 1 || y == self.dim_y - 1
    }

    fn check_top(&self, (y, x): (usize, usize), tree: &Tree) -> Option<usize> {
        (0..y)
            .map(|y| y * self.dim_x + x)
            .map(|i| &self.content[i])
            .rev()
            .position(|t| t.height >= tree.height)
            .map(|p| p + 1)
    }

    fn check_bottom(&self, (y, x): (usize, usize), tree: &Tree) -> Option<usize> {
        (y + 1..self.dim_y)
            .map(|y| y * self.dim_x + x)
            .map(|i| &self.content[i])
            .position(|t| t.height >= tree.height)
            .map(|p| p + 1)
    }

    fn check_left(&self, (y, x): (usize, usize), tree: &Tree) -> Option<usize> {
        (0..x)
            .map(|x| y * self.dim_x + x)
            .map(|i| &self.content[i])
            .rev()
            .position(|t| t.height >= tree.height)
            .map(|p| p + 1)
    }

    fn check_right(&self, (y, x): (usize, usize), tree: &Tree) -> Option<usize> {
        (x + 1..self.dim_x)
            .map(|x| y * self.dim_x + x)
            .map(|i| &self.content[i])
            .position(|t| t.height >= tree.height)
            .map(|p| p + 1)
    }

    fn is_visible(&self, pos: (usize, usize)) -> bool {
        let tree = self.get(pos);
        self.is_border(pos)
            || self.check_top(pos, tree).is_none()
            || self.check_bottom(pos, tree).is_none()
            || self.check_left(pos, tree).is_none()
            || self.check_right(pos, tree).is_none()
    }

    fn get(&self, (y, x): (usize, usize)) -> &Tree {
        &self.content[y * self.dim_x + x]
    }

    fn score(&self, pos @ (y, x): (usize, usize)) -> usize {
        if self.is_border(pos) {
            return 0;
        }

        let tree = self.get(pos);
        self.check_top(pos, tree).unwrap_or(y)
            * self.check_bottom(pos, tree).unwrap_or(self.dim_y - y - 1)
            * self.check_left(pos, tree).unwrap_or(x)
            * self.check_right(pos, tree).unwrap_or(self.dim_x - x - 1)
    }
}

pub fn input_gen(input: &str) -> Grid {
    let rows = input.lines().count();
    let cols = input
        .chars()
        .position(|c| c == '\n')
        .unwrap_or_else(|| input.chars().count());

    let trees = input
        .lines()
        .flat_map(|line| {
            line.chars().map(move |c| Tree {
                height: c.to_digit(10).unwrap(),
            })
        })
        .collect();
    Grid {
        dim_x: cols,
        dim_y: rows,
        content: trees,
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    let grid @ Grid { dim_x, dim_y, .. } = input_gen(input);

    (0..dim_y)
        .map(|y| (0..dim_x).filter(|&x| grid.is_visible((y, x))).count())
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> Option<usize> {
    let grid @ Grid { dim_x, dim_y, .. } = input_gen(input);

    (0..dim_y)
        .filter_map(|y| (0..dim_x).map(|x| grid.score((y, x))).max())
        .max()
}

#[cfg(test)]
mod tests {
    use super::{input_gen, solve_part1, solve_part2};

    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let vis = solve_part1(INPUT);
        assert_eq!(vis, 21);
        Ok(())
    }

    #[test]
    fn test_score() -> anyhow::Result<()> {
        let grid = input_gen(INPUT);
        let score = grid.score((1, 2));
        assert_eq!(score, 4);

        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let score = solve_part2(INPUT);
        assert_eq!(score, Some(8));
        Ok(())
    }
}
