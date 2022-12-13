pub fn run(input_lines: &[&str]) {
    let input1 = parse_input1(input_lines);
    let input2 = parse_input2(input_lines);

    println!("Part1:\n{}", part1(&input1));
    println!("Part2:\n{}", part2(&input2));
}

#[derive(Debug, Clone, PartialEq)]
enum PData {
    List(Vec<PData>),
    Val(u32),
}

fn parse_packet(packet: &mut std::iter::Peekable<std::str::Chars>) -> PData {
    let mut packet_data: Vec<PData> = Vec::new();

    while let Some(c) = packet.next() {
        match c {
            ']' => break,
            '[' => packet_data.push(parse_packet(packet)),
            c if c.is_digit(10) => {
                if let Some(next_c) = packet.peek() {
                    if next_c.is_digit(10) {
                        packet_data.push(PData::Val(10));
                    } else {
                        packet_data.push(PData::Val(c.to_digit(10).unwrap()));
                    }
                } else {
                    packet_data.push(PData::Val(c.to_digit(10).unwrap()));
                    break;
                }
            }
            _ => {}
        }
    }

    PData::List(packet_data)
}

fn parse_input1(input_lines: &[&str]) -> Vec<Vec<PData>> {
    input_lines
        .split(|line| line.len() == 0)
        .map(|packets| {
            packets
                .iter()
                .map(|packet| {
                    parse_packet(
                        &mut packet[1..packet.len() - 1].chars().peekable(), /* remove outside brackets */
                    )
                })
                .collect::<Vec<PData>>()
        })
        .collect::<Vec<Vec<PData>>>()
}

fn parse_input2(input_lines: &[&str]) -> Vec<PData> {
    let mut input_lines = input_lines.to_vec();
    input_lines.push("[[6]]");
    input_lines.push("[[2]]");

    input_lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|packet| {
            parse_packet(
                &mut packet[1..packet.len() - 1].chars().peekable(), /* remove outside brackets */
            )
        })
        .collect::<Vec<PData>>()
}

fn data_in_order(left: &PData, right: &PData) -> Option<bool> {
    let to_list = |p_data: PData| match p_data {
        PData::List(_) => p_data,
        PData::Val(num) => PData::List(vec![PData::Val(num)]),
    };

    if let (PData::List(left), PData::List(right)) = (left, right) {
        for (left, right) in left.iter().zip(right.iter()) {
            let status = match (left, right) {
                (PData::Val(l), PData::Val(r)) => {
                    if l < r {
                        Some(true)
                    } else if l > r {
                        Some(false)
                    } else {
                        None
                    }
                }
                (l, r) => {
                    if let Some(status) = data_in_order(&to_list(l.clone()), &to_list(r.clone())) {
                        Some(status)
                    } else {
                        None
                    }
                }
            };

            if let Some(status) = status {
                return Some(status);
            }
        }

        return if left.len() < right.len() {
            Some(true)
        } else if left.len() > right.len() {
            Some(false)
        } else {
            None
        };
    }

    unreachable!();
}

fn part1(packet_pairs: &[Vec<PData>]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, packet_pair)| data_in_order(&packet_pair[0], &packet_pair[1]).unwrap())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(packets: &[PData]) -> usize {
    let mut packets = packets.to_vec();

    loop {
        let mut done = true;
        for i in 0..packets.len() - 1 {
            if !data_in_order(&packets[i], &packets[i + 1]).unwrap() {
                packets.swap(i, i + 1);
                done = false;
            }
        }

        if done {
            break;
        }
    }

    packets
        .iter()
        .enumerate()
        .filter(|&(_, packet)| {
            *packet == PData::List(vec![PData::List(vec![PData::Val(2)])])
                || *packet == PData::List(vec![PData::List(vec![PData::Val(6)])])
        })
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::{parse_input1, parse_input2};

    const INPUT_LINES: &'static [&'static str] = &[
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    #[test]
    fn part1() {
        let input = parse_input1(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!(13, result);
    }

    #[test]
    fn part2() {
        let input = parse_input2(&INPUT_LINES);
        let result = super::part2(&input);

        assert_eq!(140, result);
    }
}
