use num::integer::Integer;
use regex::Regex;

fn main() {
    let input = parse_input(include_str!("../../../input/day13.txt"));
    eprintln!("{}", part1(&input));
}

#[derive(Debug, Clone)]
struct Game {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

type Input = Vec<Game>;

fn parse_input(input: &str) -> Input {
    let buttons = Regex::new(r#".\+(\d+)"#).unwrap();
    let prize_re = Regex::new(r#".=(\d+)"#).unwrap();
    input
        .trim()
        .split("\n\n")
        .map(|game| {
            if let [a, b, prize] = game.split('\n').collect::<Vec<&str>>().as_slice() {
                Game {
                    a: {
                        let xy = buttons
                            .captures_iter(a)
                            .map(|caps| {
                                let (_, [n]) = caps.extract();
                                n.parse::<usize>().unwrap()
                            })
                            .collect::<Vec<usize>>();
                        (xy[0], xy[1])
                    },
                    b: {
                        let xy = buttons
                            .captures_iter(b)
                            .map(|caps| {
                                let (_, [n]) = caps.extract();
                                n.parse::<usize>().unwrap()
                            })
                            .collect::<Vec<usize>>();
                        (xy[0], xy[1])
                    },
                    prize: {
                        let xy = prize_re
                            .captures_iter(prize)
                            .map(|caps| {
                                let (_, [n]) = caps.extract();
                                n.parse::<usize>().unwrap()
                            })
                            .collect::<Vec<usize>>();
                        (xy[0], xy[1])
                    },
                }
            } else {
                unreachable!("no");
            }
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(i, game)| {
            eprintln!("game: {i}");
            let Game { a, b, prize } = game;
            let ((ax, ay), (bx, by), (px, py)) = (a, b, prize);
            eprintln!("a-p {} {}", ax.gcd(px), ay.gcd(py));
            eprintln!("b-p {} {}", bx.gcd(px), by.gcd(py));
            eprintln!("ab-p {} {}", (ax + bx).gcd(px), (ay + by).gcd(py));
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    static INPUT: &'static str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
        "#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&parse_input(INPUT)), 480);
    }
}
