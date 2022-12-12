use std::include_str;
use std::iter::zip;

const COLUMN_WIDTH: usize = 99;

fn check_is_visible(trees: &Vec<isize>) -> Vec<bool> {
    let reversed_tree: Vec<isize> = trees.to_owned().into_iter().rev().collect();
    let z = zip(_is_visible(trees, false), _is_visible(&reversed_tree, true));
    z.map(|(x, y)| x || y).collect()
}
fn _is_visible(trees: &Vec<isize>, is_reversed: bool) -> Vec<bool> {
    let mut v: Vec<_> = Vec::with_capacity(trees.len());
    let mut max_value: isize = -1;
    for tree in trees {
        v.push(tree > &max_value);
        max_value = max_value.max(*tree)
    }
    if is_reversed {
        v.into_iter().rev().collect()
    } else {
        v
    }
}
fn get_rows() -> Vec<Vec<isize>> {
    include_str!("input")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as isize)
                .collect::<Vec<isize>>()
        })
        .collect()
}
fn get_columns() -> Vec<Vec<isize>> {
    let mut columns: Vec<Vec<isize>> = vec![vec![]; COLUMN_WIDTH];
    let numbers: Vec<isize> = include_str!("input")
        .replace("\n", "")
        .chars()
        .map(|x| x.to_digit(10).unwrap() as isize)
        .collect();
    for (idx, number) in numbers.into_iter().enumerate() {
        columns[idx % (COLUMN_WIDTH)].push(number);
    }
    columns
}
fn get_visible_trees(vectors: Vec<Vec<isize>>, is_row: bool) -> Vec<bool> {
    let trees: Vec<Vec<_>> = vectors.into_iter().map(|x| check_is_visible(&x)).collect();
    if is_row {
        trees.into_iter().flatten().collect()
    } else {
        let mut is_visible_cols = vec![false; include_str!("input").replace("\n", "").len()];
        for (idx, x) in trees.into_iter().flatten().enumerate() {
            is_visible_cols[(idx % COLUMN_WIDTH) * COLUMN_WIDTH + idx / COLUMN_WIDTH] = x;
        }
        is_visible_cols
    }
}
fn get_scenic_score(trees: &Vec<usize>, position: usize) -> usize {
    let mut scores = [0; 4];
    let mut current_score = 0;
    let mut current_position = position;
    let current_value = trees[position];
    // First Direction UP

    loop {
        if (current_position as isize - COLUMN_WIDTH as isize) < 0 {
            break;
        }
        let new_position = current_position - COLUMN_WIDTH;
        if (trees[new_position] >= current_value) {
            current_score += 1;
            break;
        }
        current_score += 1;
        current_position = new_position;
    }

    scores[0] = current_score;
    current_score = 0;
    current_position = position;
    loop {
        let new_position = (current_position + 1);
        if (new_position % COLUMN_WIDTH) == 0 {
            break;
        }
        if (trees[new_position] >= current_value) {
            current_score += 1;
            break;
        }
        current_score += 1;
        current_position = new_position;
    }
    scores[1] = current_score;
    current_score = 0;
    current_position = position;

    loop {
        let new_position = (current_position + COLUMN_WIDTH);
        if new_position >= trees.len() {
            break;
        }
        if (trees[new_position] >= current_value) {
            current_score += 1;
            break;
        }
        current_score += 1;
        current_position = new_position;
    }

    scores[2] = current_score;
    current_score = 0;
    current_position = position;

    loop {
        if (current_position % COLUMN_WIDTH) == 0 {
            break;
        }
        let new_position = (current_position - 1);

        if (trees[new_position] >= current_value) {
            current_score += 1;
            break;
        }
        current_score += 1;
        current_position = new_position;
    }

    scores[3] = current_score;
    scores.iter().product()
}
fn part_1() {
    let visible_rows = get_visible_trees(get_rows(), true);
    let visible_columns = get_visible_trees(get_columns(), false);

    let z: isize = zip(visible_rows, visible_columns)
        .map(|(x, y)| if x | y { 1 } else { 0 })
        .sum();
    println!("The total number of visible trees is {}.", z);
}
fn part_2() {
    let trees: Vec<_> = include_str!("input")
        .replace("\n", "")
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut max_scenic_score = 0;
    for x in 0..trees.len() {
        max_scenic_score = max_scenic_score.max(get_scenic_score(&trees, x));
    }

    println!("The max scenicscore is: {}.", max_scenic_score);
}
fn main() {
    part_1();
    part_2();
}
