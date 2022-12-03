use itertools::{Itertools, Tuples};
use std::include_str;

const LETTERS: &'static str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part_1() {
    let sum_of_priorities = include_str!("input")
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            first
                .chars()
                .filter(|f| second.contains(*f))
                .map(|x| LETTERS.find(x).unwrap())
                .unique()
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("The sum of priorities is: {}.", sum_of_priorities)
}
fn part_2() {
    let mut sum_of_priorities: usize = 0;
    for (first, second, third) in include_str!("input").lines().tuples() {
        sum_of_priorities += first
            .chars()
            .filter(|f| second.contains(*f) & third.contains(*f))
            .map(|x| LETTERS.find(x).unwrap())
            .unique()
            .sum::<usize>();
    }
    println!("The sum of priorities is: {}.", sum_of_priorities)
}
fn main() {
    part_1();
    part_2();
}
