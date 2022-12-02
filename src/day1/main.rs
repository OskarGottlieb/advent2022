use std::include_str;
use std::path::Path;

fn calories_by_elf() -> Vec<isize> {
    let f: Vec<&str> = include_str!("input").split("\n").collect();
    let mut calories: Vec<isize> = vec![];
    let mut total_calories: isize = 0;
    for value in f.into_iter() {
        total_calories = match value {
            "" => {
                calories.push(total_calories);
                0
            }
            _ => total_calories + value.parse::<isize>().unwrap(),
        }
    }
    calories.sort();
    calories
}

fn part_1() {
    let calories = calories_by_elf();
    println!(
        "The most overworked elf is carrying {} calories.",
        calories.iter().max().unwrap()
    );
}
fn part_2() {
    let calories = calories_by_elf();
    println!(
        "The three most overworked elves are carrying total of {} calories.",
        calories.iter().rev().take(3).sum::<isize>()
    );
}
fn main() {
    part_1();
    part_2();
}
