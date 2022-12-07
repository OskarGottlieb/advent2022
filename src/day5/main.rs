use std::include_str;

fn parse_instruction(columns: &mut Vec<Vec<char>>, instruction: &str) {
    let k = instruction
        .split(" ")
        .filter(|x| (*x).parse::<usize>().is_ok())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (number, idx_from, idx_to) = (k[0], k[1] - 1, k[2] - 1);
    for _ in (0..number) {
        let last_value = columns[idx_from].pop().unwrap();
        columns[idx_to].push(last_value);
    }
}
fn parse_instruction_9001(columns: &mut Vec<Vec<char>>, instruction: &str) {
    let k = instruction
        .split(" ")
        .filter(|x| (*x).parse::<usize>().is_ok())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (number, idx_from, idx_to) = (k[0], k[1] - 1, k[2] - 1);
    let idx: usize = columns[idx_from].len() - (number);
    let mut new_col = columns[idx_from].split_off(idx);
    columns[idx_to].append(&mut new_col);
}
fn create_columns(columns_raw: &str) -> Vec<Vec<char>> {
    let indices = columns_raw.split("\n").last().unwrap();
    let vec_idx: Vec<usize> = indices
        .chars()
        .filter(|x| (*x).is_digit(10))
        .map(|x| indices.find(x).unwrap())
        .collect();
    let mut columns: Vec<Vec<char>> = vec![vec![]; vec_idx.len()];

    let _: Vec<_> = columns_raw
        .split("\n")
        .filter(|row| *row != indices)
        .map(|row| {
            for idx in 0..vec_idx.len() {
                if let Some(c) = row.chars().nth(vec_idx[idx]) {
                    if c != ' ' {
                        columns[idx].insert(0, c);
                    }
                }
            }
        })
        .collect();
    columns
}
fn part_1() {
    let (columns_raw, instructions) = include_str!("input").split_once("\n\n").unwrap();
    let mut columns = create_columns(columns_raw);
    for instruction in instructions.split("\n") {
        if instruction != "" {
            parse_instruction(&mut columns, instruction);
        }
    }
    let last_letters: String = columns.into_iter().map(|mut x| x.pop().unwrap()).collect();
    println!("The letters with CreateMover9000 are: {:?}", last_letters);
}
fn part_2() {
    let (columns_raw, instructions) = include_str!("input").split_once("\n\n").unwrap();
    let mut columns = create_columns(columns_raw);
    for instruction in instructions.split("\n") {
        if instruction != "" {
            parse_instruction_9001(&mut columns, instruction);
        }
    }
    let last_letters: String = columns.into_iter().map(|mut x| x.pop().unwrap()).collect();
    println!("The letters with CreateMover9001 are: {:?}", last_letters);
}
fn main() {
    part_1();
    part_2();
}
