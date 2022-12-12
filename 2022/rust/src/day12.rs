pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1:\n{}", part1(&input));
    println!("Part2:\n{}", part2(&input));
}

#[derive(Debug, Clone, Hash)]
struct Node {
    x: isize,
    y: isize,
    height: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}

fn char_to_height(c: char) -> usize {
    ((c as u8 - 'a' as u8) + 1) as usize
}

fn parse_input(input_lines: &[&str]) -> (Vec<Node>, Node, Node) {
    let mut start = Node {
        x: 0,
        y: 0,
        height: 0,
    };
    let mut end = Node {
        x: 0,
        y: 0,
        height: 0,
    };
    let nodes: Vec<Node> = input_lines
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| {
                    let height = match c {
                        'S' => {
                            let height = char_to_height('a');
                            start = Node {
                                x: x as isize,
                                y: y as isize,
                                height,
                            };
                            height
                        }
                        'E' => {
                            let height = char_to_height('z');
                            end = Node {
                                x: x as isize,
                                y: y as isize,
                                height,
                            };
                            height
                        }
                        height => char_to_height(height),
                    };

                    Node {
                        x: x as isize,
                        y: y as isize,
                        height,
                    }
                })
                .collect::<Vec<Node>>()
        })
        .flatten()
        .collect();

    (nodes, start, end)
}

fn part1(input: &(Vec<Node>, Node, Node)) -> usize {
    let (nodes, start, end) = input;
    let (path, _) = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |&node| {
            let mut neighbours = Vec::new();

            //up
            if let Some(next_node) = nodes.iter().find(|other_node| {
                node.x == other_node.x - 1
                    && node.y == other_node.y
                    && (other_node.height as isize - node.height as isize) <= 1
            }) {
                neighbours.push((next_node, 1));
            }

            //down
            if let Some(next_node) = nodes.iter().find(|other_node| {
                node.x == other_node.x + 1
                    && node.y == other_node.y
                    && (other_node.height as isize - node.height as isize) <= 1
            }) {
                neighbours.push((next_node, 1));
            }

            //left
            if let Some(next_node) = nodes.iter().find(|other_node| {
                node.x == other_node.x
                    && node.y == other_node.y - 1
                    && (other_node.height as isize - node.height as isize) <= 1
            }) {
                neighbours.push((next_node, 1));
            }

            //up
            if let Some(next_node) = nodes.iter().find(|other_node| {
                node.x == other_node.x
                    && node.y == other_node.y + 1
                    && (other_node.height as isize - node.height as isize) <= 1
            }) {
                neighbours.push((next_node, 1));
            }

            neighbours
        },
        |&node| node == end,
    )
    .unwrap();

    path.len() - 1
}

fn part2(input: &(Vec<Node>, Node, Node)) -> usize {
    let (nodes, _, end) = input;

    let starting_points = nodes.iter().filter(|node| node.height == 1);

    starting_points.map(|start| {
        let result = pathfinding::directed::dijkstra::dijkstra(
            &start,
            |&node| {
                let mut neighbours = Vec::new();

                //up
                if let Some(next_node) = nodes.iter().find(|other_node| {
                    node.x == other_node.x - 1
                        && node.y == other_node.y
                        && (other_node.height as isize - node.height as isize) <= 1
                }) {
                    neighbours.push((next_node, 1));
                }

                //down
                if let Some(next_node) = nodes.iter().find(|other_node| {
                    node.x == other_node.x + 1
                        && node.y == other_node.y
                        && (other_node.height as isize - node.height as isize) <= 1
                }) {
                    neighbours.push((next_node, 1));
                }

                //left
                if let Some(next_node) = nodes.iter().find(|other_node| {
                    node.x == other_node.x
                        && node.y == other_node.y - 1
                        && (other_node.height as isize - node.height as isize) <= 1
                }) {
                    neighbours.push((next_node, 1));
                }

                //up
                if let Some(next_node) = nodes.iter().find(|other_node| {
                    node.x == other_node.x
                        && node.y == other_node.y + 1
                        && (other_node.height as isize - node.height as isize) <= 1
                }) {
                    neighbours.push((next_node, 1));
                }

                neighbours
            },
            |&node| node == end,
        );

        if let Some((path, _)) = result {
            path.len() - 1
        } else {
            usize::MAX
        }
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] =
        &["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!(31, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input);

        assert_eq!(29, result);
    }
}
