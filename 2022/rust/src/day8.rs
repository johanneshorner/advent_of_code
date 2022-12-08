pub fn run(input_lines: &[&str]) {
    let input = parse_input(input_lines);

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn parse_input(input_lines: &[&str]) -> Vec<Vec<usize>> {
    input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn is_tree_visible(tree_map: &[Vec<usize>], pos_to_check: (usize, usize)) -> bool {
    let (tree_x, tree_y) = pos_to_check;
    let tree_height = tree_map[tree_y][tree_x];

    let row: &Vec<usize> = &tree_map[tree_y];

    if row[0..tree_x].iter().all(|height| height < &tree_height)
        || row[tree_x + 1..].iter().all(|height| height < &tree_height)
    {
        return true;
    }

    let column: Vec<usize> = tree_map.iter().map(|row| row[tree_x]).collect();

    if column[0..tree_y].iter().all(|height| height < &tree_height)
        || column[tree_y + 1..]
            .iter()
            .all(|height| height < &tree_height)
    {
        return true;
    }

    false
}

fn part1(tree_map: &[Vec<usize>]) -> usize {
    let mut hidden_trees_count: usize = 0;

    for y in 1..tree_map.len() - 1 {
        for x in 1..tree_map[0].len() - 1 {
            if is_tree_visible(tree_map, (x, y)) {
                hidden_trees_count += 1;
            }
        }
    }

    let edge_trees_count: usize = tree_map.len() * 2 + tree_map[0].len() * 2 - 4;

    hidden_trees_count + edge_trees_count
}

use take_until::TakeUntilExt;

fn calculate_scenic_score(tree_map: &[Vec<usize>], pos_to_check: (usize, usize)) -> usize {
    let mut scenic_score: usize = 1;

    let (tree_x, tree_y) = pos_to_check;
    let tree_height: usize = tree_map[tree_y][tree_x];

    let row: &Vec<usize> = &tree_map[tree_y];

    scenic_score *= row[tree_x + 1..row.len()]
        .iter()
        .take_until(|height| **height >= tree_height)
        .count();
    scenic_score *= row[0..tree_x]
        .iter()
        .rev()
        .take_until(|height| **height >= tree_height)
        .count();

    let column: Vec<usize> = tree_map.iter().map(|row| row[tree_x]).collect();

    scenic_score *= column[tree_y + 1..row.len()]
        .iter()
        .take_until(|height| **height >= tree_height)
        .count();

    scenic_score *= column[0..tree_y]
        .iter()
        .rev()
        .take_until(|height| **height >= tree_height)
        .count();

    scenic_score
}

fn part2(tree_map: &[Vec<usize>]) -> usize {
    let mut scenic_scores = Vec::new();

    for y in 0..tree_map.len() {
        for x in 0..tree_map[0].len() {
            let score = calculate_scenic_score(tree_map, (x, y));
            scenic_scores.push(score);
        }
    }

    *scenic_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_LINES: &'static [&'static str] = &["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn part1() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part1(&input);

        assert_eq!(21, result);
    }

    #[test]
    fn part2() {
        let input = parse_input(&INPUT_LINES);
        let result = super::part2(&input);

        assert_eq!(8, result);
    }
}
