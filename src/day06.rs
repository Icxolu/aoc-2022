use aoc_runner_derive::aoc;

fn solve(input: &[u8], n: usize) -> usize {
    input
        .windows(n)
        .take_while(|&chars| {
            let flags = chars
                .iter()
                .fold(0u32, |flags, char| flags | (1 << (char - b'a')));
            flags.count_ones() as usize != n
        })
        .count()
        + n
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    solve(input, 14)
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT_PART_1: [&[u8]; 5] = [
        b"mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        b"bvwbjplbgvbhsrlpgdmjqwftvncz",
        b"nppdvjthqldpwncqszvftbrmjlhg",
        b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    const RESULT_PART_1: [usize; 5] = [7, 5, 6, 10, 11];
    const RESULT_PART_2: [usize; 5] = [19, 23, 23, 29, 26];

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        for (input, result) in INPUT_PART_1.into_iter().zip(RESULT_PART_1) {
            let len = solve_part1(input);
            assert_eq!(len, result);
        }
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        for (input, result) in INPUT_PART_1.into_iter().zip(RESULT_PART_2) {
            let len = solve_part2(input);
            assert_eq!(len, result);
        }
        Ok(())
    }
}
