use aoc_runner_derive::aoc;
use std::collections::{hash_map::Values, HashMap};

#[derive(Debug)]
struct Directory(HashMap<String, Node>);

impl Directory {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, key: String, value: Node) {
        self.0.insert(key, value);
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut Node> {
        self.0.get_mut(key)
    }

    fn size(&self) -> u64 {
        self.0
            .values()
            .map(|node| match node {
                Node::Directory(dir) => dir.size(),
                Node::File { size } => *size,
            })
            .sum()
    }

    fn dir_iter(&self) -> DirIter {
        DirIter {
            values: self.0.values(),
            childs: vec![],
        }
    }
}

struct DirIter<'a> {
    values: Values<'a, String, Node>,
    childs: Vec<&'a Directory>,
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.values.next() {
                Some(Node::File { .. }) => continue,
                Some(Node::Directory(dir)) => {
                    self.childs.push(dir);
                    return Some(dir);
                }
                None if !self.childs.is_empty() => {
                    self.values = self.childs.pop().unwrap().0.values();
                    return self.next();
                }
                None => return None,
            }
        }
    }
}

#[derive(Debug)]
enum Node {
    Directory(Directory),
    File { size: u64 },
}

fn input_gen(input: &mut &str, tree: &mut Directory) {
    loop {
        if input.is_empty() {
            return;
        }

        let (line, rest) = input.split_once('\n').unwrap_or((input, ""));
        *input = rest;

        if line.starts_with("dir") {
            tree.insert(
                line.trim_start_matches("dir ").to_owned(),
                Node::Directory(Directory::new()),
            );
        } else if line == "$ ls" || line == "$ cd /" {
        } else if line == "$ cd .." {
            return;
        } else if line.starts_with("$ cd") {
            let Some(Node::Directory(dir)) = tree.get_mut(line.trim_start_matches("$ cd ")) else {panic!()};
            input_gen(input, dir);
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            tree.insert(
                name.to_owned(),
                Node::File {
                    size: size.parse().unwrap(),
                },
            );
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(mut input: &str) -> u64 {
    let mut tree = Directory::new();
    input_gen(&mut input, &mut tree);
    tree.dir_iter()
        .map(|d| d.size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>()
}

#[aoc(day7, part2)]
pub fn solve_part2(mut input: &str) -> Option<u64> {
    let mut tree = Directory::new();
    input_gen(&mut input, &mut tree);
    let space_used = tree.size();
    let free_space = 70_000_000 - space_used;
    let space_to_free = 30_000_000 - free_space;
    tree.dir_iter()
        .map(|d| d.size())
        .filter(|&s| s >= space_to_free)
        .min()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let size = solve_part1(INPUT);
        assert_eq!(size, 95437);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let size = solve_part2(INPUT);
        assert_eq!(size, Some(24933642));
        Ok(())
    }
}
