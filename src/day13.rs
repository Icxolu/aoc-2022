use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketPart {
    Integer(u32),
    List(Vec<PacketPart>),
}

fn parse_packet_inner<I>(chars: &mut I, parts: &mut Vec<PacketPart>)
where
    I: Iterator<Item = char>,
{
    let mut num = String::new();

    loop {
        let Some(c) = chars.next() else {return;};
        match c {
            '[' => {
                let mut p = vec![];
                parse_packet_inner(chars, &mut p);
                parts.push(PacketPart::List(p));
            }
            ']' => {
                if !num.is_empty() {
                    parts.push(PacketPart::Integer(num.parse().unwrap()));
                }
                num.clear();
                return;
            }
            ',' => {
                if !num.is_empty() {
                    parts.push(PacketPart::Integer(num.parse().unwrap()));
                }
                num.clear();
            }

            '0'..='9' => num.push(c),
            _ => unreachable!(),
        }
    }
}

fn parse_packet(input: &str) -> Vec<PacketPart> {
    let mut v = vec![];
    parse_packet_inner(&mut input.chars(), &mut v);
    v
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Order {
    Correct,
    Reversed,
}

fn compare_packets(l: &PacketPart, r: &PacketPart) -> Option<Order> {
    match (l, r) {
        (PacketPart::Integer(il), PacketPart::Integer(ir)) => match il.cmp(ir) {
            std::cmp::Ordering::Less => Some(Order::Correct),
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => Some(Order::Reversed),
        },
        (PacketPart::Integer(il), r @ PacketPart::List(_)) => {
            compare_packets(&PacketPart::List(vec![PacketPart::Integer(*il)]), r)
        }
        (l @ PacketPart::List(_), PacketPart::Integer(ir)) => {
            compare_packets(l, &PacketPart::List(vec![PacketPart::Integer(*ir)]))
        }
        (PacketPart::List(l), PacketPart::List(r)) => {
            let mut l_iter = l.iter();
            let mut r_iter = r.iter();
            loop {
                let (l, r) = match (l_iter.next(), r_iter.next()) {
                    (None, None) => return None,
                    (None, Some(_)) => return Some(Order::Correct),
                    (Some(_), None) => return Some(Order::Reversed),
                    (Some(l), Some(r)) => (l, r),
                };

                match compare_packets(l, r) {
                    None => continue,
                    Some(o) => return Some(o),
                };
            }
        }
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter_map(|pair| {
            let (l, r) = pair.split_once('\n')?;
            let l = PacketPart::List(parse_packet(l));
            let r = PacketPart::List(parse_packet(r));
            Some(compare_packets(&l, &r))
        })
        .enumerate()
        .filter(|(_, c)| matches!(c, Some(Order::Correct)))
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> usize {
    let div_packet_1 = PacketPart::List(vec![PacketPart::List(vec![PacketPart::Integer(2)])]);
    let div_packet_2 = PacketPart::List(vec![PacketPart::List(vec![PacketPart::Integer(6)])]);

    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|packet| PacketPart::List(parse_packet(packet)))
        .collect();

    packets.push(div_packet_1.clone());
    packets.push(div_packet_2.clone());

    packets.sort_by(|l, r| match compare_packets(r, l) {
        Some(Order::Correct) => std::cmp::Ordering::Greater,
        Some(Order::Reversed) => std::cmp::Ordering::Less,
        None => std::cmp::Ordering::Equal,
    });
    let (ia, ib) = packets
        .into_iter()
        .enumerate()
        .fold((0, 0), |mut pos, (idx, packet)| {
            if packet == div_packet_1 {
                pos.0 = idx + 1;
            } else if packet == div_packet_2 {
                pos.1 = idx + 1;
            }
            pos
        });

    ia * ib
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let sum = solve_part1(INPUT);
        assert_eq!(sum, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let key = solve_part2(INPUT);
        assert_eq!(key, 140);
        Ok(())
    }
}
