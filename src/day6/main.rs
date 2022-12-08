use std::collections::HashSet;
use std::include_str;

fn signal_finder(window_size: usize) {
    let v: Vec<char> = include_str!("input").chars().collect();

    for x in 0..(v.len() - window_size) {
        let chunk: String = v[x..(x + window_size)]
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if chunk.len() == window_size {
            println!(
                "The first start-of-packet marker of size {window_size} is detected after character: {}",
                x + window_size
            );
            break;
        }
    }
}
fn part_1() {
    signal_finder(4);
}
fn part_2() {
    signal_finder(14);
}
fn main() {
    part_1();
    part_2();
}
