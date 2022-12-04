use std::include_str;

fn part_1() {
    let lines: Vec<(usize, usize, usize, usize)> = include_str!("input")
        .lines()
        .map(|x| {
            let (first, second) = x.split_once(",").unwrap();
            let (a, b) = first.split_once("-").unwrap();
            let (c, d) = second.split_once("-").unwrap();
            (
                a.parse::<usize>().unwrap(),
                b.parse::<usize>().unwrap(),
                c.parse::<usize>().unwrap(),
                d.parse::<usize>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| ((a <= c) & (b >= d)) | ((a >= c) & (b <= d)))
        .collect();
    println!(
        "Total number of completely overlapping assignments is: {:?}",
        lines.len()
    )
}
fn part_2() {
    let lines: Vec<(usize, usize, usize, usize)> = include_str!("input")
        .lines()
        .map(|x| {
            let (first, second) = x.split_once(",").unwrap();
            let (a, b) = first.split_once("-").unwrap();
            let (c, d) = second.split_once("-").unwrap();
            (
                a.parse::<usize>().unwrap(),
                b.parse::<usize>().unwrap(),
                c.parse::<usize>().unwrap(),
                d.parse::<usize>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| {
            ((a >= c) & (a <= d))
                | ((b >= c) & (b <= d))
                | ((a <= c) & (b >= d))
                | ((a >= c) & (b <= d))
        })
        .collect();
    println!(
        "Total number of partially overlapping assignments is: {:?}",
        lines.len()
    )
}
fn main() {
    part_1();
    part_2();
}
