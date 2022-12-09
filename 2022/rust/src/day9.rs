pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: char,
    distance: i32,
}

fn parse_input(input_lines: &[&str]) -> Vec<Motion> {
    input_lines
        .iter()
        .map(|line| {
            let (direction, distance) = match line.split(' ').collect::<Vec<&str>>()[..] {
                [direction, distance] => (direction, distance),
                _ => unreachable!(),
            };
            let direction = direction.chars().next().unwrap();
            let distance = distance.parse::<i32>().unwrap();
            Motion {
                direction,
                distance,
            }
        })
        .collect::<Vec<Motion>>()
}

use std::collections::HashSet;

fn is_adjacent(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    let (x_head, y_head) = head_pos;
    let (x_tail, y_tail) = tail_pos;

    (x_head - x_tail).abs() <= 1 && (y_head - y_tail).abs() <= 1
}

fn part1(motions: &[Motion]) -> usize {
    let (mut x_head, mut y_head, mut x_tail, mut y_tail) = (0, 0, 0, 0);

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0));

    for Motion {
        direction,
        distance,
    } in motions
    {
        for _ in 0..*distance {
            match direction {
                'L' => x_head -= 1,
                'R' => x_head += 1,
                'U' => y_head -= 1,
                'D' => y_head += 1,
                _ => unreachable!(),
            }

            if !is_adjacent((x_head, y_head), (x_tail, y_tail)) {
                if x_head < x_tail {
                    x_tail -= 1;
                } else if x_head > x_tail {
                    x_tail += 1;
                }

                if y_head < y_tail {
                    y_tail -= 1;
                } else if y_head > y_tail {
                    y_tail += 1;
                }

                tail_positions.insert((x_tail, y_tail));
            }
        }
    }

    tail_positions.len()
}

fn part2(motions: &[Motion]) -> usize {
    let mut knots = [(0, 0); 10];

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert((0, 0));

    for Motion {
        direction,
        distance,
    } in motions
    {
        for _ in 0..*distance {
            match direction {
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                'U' => knots[0].1 -= 1,
                'D' => knots[0].1 += 1,
                _ => unreachable!(),
            }

            for i in 1..knots.len() {
                let (x_prev, y_prev) = knots[i-1];
                let (x_current, y_current) = &mut knots[i];

                if !is_adjacent((x_prev, y_prev), (*x_current, *y_current)) {
                    if x_prev < *x_current {
                        *x_current -= 1;
                    } else if x_prev > *x_current {
                        *x_current += 1;
                    }

                    if y_prev < *y_current {
                        *y_current -= 1;
                    } else if y_prev > *y_current {
                        *y_current += 1;
                    }

                    if i == 9 {
                        tail_positions.insert((*x_current, *y_current));
                    }
                }
            }
        }
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES1: &'static [&'static str] =
        &["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];

    const INPUT_LINES2: &'static [&'static str] =
        &["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES1);
        let result = super::part1(&input);

        assert_eq!(13, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES2);
        let result = super::part2(&input);

        assert_eq!(36, result);
    }
}
